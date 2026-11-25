//! Native actors start together and cancel their peers on failure or unwind.
use super::{
    Actor, ActorStats, Config, HelperStats, Item, LeafMaintenance, LeafMutator, LeafRuntime,
    MaintenanceStep, Progress, failure, protocol,
};
use crate::{Error, Result};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
struct Shared {
    started: AtomicBool,
    failed: AtomicBool,
    done: AtomicUsize,
    objects: AtomicUsize,
    roots: AtomicUsize,
}
impl Shared {
    fn check(&self) -> Result<()> {
        if self.failed.load(Ordering::Acquire) {
            return Err(failure("another contention participant failed"));
        }
        Ok(())
    }
    fn wait(&self) -> Result<()> {
        while !self.started.load(Ordering::Acquire) {
            self.check()?;
            std::thread::yield_now();
        }
        self.check()
    }
}

struct Exit<'a> {
    shared: &'a Shared,
    actor: bool,
    completed: bool,
}
impl Drop for Exit<'_> {
    fn drop(&mut self) {
        if !self.completed {
            self.shared.failed.store(true, Ordering::Release);
        }
        if self.actor {
            self.shared.done.fetch_add(1, Ordering::Release);
        }
    }
}
struct Start<'a>(&'a Shared);
impl Drop for Start<'_> {
    fn drop(&mut self) {
        // If thread creation unwinds, already-created threads must not remain
        // waiting for a start that the parent can no longer issue.
        if !self.0.started.swap(true, Ordering::Release) {
            self.0.failed.store(true, Ordering::Release);
        }
    }
}

fn actor(
    id: usize,
    mut worker: LeafMutator<'_>,
    runtime: &LeafRuntime<'_>,
    drops: &Arc<AtomicUsize>,
    config: Config,
    shared: &Shared,
) -> Result<ActorStats> {
    let mut exit = Exit {
        shared,
        actor: true,
        completed: false,
    };
    let mut actor = Actor::new(id, config.retained);
    shared.wait()?;
    for sequence in 0..config.operations {
        actor.evict();
        let mut request = worker
            .begin_any(Item::new(id, sequence, drops))
            .map_err(protocol)?;
        let mut received = None;
        for polls in 1..=config.request_limit() {
            shared.check()?;
            actor.stats.polls += 1;
            if let Some(root) = request.poll().map_err(protocol)? {
                received = Some((root, polls));
                break;
            }
            if polls % 32 == 0 {
                std::thread::yield_now();
            }
        }
        let (root, polls) = received.ok_or_else(|| {
            failure(format!(
                "native allocation stalled: actor {id} sequence {sequence}"
            ))
        })?;
        actor.accept(runtime, root, polls)?;
    }
    drop(actor.held);
    exit.completed = true;
    Ok(actor.stats)
}

fn helper(
    id: usize,
    mut helper: LeafMaintenance<'_>,
    drops: &AtomicUsize,
    config: Config,
    shared: &Shared,
) -> Result<HelperStats> {
    let mut exit = Exit {
        shared,
        actor: false,
        completed: false,
    };
    let mut stats = HelperStats::default();
    shared.wait()?;
    for polls in 0..config.helper_limit() {
        shared.check()?;
        match stats.poll(&mut helper)? {
            MaintenanceStep::ReleasedObjectSlot => {
                shared.objects.fetch_add(1, Ordering::Relaxed);
            }
            MaintenanceStep::ReleasedRootSlot => {
                shared.roots.fetch_add(1, Ordering::Relaxed);
            }
            _ => {}
        }
        if shared.done.load(Ordering::Acquire) == config.actors
            && shared.objects.load(Ordering::Relaxed) == config.objects()
            && shared.roots.load(Ordering::Relaxed) == config.objects()
            && drops.load(Ordering::Relaxed) == config.objects()
        {
            exit.completed = true;
            return Ok(stats);
        }
        if polls % 32 == 0 {
            std::thread::yield_now();
        }
    }
    Err(failure(format!(
        "native reclamation stalled: helper {id} objects={} roots={} drops={}",
        shared.objects.load(Ordering::Relaxed),
        shared.roots.load(Ordering::Relaxed),
        drops.load(Ordering::Relaxed)
    )))
}

pub(super) fn run(
    runtime: &LeafRuntime<'_>,
    mutators: Vec<LeafMutator<'_>>,
    helpers: Vec<LeafMaintenance<'_>>,
    drops: &Arc<AtomicUsize>,
    config: Config,
) -> Result<Progress> {
    let shared = Shared::default();
    let (actors, helpers) = std::thread::scope(|scope| {
        let _start = Start(&shared);
        let actors = mutators
            .into_iter()
            .enumerate()
            .map(|(id, worker)| {
                let shared = &shared;
                scope.spawn(move || actor(id, worker, runtime, drops, config, shared))
            })
            .collect::<Vec<_>>();
        let helpers = helpers
            .into_iter()
            .enumerate()
            .map(|(id, worker)| {
                let shared = &shared;
                scope.spawn(move || helper(id, worker, drops, config, shared))
            })
            .collect::<Vec<_>>();
        shared.started.store(true, Ordering::Release);
        // Join every thread, even after a failure. Each exit guard cancels peers;
        // no panic or partial progress can become a successful report.
        let actors = actors
            .into_iter()
            .map(std::thread::ScopedJoinHandle::join)
            .collect::<Vec<_>>();
        let helpers = helpers
            .into_iter()
            .map(std::thread::ScopedJoinHandle::join)
            .collect::<Vec<_>>();
        (actors, helpers)
    });
    let mut errors = Vec::new();
    let actors = joined(actors, &mut errors);
    let helpers = joined(helpers, &mut errors);
    // A peer may observe cancellation before the failing participant is joined.
    // Preserve the actual cause regardless of participant/join order.
    if let Some(error) = errors.into_iter().reduce(|first, next| {
        if matches!(&first, Error::RootError(message) if message == "another contention participant failed") {
            next
        } else {
            first
        }
    }) {
        return Err(error);
    }
    shared.check()?;
    Ok((actors, helpers))
}

fn joined<T>(results: Vec<std::thread::Result<Result<T>>>, errors: &mut Vec<Error>) -> Vec<T> {
    results
        .into_iter()
        .filter_map(|result| {
            match result.unwrap_or_else(|_| Err(failure("contention participant panicked"))) {
                Ok(value) => Some(value),
                Err(error) => {
                    errors.push(error);
                    None
                }
            }
        })
        .collect()
}
