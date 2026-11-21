//! Only canonical guarded publication can move an opaque record into tracing.
use super::{
    BoundedCatalog, Command, Commands, Payload, RecordKey, RequestError, RequestState,
    diagnose_failure, lookup,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GuardedCommand {
    Abandon(RecordKey),
    Publish(RecordKey),
    Promote(RecordKey),
    Advance {
        domain: usize,
    },
    Start {
        domain: usize,
        mode: crate::canonical_catalog::DomainMode,
    },
    Finish {
        domain: usize,
    },
    /// Finish only the epoch whose sweep the caller completed. Retries retain
    /// this epoch even if another helper has finished and restarted the domain.
    FinishEpoch {
        domain: usize,
        epoch: u64,
    },
    Shade {
        key: RecordKey,
        domain: usize,
        retention: crate::canonical_catalog::TraceRetention,
    },
    Acknowledge {
        key: RecordKey,
        domain: usize,
        ticket: u64,
    },
    Retire {
        key: RecordKey,
        domain: usize,
    },
}
impl<R: Payload> Commands<R> for GuardedCommand {
    fn numeric(self) -> Command {
        match self {
            Self::Abandon(key) => Command::Abandon(key),
            Self::Publish(key) => Command::Publish(key),
            Self::Promote(key) => Command::Promote(key),
            Self::Advance { domain } => Command::Advance { domain },
            Self::Start { domain, mode } => Command::Start { domain, mode },
            Self::Finish { domain } | Self::FinishEpoch { domain, .. } => {
                Command::Finish { domain }
            }
            Self::Shade {
                key,
                domain,
                retention,
            } => Command::Shade {
                key,
                domain,
                retention,
            },
            Self::Acknowledge {
                key,
                domain,
                ticket,
            } => Command::TraceAcknowledge {
                key,
                domain,
                ticket,
            },
            Self::Retire { key, domain } => Command::TraceRetire { key, domain },
        }
    }
    fn start(
        self,
        catalog: &BoundedCatalog<R>,
        context: &R::Context,
    ) -> Result<RequestState<R, Self>, RequestError> {
        let command = <Self as Commands<R>>::numeric(self);
        let snapshot = catalog
            .raw
            .snapshot_in(context)
            .ok_or(RequestError::Contended)?;
        let phase = snapshot.phase();
        if phase.revision == u64::MAX {
            return Err(RequestError::Exhausted);
        }
        let cell = command
            .key()
            .map(|record| lookup(catalog, record))
            .transpose()?;
        let preparation = match self {
            Self::Abandon(_) => catalog
                .raw
                .begin_abandon(snapshot, cell.expect("command retains its required object")),
            Self::Publish(_) => catalog
                .raw
                .begin_publish(snapshot, cell.expect("command retains its required object")),
            Self::Promote(_) => catalog
                .raw
                .begin_promote(snapshot, cell.expect("command retains its required object")),
            Self::Advance { domain } => catalog.raw.begin_advance(snapshot, domain),
            Self::Start { domain, mode } => catalog.raw.begin_start(snapshot, domain, mode),
            Self::Finish { domain } => catalog.raw.begin_finish(snapshot, domain),
            Self::FinishEpoch { domain, epoch } => {
                catalog.raw.begin_finish_epoch(snapshot, domain, epoch)
            }
            Self::Shade {
                domain, retention, ..
            } => catalog.raw.begin_shade(
                snapshot,
                cell.expect("command retains its required object"),
                domain,
                retention,
            ),
            Self::Acknowledge { domain, ticket, .. } => catalog.raw.begin_trace_acknowledge(
                snapshot,
                cell.expect("command retains its required object"),
                domain,
                ticket,
            ),
            Self::Retire { domain, .. } => catalog.raw.begin_trace_retire(
                snapshot,
                cell.expect("command retains its required object"),
                domain,
            ),
        };
        if let Some(preparation) = preparation {
            return Ok(RequestState::Preparing(self, preparation));
        }
        if let Self::FinishEpoch { domain, epoch } = self {
            let current = phase
                .domains
                .get(domain)
                .ok_or(RequestError::InvalidDomain)?;
            if current.epoch != epoch {
                return Err(RequestError::Conflict);
            }
        }
        diagnose_failure(catalog, command, phase)
    }
}
