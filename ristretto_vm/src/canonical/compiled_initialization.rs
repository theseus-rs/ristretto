//! Owning actions parked behind registry-authorized native initializer frames.
use super::{CompiledExecution, Error, Frame, Method, Result, Value, result_bits};
use crate::canonical::statics::{PreparedInitialization, StaticField};
use std::sync::Arc;

pub(super) enum Action {
    Call {
        class: usize,
        method: Arc<Method>,
        arguments: Vec<Value>,
    },
    New {
        class: usize,
    },
    GetStatic {
        class: usize,
        field: StaticField,
    },
    PutStatic {
        caller: usize,
        method: Arc<Method>,
        class: usize,
        field: StaticField,
        value: Value,
    },
}
impl Action {
    fn initializing_class(&self) -> Option<usize> {
        match self {
            Self::Call { class, method, .. } => method.is_static().then_some(*class),
            Self::New { class } | Self::GetStatic { class, .. } | Self::PutStatic { class, .. } => {
                Some(*class)
            }
        }
    }
}

pub(super) enum Completion {
    Call,
    Initialization {
        prepared: PreparedInitialization,
        action: Action,
    },
}

impl CompiledExecution<'_, '_, '_, '_> {
    pub(super) async fn action(&mut self, frames: &mut Vec<Frame>, action: Action) -> Result<()> {
        if let Some(class) = action.initializing_class()
            && let Some(prepared) = self.execution.prepare_initialization(class).await?
        {
            if let Some(initializer) = prepared.initializer() {
                // The requested callee is not entered yet. This initializer occupies
                // its depth; nested Java calls consume additional bounded frames.
                let mut frame = self
                    .frame(prepared.class(), initializer, Vec::new(), frames.len())
                    .await?;
                frame.completion = Completion::Initialization { prepared, action };
                frames.push(frame);
                return Ok(());
            }
            self.execution.finish_initialization(prepared, Ok(None))?;
        }
        let value = match action {
            Action::Call {
                class,
                method,
                arguments,
            } => {
                if frames.is_empty() {
                    self.execution.report.entry_initialization =
                        super::super::InitializationWork::from_report(&self.execution.report);
                }
                let frame = self.frame(class, method, arguments, frames.len()).await?;
                frames.push(frame);
                return Ok(());
            }
            Action::New { class } => Some(Value::Reference(Some(
                self.execution.allocate_object(class).await?,
            ))),
            Action::GetStatic { class, field } => {
                Some(self.execution.read_prepared_static(class, &field).await?)
            }
            Action::PutStatic {
                caller,
                method,
                class,
                field,
                value,
            } => {
                self.execution
                    .write_prepared_static(caller, &method, class, &field, value)
                    .await?;
                None
            }
        };
        let parent = frames
            .last_mut()
            .ok_or(Error::Boundary("missing initialized request frame"))?;
        let bits = result_bits(
            &parent.context,
            value.as_ref(),
            self.execution.limits.request_polls,
        )
        .await?;
        parent.activation.complete_call(bits)?;
        tokio::task::yield_now().await;
        Ok(())
    }
}
