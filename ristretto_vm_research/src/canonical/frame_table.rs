//! Bounded invocation tokens. Only frame snapshots own canonical roots.
use super::{Error, Payload, Root};
use std::cell::{Cell, RefCell};
use std::num::NonZeroU64;
use std::rc::{Rc, Weak};
use std::task::Poll;

#[path = "token_ranges.rs"]
mod token_ranges;
use token_ranges::{LeaseError, TokenRange};

const LEASE_SIZE: NonZeroU64 = NonZeroU64::MIN.saturating_add(4095);

/// Test-only mechanism counts; never included in an ordinary runtime binary.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct PublicationCensus {
    pub calls: usize,
    pub unchanged: usize,
    pub nonnull_inputs: usize,
    pub rebound: usize,
    pub owner_probes: usize,
    pub snapshot_cells_checked: usize,
}

#[derive(Debug)]
struct Entry {
    token: i64,
    root: Weak<Root<Payload>>,
}

#[derive(Debug)]
struct FrameRegistry {
    active: Cell<usize>,
    limit: usize,
}

/// Root ownership drops directly with the frame, without consulting the table.
#[derive(Debug)]
pub(super) struct Snapshot {
    registry: Rc<FrameRegistry>,
    limit: usize,
    roots: Vec<(i64, Rc<Root<Payload>>)>,
    replacement: Vec<(i64, Rc<Root<Payload>>)>,
}

impl Drop for Snapshot {
    fn drop(&mut self) {
        self.registry.active.set(self.registry.active.get() - 1);
    }
}

/// Historical entries hold only weak Rust aliases. Slots may be reused; tokens never are.
#[derive(Debug)]
pub(super) struct Table {
    tokens: Option<TokenRange>,
    lease_size: NonZeroU64,
    entries: Vec<Option<Entry>>,
    frames: Rc<FrameRegistry>,
    roots_per_frame: usize,
    #[cfg(test)]
    leases_installed: usize,
    #[cfg(test)]
    pub(super) publication_census: Cell<PublicationCensus>,
}

impl Table {
    pub(super) fn new(
        objects: usize,
        frames: usize,
        roots_per_frame: usize,
    ) -> Result<Self, Error> {
        if objects == 0 || frames == 0 || roots_per_frame == 0 {
            return Err(Error::Invalid(
                "zero canonical frame-table capacity".to_owned(),
            ));
        }
        let mut entries = Vec::new();
        entries
            .try_reserve_exact(objects)
            .map_err(|_| Error::Limit("frame-table allocation"))?;
        entries.resize_with(objects, || None);
        Ok(Self {
            tokens: None,
            lease_size: LEASE_SIZE,
            entries,
            frames: Rc::new(FrameRegistry {
                active: Cell::new(0),
                limit: frames,
            }),
            roots_per_frame,
            #[cfg(test)]
            leases_installed: 0,
            #[cfg(test)]
            publication_census: Cell::new(PublicationCensus::default()),
        })
    }

    pub(super) fn enter(&self) -> Result<Snapshot, Error> {
        let active = self.frames.active.get();
        if active >= self.frames.limit {
            return Err(Error::Limit("frame-table frames"));
        }
        let mut roots = Vec::new();
        let mut replacement = Vec::new();
        roots
            .try_reserve_exact(self.roots_per_frame)
            .map_err(|_| Error::Limit("frame roots allocation"))?;
        replacement
            .try_reserve_exact(self.roots_per_frame)
            .map_err(|_| Error::Limit("frame roots allocation"))?;
        self.frames.active.set(active + 1);
        Ok(Snapshot {
            registry: self.frames.clone(),
            limit: self.roots_per_frame,
            roots,
            replacement,
        })
    }

    fn check_frame(&self, frame: &Snapshot) -> Result<(), Error> {
        if !Rc::ptr_eq(&frame.registry, &self.frames) {
            return Err(Error::Boundary("foreign canonical frame"));
        }
        Ok(())
    }

    fn owner(&self, token: i64) -> Result<Rc<Root<Payload>>, Error> {
        if token <= 0 {
            return Err(Error::Boundary("nonpositive reference token"));
        }
        self.entries
            .iter()
            .flatten()
            .find(|entry| {
                #[cfg(test)]
                {
                    let mut census = self.publication_census.get();
                    census.owner_probes += 1;
                    self.publication_census.set(census);
                }
                entry.token == token
            })
            .and_then(|entry| entry.root.upgrade())
            .ok_or_else(|| Error::Boundary("stale or foreign reference token"))
    }

    pub(super) fn resolve(&self, token: i64) -> Result<Option<Root<Payload>>, Error> {
        if token == 0 {
            return Ok(None);
        }
        Ok(Some((*self.owner(token)?).clone()))
    }

    /// Transfer a root before exposing its token. `Pending` makes no ownership
    /// change: the host must refill outside every table/frame borrow and native callback.
    fn intern(&mut self, frame: &mut Snapshot, root: &Root<Payload>) -> Result<Poll<i64>, Error> {
        self.check_frame(frame)?;
        for entry in self.entries.iter().flatten() {
            if let Some(owner) = entry.root.upgrade()
                && owner.same_object(root)
            {
                if !frame.roots.iter().any(|(token, _)| *token == entry.token) {
                    if frame.roots.len() >= frame.limit {
                        return Err(Error::Limit("frame roots"));
                    }
                    frame.roots.push((entry.token, owner));
                }
                return Ok(Poll::Ready(entry.token));
            }
        }
        if frame.roots.len() >= frame.limit {
            return Err(Error::Limit("frame roots"));
        }
        let entry = self
            .entries
            .iter_mut()
            .find(|entry| {
                entry
                    .as_ref()
                    .is_none_or(|entry| entry.root.strong_count() == 0)
            })
            .ok_or(Error::Limit("frame-table objects"))?;
        // Deduplication and quota checks precede issuance. Missing lease does not
        // change the entry or frame. Leased-range membership never authorizes a token.
        let Some(token) = self.tokens.as_mut().and_then(TokenRange::issue) else {
            return Ok(Poll::Pending);
        };
        let owner = Rc::new(root.clone());
        *entry = Some(Entry {
            token,
            root: Rc::downgrade(&owner),
        });
        frame.roots.push((token, owner));
        Ok(Poll::Ready(token))
    }

