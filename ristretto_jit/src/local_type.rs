use crate::Result;
use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::{BaseType, FieldType};

/// Represents an Object type local.
const OBJECT_TYPE: FieldType = FieldType::Object(String::new());

/// Trait to determine if an instruction corresponds to a local variable, and if so, to return its
/// type and index.
pub trait LocalType {
    /// Returns the index and type of the local variable if the instruction corresponds to one.
    fn local_type(&self) -> Result<Option<(usize, FieldType)>>;
}

impl LocalType for Instruction {
    fn local_type(&self) -> Result<Option<(usize, FieldType)>> {
        let local_type = match self {
            Instruction::Aload_0 | Instruction::Astore_0 => (0, OBJECT_TYPE),
            Instruction::Aload_1 | Instruction::Astore_1 => (1, OBJECT_TYPE),
            Instruction::Aload_2 | Instruction::Astore_2 => (2, OBJECT_TYPE),
            Instruction::Aload_3 | Instruction::Astore_3 => (3, OBJECT_TYPE),
            Instruction::Aload(index) | Instruction::Astore(index) => {
                (usize::from(*index), OBJECT_TYPE)
            }
            Instruction::Aload_w(index) | Instruction::Astore_w(index) => {
                (usize::from(*index), OBJECT_TYPE)
            }
            Instruction::Dload_0 | Instruction::Dstore_0 => (0, FieldType::Base(BaseType::Double)),
            Instruction::Dload_1 | Instruction::Dstore_1 => (1, FieldType::Base(BaseType::Double)),
            Instruction::Dload_2 | Instruction::Dstore_2 => (2, FieldType::Base(BaseType::Double)),
            Instruction::Dload_3 | Instruction::Dstore_3 => (3, FieldType::Base(BaseType::Double)),
            Instruction::Dload(index) | Instruction::Dstore(index) => {
                (usize::from(*index), FieldType::Base(BaseType::Double))
            }
            Instruction::Dload_w(index) | Instruction::Dstore_w(index) => {
                (usize::from(*index), FieldType::Base(BaseType::Double))
            }
            Instruction::Fload_0 | Instruction::Fstore_0 => (0, FieldType::Base(BaseType::Float)),
            Instruction::Fload_1 | Instruction::Fstore_1 => (1, FieldType::Base(BaseType::Float)),
            Instruction::Fload_2 | Instruction::Fstore_2 => (2, FieldType::Base(BaseType::Float)),
            Instruction::Fload_3 | Instruction::Fstore_3 => (3, FieldType::Base(BaseType::Float)),
            Instruction::Fload(index) | Instruction::Fstore(index) => {
                (usize::from(*index), FieldType::Base(BaseType::Float))
            }
            Instruction::Fload_w(index) | Instruction::Fstore_w(index) => {
                (usize::from(*index), FieldType::Base(BaseType::Float))
            }
            Instruction::Iload_0 | Instruction::Istore_0 => (0, FieldType::Base(BaseType::Int)),
            Instruction::Iload_1 | Instruction::Istore_1 => (1, FieldType::Base(BaseType::Int)),
            Instruction::Iload_2 | Instruction::Istore_2 => (2, FieldType::Base(BaseType::Int)),
            Instruction::Iload_3 | Instruction::Istore_3 => (3, FieldType::Base(BaseType::Int)),
            Instruction::Iload(index)
            | Instruction::Istore(index)
            | Instruction::Iinc(index, ..) => (usize::from(*index), FieldType::Base(BaseType::Int)),
            Instruction::Iload_w(index)
            | Instruction::Istore_w(index)
            | Instruction::Iinc_w(index, ..) => {
                (usize::from(*index), FieldType::Base(BaseType::Int))
            }
            Instruction::Lload_0 | Instruction::Lstore_0 => (0, FieldType::Base(BaseType::Long)),
            Instruction::Lload_1 | Instruction::Lstore_1 => (1, FieldType::Base(BaseType::Long)),
            Instruction::Lload_2 | Instruction::Lstore_2 => (2, FieldType::Base(BaseType::Long)),
            Instruction::Lload_3 | Instruction::Lstore_3 => (3, FieldType::Base(BaseType::Long)),
            Instruction::Lload(index) | Instruction::Lstore(index) => {
                (usize::from(*index), FieldType::Base(BaseType::Long))
            }
            Instruction::Lload_w(index) | Instruction::Lstore_w(index) => {
                (usize::from(*index), FieldType::Base(BaseType::Long))
            }
            _ => return Ok(None),
        };
        Ok(Some(local_type))
    }
}
