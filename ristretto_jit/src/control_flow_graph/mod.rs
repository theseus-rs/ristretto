/// This module creates a Control Flow Graph (CFG) for Java byte code using Cranelift
/// [Block](https://docs.rs/cranelift-codegen/latest/cranelift_codegen/ir/entities/struct.Block.html)
/// structures.  The Java stack is transformed into `Block` arguments with Static Single Assignment
/// (SSA) [Values](https://docs.rs/cranelift-codegen/latest/cranelift_codegen/ir/entities/struct.Value.html)
/// passed as parameters to blocks with branching operations (e.g. `jump`, `brif`, etc)
///
/// Java byte code for `Integer.max(II)I`:
/// ```rust
/// use ristretto_classfile::attributes::Instruction;
///
/// let instructions = vec![
///    Instruction::Iload_0,      // 0: Load local variable 0
///    Instruction::Iload_1,      // 1: Load local variable 1
///    Instruction::If_icmplt(5), // 2: If var0 < var1, branch to instruction 5
///    Instruction::Iload_0,      // 3: Load local variable 0
///    Instruction::Goto(6),      // 4: Jump to instruction 6
///    Instruction::Iload_1,      // 5: Load local variable 1
///    Instruction::Ireturn,      // 6: Return value on stack
/// ];
/// ```
///
/// Control flow graph diagram for `Integer.max(II)I` transformed into Cranelift blocks where the
/// stack state is managed using SSA by defining arguments on blocks and passing parameters:
/// ```text
///                         +------------------------------------------------------------+
///                         |                         block0 ()                          |
///                         |  // entry point with 2 local variables (l0: i32, l1: i32)  |
///                         |                                                            |
///                         |  ; Java 0: Iload_0                                         |
///                         |  v0 = load.i32 notrap aligned l0              stack: [v0]  |
///                         |                                                            |
///                         |  ; Java 1: Iload_1                                         |
///                         |  v1 = load.i32 notrap aligned l1          stack: [v0, v1]  |
///                         |                                                            |
///                         |  ; Java 2: If_icmplt(5)                                    |
///                         |  v2 = icmp slt v0, v1                           stack: []  |
///                         |  brif v2, block1, block2                        stack: []  |
///                         +------------------------------------------------------------+
///                                           /                        \
///                                     (cond == 1)                (cond == 0)
///                                         /                            \
///  +--------------------------------------------------+    +--------------------------------------------------+
///  |                    block1 ()                     |    |                    block2 ()                     |
///  |  // target of Java If_icmplt(5)                  |    |  // fall-through path to instructions 3 & 4      |
///  |                                                  |    |                                                  |
///  |  ; Java 5: Iload_1                               |    |  ; Java 3: Iload_0                               |
///  |  v3 = load.i32 notrap aligned l1    stack: [v3]  |    |  v4 = load.i32 notrap aligned l0    stack: [v4]  |
///  |                                                  |    |                                                  |
///  |  jump block3(v3)                    stack: [v3]  |    |  ; Java 4: Goto 6                                |
///  +--------------------------------------------------+    |  jump block3(v4)                    stack: [v4]  |
///                                         \                +--------------------------------------------------+
///                                          \                           /
///                                           \                         /
///                                            \                       /
///                                             v                     v
///                              +--------------------------------------------------+
///                              |             block3 (p0: i32)                     |
///                              |  // join point with 1 incoming i32 param         |
///                              |  v5 = load.i32 notrap aligned p0    stack: [v5]  |
///                              |                                                  |
///                              |  ; Java 6: Ireturn                               |
///                              |  return v5                            stack: []  |
///                              +--------------------------------------------------+
/// ```
///
mod blocks;
mod instruction;
mod type_stack;

pub(crate) use blocks::{append_block_params, get_blocks};