    fn install_lease(&mut self, range: TokenRange) -> Result<(), Error> {
        if range.remaining() == 0 || self.tokens.as_ref().is_some_and(|old| old.remaining() != 0) {
            return Err(Error::Boundary("invalid token lease replacement"));
        }
        self.tokens = Some(range);
        #[cfg(test)]
        {
            self.leases_installed += 1;
        }
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn with_lease_size(
        objects: usize,
        frames: usize,
        roots_per_frame: usize,
        lease_size: NonZeroU64,
    ) -> Result<Self, Error> {
        let mut table = Self::new(objects, frames, roots_per_frame)?;
        table.lease_size = lease_size;
        Ok(table)
    }

    #[cfg(test)]
    pub(super) fn leases_installed(&self) -> usize {
        self.leases_installed
    }

    /// Validate/retain the entire replacement before releasing any previous owner.
    /// Failure leaves the previous snapshot intact. This path never allocates.
    pub(super) fn publish(&self, frame: &mut Snapshot, tokens: &[i64]) -> Result<(), Error> {
        self.check_frame(frame)?;
        if tokens.len() > frame.limit {
            return Err(Error::Limit("published frame roots"));
        }
        // These existing owners pin the exact entries: intern never replaces an
        // entry with a live strong owner, and issued tokens are never reused.
        // Equality includes order and duplicate multiplicity, ignoring only null.
        // The provenance and raw-cell quota checks above still precede this path.
        let unchanged = frame
            .roots
            .iter()
            .map(|(token, _)| {
                #[cfg(test)]
                {
                    let mut census = self.publication_census.get();
                    census.snapshot_cells_checked += 1;
                    self.publication_census.set(census);
                }
                *token
            })
            .eq(tokens.iter().copied().filter(|token| *token != 0));
        #[cfg(test)]
        {
            let mut census = self.publication_census.get();
            census.calls += 1;
            census.unchanged += usize::from(unchanged);
            census.nonnull_inputs += tokens.iter().filter(|token| **token != 0).count();
            self.publication_census.set(census);
        }
        if unchanged {
            return Ok(());
        }
        frame.replacement.clear();
        for &token in tokens {
            if token == 0 {
                continue;
            }
            #[cfg(test)]
            {
                let mut census = self.publication_census.get();
                census.rebound += 1;
                self.publication_census.set(census);
            }
            let owner = match self.owner(token) {
                Ok(owner) => owner,
                Err(error) => {
                    frame.replacement.clear();
                    return Err(error);
                }
            };
            frame.replacement.push((token, owner));
        }
        std::mem::swap(&mut frame.roots, &mut frame.replacement);
        frame.replacement.clear();
        Ok(())
    }

    pub(super) fn publication_limit(&self) -> usize {
        self.roots_per_frame
    }

    #[cfg(test)]
    pub(super) fn live_objects(&self) -> usize {
        self.entries
            .iter()
            .flatten()
            .filter(|entry| entry.root.strong_count() != 0)
            .count()
    }
}

/// Host-side ownership handoff. The incoming owning root stays borrowed until
/// the frame has retained it, including across contention yields. This function
/// is never invoked from generated code or a publication callback.
pub(super) async fn intern_reference(
    table: &RefCell<Table>,
    frame: &RefCell<Snapshot>,
    root: &Root<Payload>,
    attempts: usize,
) -> Result<i64, Error> {
    intern_with(table, frame, root, attempts, TokenRange::try_acquire).await
}

async fn intern_with(
    table: &RefCell<Table>,
    frame: &RefCell<Snapshot>,
    root: &Root<Payload>,
    attempts: usize,
    mut acquire: impl FnMut(NonZeroU64) -> Poll<Result<TokenRange, LeaseError>>,
) -> Result<i64, Error> {
    let mut used = 0;
    loop {
        let lease_size = {
            let mut table = table
                .try_borrow_mut()
                .map_err(|_| Error::Boundary("reentrant canonical token table"))?;
            let mut frame = frame
                .try_borrow_mut()
                .map_err(|_| Error::Boundary("reentrant canonical token frame"))?;
            if let Poll::Ready(token) = table.intern(&mut frame, root)? {
                return Ok(token);
            }
            table.lease_size
        };
        // No RefCell guard exists during reservation or the following await.
        if used == attempts {
            return Err(Error::Limit("token lease attempts"));
        }
        used += 1;
        match acquire(lease_size) {
            Poll::Pending => tokio::task::yield_now().await,
            Poll::Ready(Ok(range)) => table
                .try_borrow_mut()
                .map_err(|_| Error::Boundary("reentrant canonical token table"))?
                .install_lease(range)?,
            Poll::Ready(Err(LeaseError::Exhausted)) => {
                return Err(Error::Limit("token identity"));
            }
            #[cfg(not(target_has_atomic = "64"))]
            Poll::Ready(Err(LeaseError::UnsupportedAtomic64)) => {
                return Err(Error::Unsupported("native 64-bit token atomics".to_owned()));
            }
        }
    }
}

#[cfg(test)]
#[path = "frame_table_tests.rs"]
mod tests;
