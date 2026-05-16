//! # Verification Diagnostics
//!
//! This module provides detailed diagnostic information for verification failures.
//! It includes context about the verification state at the point of failure,
//! making it easier to debug bytecode issues.
//!
//! # Features
//!
//! - Detailed error context with class, method, PC, and opcode
//! - Pre/post stack and locals state
//! - Expected vs actual types for type mismatches
//! - Deterministic "repro string" for bug reports
//!
//! # References
//!
//! - [JVMS §4.10 - Verification of class Files](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10)

use std::fmt;

use crate::attributes::Instruction;
use crate::verifiers::bytecode::frame::Frame;
use crate::verifiers::bytecode::type_system::VerificationType;

/// Verification diagnostic context for detailed error reporting.
#[derive(Debug, Clone)]
pub struct VerificationDiagnostic {
    /// Class name where the error occurred.
    pub class_name: String,

    /// Method name where the error occurred.
    pub method_name: String,

    /// Method descriptor.
    pub method_descriptor: String,

    /// Bytecode offset (PC) where the error occurred.
    pub pc: u16,

    /// The instruction at the error location (if available).
    pub instruction: Option<String>,

    /// Frame state before the instruction (if available).
    pub pre_frame: Option<FrameSnapshot>,

    /// Expected types (for type mismatch errors).
    pub expected: Option<Vec<VerificationType>>,

    /// Actual types encountered.
    pub actual: Option<Vec<VerificationType>>,

    /// The error message.
    pub message: String,

    /// Additional context or notes.
    pub notes: Vec<String>,
}

/// A snapshot of frame state for diagnostics.
#[derive(Debug, Clone)]
pub struct FrameSnapshot {
    /// Local variable types.
    pub locals: Vec<VerificationType>,
    /// Operand stack types.
    pub stack: Vec<VerificationType>,
}

impl From<&Frame> for FrameSnapshot {
    fn from(frame: &Frame) -> Self {
        Self {
            locals: frame.locals.clone(),
            stack: frame.stack.clone(),
        }
    }
}

impl VerificationDiagnostic {
    /// Creates a new diagnostic for a verification failure.
    pub fn new(
        class_name: impl Into<String>,
        method_name: impl Into<String>,
        method_descriptor: impl Into<String>,
        pc: u16,
        message: impl Into<String>,
    ) -> Self {
        Self {
            class_name: class_name.into(),
            method_name: method_name.into(),
            method_descriptor: method_descriptor.into(),
            pc,
            instruction: None,
            pre_frame: None,
            expected: None,
            actual: None,
            message: message.into(),
            notes: Vec::new(),
        }
    }

    /// Sets the instruction context.
    #[must_use]
    pub fn with_instruction(mut self, instruction: &Instruction) -> Self {
        self.instruction = Some(format!("{instruction:?}"));
        self
    }

    /// Sets the pre-instruction frame state.
    #[must_use]
    pub fn with_frame(mut self, frame: &Frame) -> Self {
        self.pre_frame = Some(FrameSnapshot::from(frame));
        self
    }

    /// Sets the expected types for type mismatch errors.
    #[must_use]
    pub fn with_expected(mut self, types: Vec<VerificationType>) -> Self {
        self.expected = Some(types);
        self
    }

    /// Sets the actual types encountered.
    #[must_use]
    pub fn with_actual(mut self, types: Vec<VerificationType>) -> Self {
        self.actual = Some(types);
        self
    }

    /// Adds a note to the diagnostic.
    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Generates a deterministic "repro string" for bug reports.
    ///
    /// This string uniquely identifies the verification failure and can
    /// be used to reproduce or look up the issue.
    #[must_use]
    pub fn repro_string(&self) -> String {
        format!(
            "{}#{}{} @{}: {}",
            self.class_name,
            self.method_name,
            self.method_descriptor,
            self.pc,
            self.message.chars().take(50).collect::<String>()
        )
    }

