//! # Control Flow Graph (CFG)
//!
//! This module creates a Control Flow Graph (CFG) for Java bytecode using Cranelift
//! [Block](https://docs.rs/cranelift-codegen/latest/cranelift_codegen/ir/entities/struct.Block.html)
//! structures. The Java operand stack is transformed into `Block` arguments with Static Single Assignment
//! (SSA) [Values](https://docs.rs/cranelift-codegen/latest/cranelift_codegen/ir/entities/struct.Value.html)
//! passed as parameters to blocks with branching operations.
//!
//! ## Purpose
//!
//! The CFG transformation serves as a critical step in JIT compilation, enabling:
//! - Conversion of stack-based Java bytecode to Cranelift's register-based IR
//! - Preservation of program control flow (branches, loops, etc.)
//! - Implementation of SSA form for optimizations
//! - Handling of Java's operand stack state across basic blocks
//!
//! ## Components
//!
//! This module consists of several submodules:
//! - `blocks`: Manages creation and configuration of Cranelift `Block` structures
//! - `control_flow`: Analyzes bytecode for control flow patterns
//! - `instruction`: Transforms Java bytecode instructions to Cranelift IR
//! - `type_stack`: Tracks operand types throughout control flow
//!
//! ## Examples
//!
//! Java byte code for `Integer.max(II)I`:
//! ```rust
//! use ristretto_classfile::attributes::Instruction;
//!
//! let instructions = vec![
//!    Instruction::Iload_0,      // 0: Load local variable 0
//!    Instruction::Iload_1,      // 1: Load local variable 1
//!    Instruction::If_icmplt(7), // 2: If var0 < var1, branch to byte offset 9
//!    Instruction::Iload_0,      // 5: Load local variable 0
//!    Instruction::Goto(4),      // 6: Jump to byte offset 10
//!    Instruction::Iload_1,      // 9: Load local variable 1
//!    Instruction::Ireturn,      // 10: Return value on stack
//! ];
//! ```
//!
//! Control flow graph diagram for `Integer.max(II)I` transformed into Cranelift blocks where the
//! stack state is managed using SSA by defining arguments on blocks and passing parameters:
//! ```text
//!                         ╭────────────────────────────────────────────────────────────╮
//!                         │                         block0 ()                          │
//!                         │  // entry point with 2 local variables (l0: i32, l1: i32)  │
//!                         │                                                            │
//!                         │  ; Java 0: Iload_0                                         │
//!                         │  v0 = load.i32 notrap aligned l0              stack: [v0]  │
//!                         │                                                            │
//!                         │  ; Java 1: Iload_1                                         │
//!                         │  v1 = load.i32 notrap aligned l1          stack: [v0, v1]  │
//!                         │                                                            │
//!                         │  ; Java 2: If_icmplt(7)                                    │
//!                         │  v2 = icmp slt v0, v1                     stack: [v0, v1]  │
//!                         │  brif v2, block1, block2                  stack: [v0, v1]  │
//!                         ╰────────────────────────────────────────────────────────────╯
//!                                           /                        \
//!                                     (cond == 1)                (cond == 0)
//!                                         /                            \
//!  ╭──────────────────────────────────────────────────╮    ╭──────────────────────────────────────────────────╮
//!  │                    block1 ()                     │    │                    block2 ()                     │
//!  │  // target of Java If_icmplt(7)                  │    │  // fall-through path to byte offsets 5 & 6      │
//!  │                                                  │    │                                                  │
//!  │  ; Java 9: Iload_1                               │    │  ; Java 5: Iload_0                               │
//!  │  v3 = load.i32 notrap aligned l1    stack: [v3]  │    │  v4 = load.i32 notrap aligned l0    stack: [v4]  │
//!  │                                                  │    │                                                  │
//!  │  jump block3(v3)                    stack: [v3]  │    │  ; Java 6: Goto 4                                │
//!  ╰──────────────────────────────────────────────────╯    │  jump block3(v4)                    stack: [v4]  │
//!                                         \                ╰──────────────────────────────────────────────────╯
//!                                          \                           /
//!                                           \                         /
//!                                            \                       /
//!                                             v                     v
//!                              ╭──────────────────────────────────────────────────╮
//!                              │             block3 (p0: i32)                     │
//!                              │  // join point with 1 incoming i32 param         │
//!                              │  v5 = load.i32 notrap aligned p0    stack: [v5]  │
//!                              │                                                  │
//!                              │  ; Java 10: Ireturn                               │
//!                              │  return v5                            stack: []  │
//!                              ╰──────────────────────────────────────────────────╯
//! ```
//!
mod blocks;
mod control_flow;
mod instruction;
mod type_stack;

/// Exports functions for appending parameters to blocks and retrieving the block structure
#[cfg(not(target_family = "wasm"))]
pub(crate) use blocks::{append_block_params, get_blocks};

/// Exports traits for analyzing control flow of instructions
#[cfg(not(target_family = "wasm"))]
pub(crate) use control_flow::InstructionControlFlow;