    /// Formats the diagnostic as a detailed multi-line string.
    #[must_use]
    pub fn detailed_string(&self) -> String {
        use std::fmt::Write;
        let mut result = String::new();

        let _ = writeln!(result, "VerifyError in {}", self.class_name);
        let _ = writeln!(
            result,
            "  Method: {}{}",
            self.method_name, self.method_descriptor
        );
        let _ = writeln!(result, "  PC: {}", self.pc);

        if let Some(ref instr) = self.instruction {
            let _ = writeln!(result, "  Instruction: {instr}");
        }

        let _ = writeln!(result, "  Error: {}", self.message);

        if let Some(expected) = &self.expected {
            push_type_list(&mut result, "Expected", expected);
        }

        if let Some(actual) = &self.actual {
            push_type_list(&mut result, "Actual", actual);
        }

        if let Some(ref frame) = self.pre_frame {
            result.push_str("  Frame state:\n");
            result.push_str("    Locals: [");
            for (i, t) in frame.locals.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                let _ = write!(result, "{t}");
            }
            result.push_str("]\n");

            result.push_str("    Stack: [");
            for (i, t) in frame.stack.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                let _ = write!(result, "{t}");
            }
            result.push_str("]\n");
        }

        for note in &self.notes {
            let _ = writeln!(result, "  Note: {note}");
        }

        let _ = writeln!(result, "  Repro: {}", self.repro_string());

        result
    }
}

fn push_type_list(result: &mut String, label: &str, types: &[VerificationType]) {
    use std::fmt::Write;

    result.push_str("  ");
    result.push_str(label);
    result.push_str(": ");
    for (i, t) in types.iter().enumerate() {
        if i > 0 {
            result.push_str(", ");
        }
        let _ = write!(result, "{t}");
    }
    result.push('\n');
}

impl fmt::Display for VerificationDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.detailed_string())
    }
}

/// Trace log for verbose verification output.
#[derive(Debug, Default)]
pub struct VerificationTrace {
    /// Whether tracing is enabled.
    enabled: bool,

    /// Collected trace entries.
    entries: Vec<TraceEntry>,
}

/// A single trace entry.
#[derive(Debug, Clone)]
pub struct TraceEntry {
    /// Bytecode offset.
    pub pc: u16,

    /// Instruction being verified.
    pub instruction: String,

    /// Frame state before execution.
    pub pre_stack: Vec<VerificationType>,

    /// Frame state after execution.
    pub post_stack: Vec<VerificationType>,

    /// Whether this is a `StackMapTable` anchor point.
    pub is_anchor: bool,

    /// Any additional notes.
    pub notes: Vec<String>,
}

impl VerificationTrace {
    /// Creates a new trace with tracing disabled.
    #[must_use]
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            entries: Vec::new(),
        }
    }

    /// Returns true if tracing is enabled.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Logs an instruction verification.
    pub fn log_instruction(
        &mut self,
        pc: u16,
        instruction: &Instruction,
        pre_frame: &Frame,
        post_frame: &Frame,
        is_anchor: bool,
    ) {
        if !self.enabled {
            return;
        }

        self.entries.push(TraceEntry {
            pc,
            instruction: format!("{instruction:?}"),
            pre_stack: pre_frame.stack.clone(),
            post_stack: post_frame.stack.clone(),
            is_anchor,
            notes: Vec::new(),
        });
    }

    /// Logs a note for the current position.
    pub fn log_note(&mut self, pc: u16, note: impl Into<String>) {
        if !self.enabled {
            return;
        }

        // Add to last entry if same PC, otherwise create new entry
        if let Some(last) = self.entries.last_mut()
            && last.pc == pc
        {
            last.notes.push(note.into());
            return;
        }

        self.entries.push(TraceEntry {
            pc,
            instruction: String::new(),
            pre_stack: Vec::new(),
            post_stack: Vec::new(),
            is_anchor: false,
            notes: vec![note.into()],
        });
    }

    /// Logs a `StackMapTable` anchor.
    pub fn log_anchor(&mut self, pc: u16, frame: &Frame) {
        if !self.enabled {
            return;
        }

        let mut entry = TraceEntry {
            pc,
            instruction: "[StackMapTable Frame]".to_string(),
            pre_stack: Vec::new(),
            post_stack: frame.stack.clone(),
            is_anchor: true,
            notes: Vec::new(),
        };

        entry.notes.push(format!("Locals: {:?}", frame.locals));

        self.entries.push(entry);
    }

    /// Returns all trace entries.
    #[must_use]
    pub fn entries(&self) -> &[TraceEntry] {
        &self.entries
    }

    /// Formats the trace as a multi-line string.
    #[must_use]
    pub fn format(&self) -> String {
        use std::fmt::Write;
        let mut result = String::new();

        for entry in &self.entries {
            if entry.is_anchor {
                let _ = writeln!(result, "\n=== PC {} [ANCHOR] ===", entry.pc);
            } else {
                let _ = writeln!(result, "PC {:4}: {}", entry.pc, entry.instruction);
            }

            if !entry.pre_stack.is_empty() || !entry.post_stack.is_empty() {
                let _ = writeln!(
                    result,
                    "         Stack: {:?} -> {:?}",
                    entry.pre_stack, entry.post_stack
                );
            }

            for note in &entry.notes {
                let _ = writeln!(result, "         Note: {note}");
            }
        }

        result
    }

    /// Clears all trace entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_repro_string() {
        let diag = VerificationDiagnostic::new(
            "com/example/Test",
            "testMethod",
            "(II)V",
            42,
            "Stack underflow",
        );

        let repro = diag.repro_string();
        assert!(repro.contains("com/example/Test"));
        assert!(repro.contains("testMethod"));
        assert!(repro.contains("42"));
    }

    #[test]
    fn test_diagnostic_with_types() {
        let diag = VerificationDiagnostic::new("Test", "foo", "()V", 10, "Type mismatch")
            .with_expected(vec![VerificationType::Integer])
            .with_actual(vec![VerificationType::Long]);

        let detailed = diag.detailed_string();
        assert!(detailed.contains("Expected:"));
        assert!(detailed.contains("Actual:"));
    }

    #[test]
    fn test_trace_logging() {
        let mut trace = VerificationTrace::new(true);

        let frame = Frame::new(2, 4);
        trace.log_anchor(0, &frame);
        trace.log_note(0, "Method entry");

        // log_note with same PC adds to the last entry's notes
        assert_eq!(trace.entries().len(), 1);
        assert!(trace.entries()[0].is_anchor);
        assert_eq!(trace.entries()[0].notes.len(), 2); // One from log_anchor, one from log_note
    }

    #[test]
    fn test_trace_disabled() {
        let mut trace = VerificationTrace::new(false);

        let frame = Frame::new(2, 4);
        trace.log_anchor(0, &frame);
        trace.log_instruction(1, &Instruction::Nop, &frame, &frame, false);
        trace.log_note(1, "disabled note");

        assert!(trace.entries().is_empty());
    }

    #[test]
    fn test_diagnostic_full_detail_and_display() {
        let mut frame = Frame::new(2, 2);
        frame.locals[0] = VerificationType::Integer;
        frame.stack.push(VerificationType::java_lang_object());
        frame.stack.push(VerificationType::Null);

        let message = "a".repeat(60);
        let diag = VerificationDiagnostic::new("Test", "method", "(I)V", 7, message)
            .with_instruction(&Instruction::Ireturn)
            .with_frame(&frame)
            .with_expected(vec![VerificationType::Integer, VerificationType::Float])
            .with_actual(vec![VerificationType::Long, VerificationType::Double])
            .with_note("first note")
            .with_note("second note");

        let details = diag.detailed_string();
        assert!(details.contains("Instruction: Ireturn"));
        assert!(details.contains("Frame state:"));
        assert!(details.contains("Locals: [int"));
        assert!(details.contains("Stack: [java/lang/Object, null"));
        assert!(details.contains("Expected: int, float"));
        assert!(details.contains("Actual: long, double"));
        assert!(details.contains("Note: first note"));
        assert!(details.contains("Repro: Test#method(I)V @7: "));
        assert_eq!(details, diag.to_string());
        assert!(diag.repro_string().len() < 90);
    }

    #[test]
    fn test_trace_instruction_note_format_and_clear() {
        let mut trace = VerificationTrace::new(true);
        assert!(trace.is_enabled());

        let mut pre = Frame::new(1, 2);
        pre.stack.push(VerificationType::Integer);
        let mut post = pre.clone();
        post.stack.push(VerificationType::Float);

        trace.log_instruction(2, &Instruction::Fconst_0, &pre, &post, false);
        trace.log_note(2, "same pc note");
        trace.log_anchor(4, &post);
        trace.log_note(3, "new pc note");

        assert_eq!(3, trace.entries().len());
        assert_eq!("Fconst_0", trace.entries()[0].instruction);
        assert_eq!(
            vec![VerificationType::Integer],
            trace.entries()[0].pre_stack
        );
        assert_eq!(
            vec![VerificationType::Integer, VerificationType::Float],
            trace.entries()[0].post_stack
        );
        assert_eq!(vec!["same pc note"], trace.entries()[0].notes);
        assert_eq!(3, trace.entries()[2].pc);

        let formatted = trace.format();
        assert!(formatted.contains("PC    2: Fconst_0"));
        assert!(formatted.contains("Stack:"));
        assert!(formatted.contains("Note: same pc note"));
        assert!(formatted.contains("PC    3:"));
        assert!(formatted.contains("[ANCHOR]"));

        trace.clear();
        assert!(trace.entries().is_empty());
    }
}
