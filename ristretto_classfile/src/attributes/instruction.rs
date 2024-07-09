use crate::attributes::ArrayType;
use crate::error::Error::InvalidInstruction;
use crate::error::Result;
use crate::Error::InvalidWideInstruction;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `Instruction`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-6.html#jvms-6.5>
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Nop,
    Aconst_null,
    Iconst_m1,
    Iconst_0,
    Iconst_1,
    Iconst_2,
    Iconst_3,
    Iconst_4,
    Iconst_5,
    Lconst_0,
    Lconst_1,
    Fconst_0,
    Fconst_1,
    Fconst_2,
    Dconst_0,
    Dconst_1,
    Bipush(u8),
    Sipush(i16),
    Ldc(u8),
    Ldc_w(u16),
    Ldc2_w(u16),
    Iload(u8),
    Lload(u8),
    Fload(u8),
    Dload(u8),
    Aload(u8),
    Iload_0,
    Iload_1,
    Iload_2,
    Iload_3,
    Lload_0,
    Lload_1,
    Lload_2,
    Lload_3,
    Fload_0,
    Fload_1,
    Fload_2,
    Fload_3,
    Dload_0,
    Dload_1,
    Dload_2,
    Dload_3,
    Aload_0,
    Aload_1,
    Aload_2,
    Aload_3,
    Iaload,
    Laload,
    Faload,
    Daload,
    Aaload,
    Baload,
    Caload,
    Saload,
    Istore(u8),
    Lstore(u8),
    Fstore(u8),
    Dstore(u8),
    Astore(u8),
    Istore_0,
    Istore_1,
    Istore_2,
    Istore_3,
    Lstore_0,
    Lstore_1,
    Lstore_2,
    Lstore_3,
    Fstore_0,
    Fstore_1,
    Fstore_2,
    Fstore_3,
    Dstore_0,
    Dstore_1,
    Dstore_2,
    Dstore_3,
    Astore_0,
    Astore_1,
    Astore_2,
    Astore_3,
    Iastore,
    Lastore,
    Fastore,
    Dastore,
    Aastore,
    Bastore,
    Castore,
    Sastore,
    Pop,
    Pop2,
    Dup,
    Dup_x1,
    Dup_x2,
    Dup2,
    Dup2_x1,
    Dup2_x2,
    Swap,
    Iadd,
    Ladd,
    Fadd,
    Dadd,
    Isub,
    Lsub,
    Fsub,
    Dsub,
    Imul,
    Lmul,
    Fmul,
    Dmul,
    Idiv,
    Ldiv,
    Fdiv,
    Ddiv,
    Irem,
    Lrem,
    Frem,
    Drem,
    Ineg,
    Lneg,
    Fneg,
    Dneg,
    Ishl,
    Lshl,
    Ishr,
    Lshr,
    Iushr,
    Lushr,
    Iand,
    Land,
    Ior,
    Lor,
    Ixor,
    Lxor,
    Iinc(u8, i8),
    I2l,
    I2f,
    I2d,
    L2i,
    L2f,
    L2d,
    F2i,
    F2l,
    F2d,
    D2i,
    D2l,
    D2f,
    I2b,
    I2c,
    I2s,
    Lcmp,
    Fcmpl,
    Fcmpg,
    Dcmpl,
    Dcmpg,
    Ifeq(u16),
    Ifne(u16),
    Iflt(u16),
    Ifge(u16),
    Ifgt(u16),
    Ifle(u16),
    If_icmpeq(u16),
    If_icmpne(u16),
    If_icmplt(u16),
    If_icmpge(u16),
    If_icmpgt(u16),
    If_icmple(u16),
    If_acmpeq(u16),
    If_acmpne(u16),
    Goto(u16),
    Jsr(u16),
    Ret(u8),
    Tableswitch {
        default: i32,
        low: i32,
        high: i32,
        offsets: Vec<i32>,
    },
    Lookupswitch {
        default: i32,
        pairs: Vec<(i32, i32)>,
    },
    Ireturn,
    Lreturn,
    Freturn,
    Dreturn,
    Areturn,
    Return,
    Getstatic(u16),
    Putstatic(u16),
    Getfield(u16),
    Putfield(u16),
    Invokevirtual(u16),
    Invokespecial(u16),
    Invokestatic(u16),
    Invokeinterface(u16, u8),
    Invokedynamic(u16),
    New(u16),
    Newarray(ArrayType),
    Anewarray(u16),
    Arraylength,
    Athrow,
    Checkcast(u16),
    Instanceof(u16),
    Monitorenter,
    Monitorexit,
    Wide,
    Multianewarray(u16, u8),
    Ifnull(u16),
    Ifnonnull(u16),
    Goto_w(u16),
    Jsr_w(u16),
    Breakpoint,
    // Wide instructions
    Iload_w(u16),
    Lload_w(u16),
    Fload_w(u16),
    Dload_w(u16),
    Aload_w(u16),
    Istore_w(u16),
    Lstore_w(u16),
    Fstore_w(u16),
    Dstore_w(u16),
    Astore_w(u16),
    Iinc_w(u16, i16),
    Ret_w(u16),
}

impl Instruction {
    /// Return the tag for the instruction element.
    #[allow(clippy::match_same_arms)]
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn code(&self) -> u8 {
        match self {
            Instruction::Nop => 0,                   // 0x00
            Instruction::Aconst_null => 1,           // 0x01
            Instruction::Iconst_m1 => 2,             // 0x02
            Instruction::Iconst_0 => 3,              // 0x03
            Instruction::Iconst_1 => 4,              // 0x04
            Instruction::Iconst_2 => 5,              // 0x05
            Instruction::Iconst_3 => 6,              // 0x06
            Instruction::Iconst_4 => 7,              // 0x07
            Instruction::Iconst_5 => 8,              // 0x08
            Instruction::Lconst_0 => 9,              // 0x09
            Instruction::Lconst_1 => 10,             // 0x0a
            Instruction::Fconst_0 => 11,             // 0x0b
            Instruction::Fconst_1 => 12,             // 0x0c
            Instruction::Fconst_2 => 13,             // 0x0d
            Instruction::Dconst_0 => 14,             // 0x0e
            Instruction::Dconst_1 => 15,             // 0x0f
            Instruction::Bipush(..) => 16,           // 0x10
            Instruction::Sipush(..) => 17,           // 0x11
            Instruction::Ldc(..) => 18,              // 0x12
            Instruction::Ldc_w(..) => 19,            // 0x13
            Instruction::Ldc2_w(..) => 20,           // 0x14
            Instruction::Iload(..) => 21,            // 0x15
            Instruction::Lload(..) => 22,            // 0x16
            Instruction::Fload(..) => 23,            // 0x17
            Instruction::Dload(..) => 24,            // 0x18
            Instruction::Aload(..) => 25,            // 0x19
            Instruction::Iload_0 => 26,              // 0x1a
            Instruction::Iload_1 => 27,              // 0x1b
            Instruction::Iload_2 => 28,              // 0x1c
            Instruction::Iload_3 => 29,              // 0x1d
            Instruction::Lload_0 => 30,              // 0x1e
            Instruction::Lload_1 => 31,              // 0x1f
            Instruction::Lload_2 => 32,              // 0x20
            Instruction::Lload_3 => 33,              // 0x21
            Instruction::Fload_0 => 34,              // 0x22
            Instruction::Fload_1 => 35,              // 0x23
            Instruction::Fload_2 => 36,              // 0x24
            Instruction::Fload_3 => 37,              // 0x25
            Instruction::Dload_0 => 38,              // 0x26
            Instruction::Dload_1 => 39,              // 0x27
            Instruction::Dload_2 => 40,              // 0x28
            Instruction::Dload_3 => 41,              // 0x29
            Instruction::Aload_0 => 42,              // 0x2a
            Instruction::Aload_1 => 43,              // 0x2b
            Instruction::Aload_2 => 44,              // 0x2c
            Instruction::Aload_3 => 45,              // 0x2d
            Instruction::Iaload => 46,               // 0x2e
            Instruction::Laload => 47,               // 0x2f
            Instruction::Faload => 48,               // 0x30
            Instruction::Daload => 49,               // 0x31
            Instruction::Aaload => 50,               // 0x32
            Instruction::Baload => 51,               // 0x33
            Instruction::Caload => 52,               // 0x34
            Instruction::Saload => 53,               // 0x35
            Instruction::Istore(..) => 54,           // 0x36
            Instruction::Lstore(..) => 55,           // 0x37
            Instruction::Fstore(..) => 56,           // 0x38
            Instruction::Dstore(..) => 57,           // 0x39
            Instruction::Astore(..) => 58,           // 0x3a
            Instruction::Istore_0 => 59,             // 0x3b
            Instruction::Istore_1 => 60,             // 0x3c
            Instruction::Istore_2 => 61,             // 0x3d
            Instruction::Istore_3 => 62,             // 0x3e
            Instruction::Lstore_0 => 63,             // 0x3f
            Instruction::Lstore_1 => 64,             // 0x40
            Instruction::Lstore_2 => 65,             // 0x41
            Instruction::Lstore_3 => 66,             // 0x42
            Instruction::Fstore_0 => 67,             // 0x43
            Instruction::Fstore_1 => 68,             // 0x44
            Instruction::Fstore_2 => 69,             // 0x45
            Instruction::Fstore_3 => 70,             // 0x46
            Instruction::Dstore_0 => 71,             // 0x47
            Instruction::Dstore_1 => 72,             // 0x48
            Instruction::Dstore_2 => 73,             // 0x49
            Instruction::Dstore_3 => 74,             // 0x4a
            Instruction::Astore_0 => 75,             // 0x4b
            Instruction::Astore_1 => 76,             // 0x4c
            Instruction::Astore_2 => 77,             // 0x4d
            Instruction::Astore_3 => 78,             // 0x4e
            Instruction::Iastore => 79,              // 0x4f
            Instruction::Lastore => 80,              // 0x50
            Instruction::Fastore => 81,              // 0x51
            Instruction::Dastore => 82,              // 0x52
            Instruction::Aastore => 83,              // 0x53
            Instruction::Bastore => 84,              // 0x54
            Instruction::Castore => 85,              // 0x55
            Instruction::Sastore => 86,              // 0x56
            Instruction::Pop => 87,                  // 0x57
            Instruction::Pop2 => 88,                 // 0x58
            Instruction::Dup => 89,                  // 0x59
            Instruction::Dup_x1 => 90,               // 0x5a
            Instruction::Dup_x2 => 91,               // 0x5b
            Instruction::Dup2 => 92,                 // 0x5c
            Instruction::Dup2_x1 => 93,              // 0x5d
            Instruction::Dup2_x2 => 94,              // 0x5e
            Instruction::Swap => 95,                 // 0x5f
            Instruction::Iadd => 96,                 // 0x60
            Instruction::Ladd => 97,                 // 0x61
            Instruction::Fadd => 98,                 // 0x62
            Instruction::Dadd => 99,                 // 0x63
            Instruction::Isub => 100,                // 0x64
            Instruction::Lsub => 101,                // 0x65
            Instruction::Fsub => 102,                // 0x66
            Instruction::Dsub => 103,                // 0x67
            Instruction::Imul => 104,                // 0x68
            Instruction::Lmul => 105,                // 0x69
            Instruction::Fmul => 106,                // 0x6a
            Instruction::Dmul => 107,                // 0x6b
            Instruction::Idiv => 108,                // 0x6c
            Instruction::Ldiv => 109,                // 0x6d
            Instruction::Fdiv => 110,                // 0x6e
            Instruction::Ddiv => 111,                // 0x6f
            Instruction::Irem => 112,                // 0x70
            Instruction::Lrem => 113,                // 0x71
            Instruction::Frem => 114,                // 0x72
            Instruction::Drem => 115,                // 0x73
            Instruction::Ineg => 116,                // 0x74
            Instruction::Lneg => 117,                // 0x75
            Instruction::Fneg => 118,                // 0x76
            Instruction::Dneg => 119,                // 0x77
            Instruction::Ishl => 120,                // 0x78
            Instruction::Lshl => 121,                // 0x79
            Instruction::Ishr => 122,                // 0x7a
            Instruction::Lshr => 123,                // 0x7b
            Instruction::Iushr => 124,               // 0x7c
            Instruction::Lushr => 125,               // 0x7d
            Instruction::Iand => 126,                // 0x7e
            Instruction::Land => 127,                // 0x7f
            Instruction::Ior => 128,                 // 0x80
            Instruction::Lor => 129,                 // 0x81
            Instruction::Ixor => 130,                // 0x82
            Instruction::Lxor => 131,                // 0x83
            Instruction::Iinc(..) => 132,            // 0x84
            Instruction::I2l => 133,                 // 0x85
            Instruction::I2f => 134,                 // 0x86
            Instruction::I2d => 135,                 // 0x87
            Instruction::L2i => 136,                 // 0x88
            Instruction::L2f => 137,                 // 0x89
            Instruction::L2d => 138,                 // 0x8a
            Instruction::F2i => 139,                 // 0x8b
            Instruction::F2l => 140,                 // 0x8c
            Instruction::F2d => 141,                 // 0x8d
            Instruction::D2i => 142,                 // 0x8e
            Instruction::D2l => 143,                 // 0x8f
            Instruction::D2f => 144,                 // 0x90
            Instruction::I2b => 145,                 // 0x91
            Instruction::I2c => 146,                 // 0x92
            Instruction::I2s => 147,                 // 0x93
            Instruction::Lcmp => 148,                // 0x94
            Instruction::Fcmpl => 149,               // 0x95
            Instruction::Fcmpg => 150,               // 0x96
            Instruction::Dcmpl => 151,               // 0x97
            Instruction::Dcmpg => 152,               // 0x98
            Instruction::Ifeq(..) => 153,            // 0x99
            Instruction::Ifne(..) => 154,            // 0x9a
            Instruction::Iflt(..) => 155,            // 0x9b
            Instruction::Ifge(..) => 156,            // 0x9c
            Instruction::Ifgt(..) => 157,            // 0x9d
            Instruction::Ifle(..) => 158,            // 0x9e
            Instruction::If_icmpeq(..) => 159,       // 0x9f
            Instruction::If_icmpne(..) => 160,       // 0xa0
            Instruction::If_icmplt(..) => 161,       // 0xa1
            Instruction::If_icmpge(..) => 162,       // 0xa2
            Instruction::If_icmpgt(..) => 163,       // 0xa3
            Instruction::If_icmple(..) => 164,       // 0xa4
            Instruction::If_acmpeq(..) => 165,       // 0xa5
            Instruction::If_acmpne(..) => 166,       // 0xa6
            Instruction::Goto(..) => 167,            // 0xa7
            Instruction::Jsr(..) => 168,             // 0xa8
            Instruction::Ret(..) => 169,             // 0xa9
            Instruction::Tableswitch { .. } => 170,  // 0xaa
            Instruction::Lookupswitch { .. } => 171, // 0xab
            Instruction::Ireturn => 172,             // 0xac
            Instruction::Lreturn => 173,             // 0xad
            Instruction::Freturn => 174,             // 0xae
            Instruction::Dreturn => 175,             // 0xaf
            Instruction::Areturn => 176,             // 0xb0
            Instruction::Return => 177,              // 0xb1
            Instruction::Getstatic(..) => 178,       // 0xb2
            Instruction::Putstatic(..) => 179,       // 0xb3
            Instruction::Getfield(..) => 180,        // 0xb4
            Instruction::Putfield(..) => 181,        // 0xb5
            Instruction::Invokevirtual(..) => 182,   // 0xb6
            Instruction::Invokespecial(..) => 183,   // 0xb7
            Instruction::Invokestatic(..) => 184,    // 0xb8
            Instruction::Invokeinterface(..) => 185, // 0xb9
            Instruction::Invokedynamic(..) => 186,   // 0xba
            Instruction::New(..) => 187,             // 0xbb
            Instruction::Newarray(..) => 188,        // 0xbc
            Instruction::Anewarray(..) => 189,       // 0xbd
            Instruction::Arraylength => 190,         // 0xbe
            Instruction::Athrow => 191,              // 0xbf
            Instruction::Checkcast(..) => 192,       // 0xc0
            Instruction::Instanceof(..) => 193,      // 0xc1
            Instruction::Monitorenter => 194,        // 0xc2
            Instruction::Monitorexit => 195,         // 0xc3
            Instruction::Wide => 196,                // 0xc4
            Instruction::Multianewarray(..) => 197,  // 0xc5
            Instruction::Ifnull(..) => 198,          // 0xc6
            Instruction::Ifnonnull(..) => 199,       // 0xc7
            Instruction::Goto_w(..) => 200,          // 0xc8
            Instruction::Jsr_w(..) => 201,           // 0xc9
            Instruction::Breakpoint => 202,          // 0xca
            // Wide instructions
            Instruction::Iload_w(..) => 196,  // 0xc4
            Instruction::Lload_w(..) => 196,  // 0xc4
            Instruction::Fload_w(..) => 196,  // 0xc4
            Instruction::Dload_w(..) => 196,  // 0xc4
            Instruction::Aload_w(..) => 196,  // 0xc4
            Instruction::Istore_w(..) => 196, // 0xc4
            Instruction::Lstore_w(..) => 196, // 0xc4
            Instruction::Fstore_w(..) => 196, // 0xc4
            Instruction::Dstore_w(..) => 196, // 0xc4
            Instruction::Astore_w(..) => 196, // 0xc4
            Instruction::Iinc_w(..) => 196,   // 0xc4
            Instruction::Ret_w(..) => 196,    // 0xc4
        }
    }

    /// Deserialize the `Instruction` from bytes.
    ///
    /// # Errors
    /// Returns an error if the instruction is invalid.
    #[allow(clippy::too_many_lines)]
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<Instruction> {
        let current_position = i32::try_from(bytes.position())?;
        let code = bytes.read_u8()?;

        let instruction = match code {
            0 => Instruction::Nop,
            1 => Instruction::Aconst_null,
            2 => Instruction::Iconst_m1,
            3 => Instruction::Iconst_0,
            4 => Instruction::Iconst_1,
            5 => Instruction::Iconst_2,
            6 => Instruction::Iconst_3,
            7 => Instruction::Iconst_4,
            8 => Instruction::Iconst_5,
            9 => Instruction::Lconst_0,
            10 => Instruction::Lconst_1,
            11 => Instruction::Fconst_0,
            12 => Instruction::Fconst_1,
            13 => Instruction::Fconst_2,
            14 => Instruction::Dconst_0,
            15 => Instruction::Dconst_1,
            16 => Instruction::Bipush(bytes.read_u8()?),
            17 => Instruction::Sipush(bytes.read_i16::<BigEndian>()?),
            18 => Instruction::Ldc(bytes.read_u8()?),
            19 => Instruction::Ldc_w(bytes.read_u16::<BigEndian>()?),
            20 => Instruction::Ldc2_w(bytes.read_u16::<BigEndian>()?),
            21 => Instruction::Iload(bytes.read_u8()?),
            22 => Instruction::Lload(bytes.read_u8()?),
            23 => Instruction::Fload(bytes.read_u8()?),
            24 => Instruction::Dload(bytes.read_u8()?),
            25 => Instruction::Aload(bytes.read_u8()?),
            26 => Instruction::Iload_0,
            27 => Instruction::Iload_1,
            28 => Instruction::Iload_2,
            29 => Instruction::Iload_3,
            30 => Instruction::Lload_0,
            31 => Instruction::Lload_1,
            32 => Instruction::Lload_2,
            33 => Instruction::Lload_3,
            34 => Instruction::Fload_0,
            35 => Instruction::Fload_1,
            36 => Instruction::Fload_2,
            37 => Instruction::Fload_3,
            38 => Instruction::Dload_0,
            39 => Instruction::Dload_1,
            40 => Instruction::Dload_2,
            41 => Instruction::Dload_3,
            42 => Instruction::Aload_0,
            43 => Instruction::Aload_1,
            44 => Instruction::Aload_2,
            45 => Instruction::Aload_3,
            46 => Instruction::Iaload,
            47 => Instruction::Laload,
            48 => Instruction::Faload,
            49 => Instruction::Daload,
            50 => Instruction::Aaload,
            51 => Instruction::Baload,
            52 => Instruction::Caload,
            53 => Instruction::Saload,
            54 => Instruction::Istore(bytes.read_u8()?),
            55 => Instruction::Lstore(bytes.read_u8()?),
            56 => Instruction::Fstore(bytes.read_u8()?),
            57 => Instruction::Dstore(bytes.read_u8()?),
            58 => Instruction::Astore(bytes.read_u8()?),
            59 => Instruction::Istore_0,
            60 => Instruction::Istore_1,
            61 => Instruction::Istore_2,
            62 => Instruction::Istore_3,
            63 => Instruction::Lstore_0,
            64 => Instruction::Lstore_1,
            65 => Instruction::Lstore_2,
            66 => Instruction::Lstore_3,
            67 => Instruction::Fstore_0,
            68 => Instruction::Fstore_1,
            69 => Instruction::Fstore_2,
            70 => Instruction::Fstore_3,
            71 => Instruction::Dstore_0,
            72 => Instruction::Dstore_1,
            73 => Instruction::Dstore_2,
            74 => Instruction::Dstore_3,
            75 => Instruction::Astore_0,
            76 => Instruction::Astore_1,
            77 => Instruction::Astore_2,
            78 => Instruction::Astore_3,
            79 => Instruction::Iastore,
            80 => Instruction::Lastore,
            81 => Instruction::Fastore,
            82 => Instruction::Dastore,
            83 => Instruction::Aastore,
            84 => Instruction::Bastore,
            85 => Instruction::Castore,
            86 => Instruction::Sastore,
            87 => Instruction::Pop,
            88 => Instruction::Pop2,
            89 => Instruction::Dup,
            90 => Instruction::Dup_x1,
            91 => Instruction::Dup_x2,
            92 => Instruction::Dup2,
            93 => Instruction::Dup2_x1,
            94 => Instruction::Dup2_x2,
            95 => Instruction::Swap,
            96 => Instruction::Iadd,
            97 => Instruction::Ladd,
            98 => Instruction::Fadd,
            99 => Instruction::Dadd,
            100 => Instruction::Isub,
            101 => Instruction::Lsub,
            102 => Instruction::Fsub,
            103 => Instruction::Dsub,
            104 => Instruction::Imul,
            105 => Instruction::Lmul,
            106 => Instruction::Fmul,
            107 => Instruction::Dmul,
            108 => Instruction::Idiv,
            109 => Instruction::Ldiv,
            110 => Instruction::Fdiv,
            111 => Instruction::Ddiv,
            112 => Instruction::Irem,
            113 => Instruction::Lrem,
            114 => Instruction::Frem,
            115 => Instruction::Drem,
            116 => Instruction::Ineg,
            117 => Instruction::Lneg,
            118 => Instruction::Fneg,
            119 => Instruction::Dneg,
            120 => Instruction::Ishl,
            121 => Instruction::Lshl,
            122 => Instruction::Ishr,
            123 => Instruction::Lshr,
            124 => Instruction::Iushr,
            125 => Instruction::Lushr,
            126 => Instruction::Iand,
            127 => Instruction::Land,
            128 => Instruction::Ior,
            129 => Instruction::Lor,
            130 => Instruction::Ixor,
            131 => Instruction::Lxor,
            132 => Instruction::Iinc(bytes.read_u8()?, bytes.read_i8()?),
            133 => Instruction::I2l,
            134 => Instruction::I2f,
            135 => Instruction::I2d,
            136 => Instruction::L2i,
            137 => Instruction::L2f,
            138 => Instruction::L2d,
            139 => Instruction::F2i,
            140 => Instruction::F2l,
            141 => Instruction::F2d,
            142 => Instruction::D2i,
            143 => Instruction::D2l,
            144 => Instruction::D2f,
            145 => Instruction::I2b,
            146 => Instruction::I2c,
            147 => Instruction::I2s,
            148 => Instruction::Lcmp,
            149 => Instruction::Fcmpl,
            150 => Instruction::Fcmpg,
            151 => Instruction::Dcmpl,
            152 => Instruction::Dcmpg,
            153 => Instruction::Ifeq(Self::read_offset(bytes, current_position)?),
            154 => Instruction::Ifne(Self::read_offset(bytes, current_position)?),
            155 => Instruction::Iflt(Self::read_offset(bytes, current_position)?),
            156 => Instruction::Ifge(Self::read_offset(bytes, current_position)?),
            157 => Instruction::Ifgt(Self::read_offset(bytes, current_position)?),
            158 => Instruction::Ifle(Self::read_offset(bytes, current_position)?),
            159 => Instruction::If_icmpeq(Self::read_offset(bytes, current_position)?),
            160 => Instruction::If_icmpne(Self::read_offset(bytes, current_position)?),
            161 => Instruction::If_icmplt(Self::read_offset(bytes, current_position)?),
            162 => Instruction::If_icmpge(Self::read_offset(bytes, current_position)?),
            163 => Instruction::If_icmpgt(Self::read_offset(bytes, current_position)?),
            164 => Instruction::If_icmple(Self::read_offset(bytes, current_position)?),
            165 => Instruction::If_acmpeq(Self::read_offset(bytes, current_position)?),
            166 => Instruction::If_acmpne(Self::read_offset(bytes, current_position)?),
            167 => Instruction::Goto(Self::read_offset(bytes, current_position)?),
            168 => Instruction::Jsr(Self::read_offset(bytes, current_position)?),
            169 => Instruction::Ret(bytes.read_u8()?),
            170 => {
                let position = u32::try_from(bytes.position())?;
                let padding = (4 - (position % 4)) % 4;
                for _ in 0..padding {
                    bytes.read_u8()?;
                }
                let default = bytes.read_i32::<BigEndian>()?;
                let low = bytes.read_i32::<BigEndian>()?;
                let high = bytes.read_i32::<BigEndian>()?;
                let mut offsets = Vec::new();
                for _ in low..=high {
                    let offset = bytes.read_i32::<BigEndian>()?;
                    offsets.push(offset);
                }
                Instruction::Tableswitch {
                    default,
                    low,
                    high,
                    offsets,
                }
            }
            171 => {
                let position = u32::try_from(bytes.position())?;
                let padding = (4 - (position % 4)) % 4;
                for _ in 0..padding {
                    bytes.read_u8()?;
                }
                let default = bytes.read_i32::<BigEndian>()?;
                let npairs = bytes.read_i32::<BigEndian>()?;
                let mut pairs = Vec::new();
                for _ in 0..npairs {
                    let match_ = bytes.read_i32::<BigEndian>()?;
                    let offset = bytes.read_i32::<BigEndian>()?;
                    pairs.push((match_, offset));
                }
                Instruction::Lookupswitch { default, pairs }
            }
            172 => Instruction::Ireturn,
            173 => Instruction::Lreturn,
            174 => Instruction::Freturn,
            175 => Instruction::Dreturn,
            176 => Instruction::Areturn,
            177 => Instruction::Return,
            178 => Instruction::Getstatic(bytes.read_u16::<BigEndian>()?),
            179 => Instruction::Putstatic(bytes.read_u16::<BigEndian>()?),
            180 => Instruction::Getfield(bytes.read_u16::<BigEndian>()?),
            181 => Instruction::Putfield(bytes.read_u16::<BigEndian>()?),
            182 => Instruction::Invokevirtual(bytes.read_u16::<BigEndian>()?),
            183 => Instruction::Invokespecial(bytes.read_u16::<BigEndian>()?),
            184 => Instruction::Invokestatic(bytes.read_u16::<BigEndian>()?),
            185 => {
                let constant_index = bytes.read_u16::<BigEndian>()?;
                let count = bytes.read_u8()?;
                let null = bytes.read_u8()?;
                if null != 0 {
                    return Err(InvalidInstruction(code));
                }
                Instruction::Invokeinterface(constant_index, count)
            }
            186 => Instruction::Invokedynamic(bytes.read_u16::<BigEndian>()?),
            187 => Instruction::New(bytes.read_u16::<BigEndian>()?),
            188 => {
                let array_type = ArrayType::from_bytes(bytes)?;
                Instruction::Newarray(array_type)
            }
            189 => Instruction::Anewarray(bytes.read_u16::<BigEndian>()?),
            190 => Instruction::Arraylength,
            191 => Instruction::Athrow,
            192 => Instruction::Checkcast(bytes.read_u16::<BigEndian>()?),
            193 => Instruction::Instanceof(bytes.read_u16::<BigEndian>()?),
            194 => Instruction::Monitorenter,
            195 => Instruction::Monitorexit,
            196 => {
                // Wide instructions
                // Get the next byte to determine the wide instruction
                let wide_code = bytes.read_u8()?;
                match wide_code {
                    21 => Instruction::Iload_w(bytes.read_u16::<BigEndian>()?),
                    22 => Instruction::Lload_w(bytes.read_u16::<BigEndian>()?),
                    23 => Instruction::Fload_w(bytes.read_u16::<BigEndian>()?),
                    24 => Instruction::Dload_w(bytes.read_u16::<BigEndian>()?),
                    25 => Instruction::Aload_w(bytes.read_u16::<BigEndian>()?),
                    54 => Instruction::Istore_w(bytes.read_u16::<BigEndian>()?),
                    55 => Instruction::Lstore_w(bytes.read_u16::<BigEndian>()?),
                    56 => Instruction::Fstore_w(bytes.read_u16::<BigEndian>()?),
                    57 => Instruction::Dstore_w(bytes.read_u16::<BigEndian>()?),
                    58 => Instruction::Astore_w(bytes.read_u16::<BigEndian>()?),
                    132 => Instruction::Iinc_w(
                        bytes.read_u16::<BigEndian>()?,
                        bytes.read_i16::<BigEndian>()?,
                    ),
                    169 => Instruction::Ret_w(bytes.read_u16::<BigEndian>()?),
                    _ => return Err(InvalidWideInstruction(wide_code)),
                }
            }
            197 => Instruction::Multianewarray(bytes.read_u16::<BigEndian>()?, bytes.read_u8()?),
            198 => Instruction::Ifnull(Self::read_offset(bytes, current_position)?),
            199 => Instruction::Ifnonnull(Self::read_offset(bytes, current_position)?),
            200 => {
                let offset = bytes.read_i32::<BigEndian>()?;
                let position = u16::try_from(current_position + offset)?;
                Instruction::Goto_w(position)
            }
            201 => {
                let offset = bytes.read_i32::<BigEndian>()?;
                let position = u16::try_from(current_position + offset)?;
                Instruction::Jsr_w(position)
            }
            202 => Instruction::Breakpoint,
            _ => return Err(InvalidInstruction(code)),
        };
        Ok(instruction)
    }

    /// Read a signed 16-bit offset from the bytes.
    ///
    /// # Errors
    /// Returns an error if the offset is invalid.
    fn read_offset(bytes: &mut Cursor<Vec<u8>>, current_position: i32) -> Result<u16> {
        let offset = bytes.read_i16::<BigEndian>()?;
        let position = u16::try_from(current_position + i32::from(offset))?;
        Ok(position)
    }

    /// Serialize the `Instruction` to bytes.
    ///
    /// # Errors
    /// If an instruction cannot be serialized to bytes.
    #[allow(clippy::match_same_arms)]
    #[allow(clippy::too_many_lines)]
    pub fn to_bytes(&self, bytes: &mut Cursor<Vec<u8>>) -> Result<()> {
        bytes.write_u8(self.code())?;

        match self {
            Instruction::Bipush(value) => bytes.write_u8(*value)?,
            Instruction::Sipush(value) => bytes.write_i16::<BigEndian>(*value)?,
            Instruction::Ldc(value) => bytes.write_u8(*value)?,
            Instruction::Ldc_w(value) => bytes.write_u16::<BigEndian>(*value)?,
            Instruction::Ldc2_w(value) => bytes.write_u16::<BigEndian>(*value)?,
            Instruction::Iload(value) => bytes.write_u8(*value)?,
            Instruction::Lload(value) => bytes.write_u8(*value)?,
            Instruction::Fload(value) => bytes.write_u8(*value)?,
            Instruction::Dload(value) => bytes.write_u8(*value)?,
            Instruction::Aload(value) => bytes.write_u8(*value)?,
            Instruction::Istore(value) => bytes.write_u8(*value)?,
            Instruction::Lstore(value) => bytes.write_u8(*value)?,
            Instruction::Fstore(value) => bytes.write_u8(*value)?,
            Instruction::Dstore(value) => bytes.write_u8(*value)?,
            Instruction::Astore(value) => bytes.write_u8(*value)?,
            Instruction::Iinc(value1, value2) => {
                bytes.write_u8(*value1)?;
                bytes.write_i8(*value2)?;
            }
            Instruction::Ifeq(value) => Self::write_offset(bytes, *value)?,
            Instruction::Ifne(value) => Self::write_offset(bytes, *value)?,
            Instruction::Iflt(value) => Self::write_offset(bytes, *value)?,
            Instruction::Ifge(value) => Self::write_offset(bytes, *value)?,
            Instruction::Ifgt(value) => Self::write_offset(bytes, *value)?,
            Instruction::Ifle(value) => Self::write_offset(bytes, *value)?,
            Instruction::If_icmpeq(value) => Self::write_offset(bytes, *value)?,
            Instruction::If_icmpne(value) => Self::write_offset(bytes, *value)?,
            Instruction::If_icmplt(value) => Self::write_offset(bytes, *value)?,
            Instruction::If_icmpge(value) => Self::write_offset(bytes, *value)?,
            Instruction::If_icmpgt(value) => Self::write_offset(bytes, *value)?,
            Instruction::If_icmple(value) => Self::write_offset(bytes, *value)?,
            Instruction::If_acmpeq(value) => Self::write_offset(bytes, *value)?,
            Instruction::If_acmpne(value) => Self::write_offset(bytes, *value)?,
            Instruction::Goto(value) => Self::write_offset(bytes, *value)?,
            Instruction::Jsr(value) => Self::write_offset(bytes, *value)?,
            Instruction::Ret(value) => bytes.write_u8(*value)?,
            Instruction::Tableswitch {
                default,
                low,
                high,
                offsets,
            } => {
                let current_position = i32::try_from(bytes.position())?;
                let position = u32::try_from(current_position + 1)?;
                let padding = (4 - (position % 4)) % 4;
                for _ in 0..padding {
                    bytes.write_u8(0)?;
                }
                bytes.write_i32::<BigEndian>(*default)?;
                bytes.write_i32::<BigEndian>(*low)?;
                bytes.write_i32::<BigEndian>(*high)?;
                for offset in offsets {
                    bytes.write_i32::<BigEndian>(*offset)?;
                }
            }
            Instruction::Lookupswitch { pairs, default } => {
                let current_position = i32::try_from(bytes.position())?;
                let position = u32::try_from(current_position + 1)?;
                let padding = (4 - (position % 4)) % 4;
                for _ in 0..padding {
                    bytes.write_u8(0)?;
                }
                bytes.write_i32::<BigEndian>(*default)?;
                let pairs_count = i32::try_from(pairs.len())?;
                bytes.write_i32::<BigEndian>(pairs_count)?;
                for (match_, offset) in pairs {
                    bytes.write_i32::<BigEndian>(*match_)?;
                    bytes.write_i32::<BigEndian>(*offset)?;
                }
            }
            Instruction::Getstatic(value) => bytes.write_u16::<BigEndian>(*value)?,
            Instruction::Putstatic(value) => bytes.write_u16::<BigEndian>(*value)?,
            Instruction::Getfield(value) => bytes.write_u16::<BigEndian>(*value)?,
            Instruction::Putfield(value) => bytes.write_u16::<BigEndian>(*value)?,
            Instruction::Invokevirtual(value) => bytes.write_u16::<BigEndian>(*value)?,
            Instruction::Invokespecial(value) => bytes.write_u16::<BigEndian>(*value)?,
            Instruction::Invokestatic(value) => bytes.write_u16::<BigEndian>(*value)?,
            Instruction::Invokeinterface(value1, value2) => {
                bytes.write_u16::<BigEndian>(*value1)?;
                bytes.write_u8(*value2)?;
                bytes.write_u8(0)?;
            }
            Instruction::Invokedynamic(value) => bytes.write_u16::<BigEndian>(*value)?,
            Instruction::New(value) => bytes.write_u16::<BigEndian>(*value)?,
            Instruction::Newarray(array_type) => array_type.to_bytes(bytes)?,
            Instruction::Anewarray(value) => bytes.write_u16::<BigEndian>(*value)?,
            Instruction::Checkcast(value) => bytes.write_u16::<BigEndian>(*value)?,
            Instruction::Instanceof(value) => bytes.write_u16::<BigEndian>(*value)?,
            Instruction::Multianewarray(value1, value2) => {
                bytes.write_u16::<BigEndian>(*value1)?;
                bytes.write_u8(*value2)?;
            }
            Instruction::Ifnull(value) => Self::write_offset(bytes, *value)?,
            Instruction::Ifnonnull(value) => Self::write_offset(bytes, *value)?,
            Instruction::Goto_w(value) => {
                let current_position = i32::try_from(bytes.position())? - 1;
                let offset = i32::from(*value) - current_position;
                bytes.write_i32::<BigEndian>(offset)?;
            }
            Instruction::Jsr_w(value) => {
                let current_position = i32::try_from(bytes.position())? - 1;
                let offset = i32::from(*value) - current_position;
                bytes.write_i32::<BigEndian>(offset)?;
            }
            // Wide instructions
            Instruction::Iload_w(value) => {
                bytes.write_u8(21)?;
                bytes.write_u16::<BigEndian>(*value)?;
            }
            Instruction::Lload_w(value) => {
                bytes.write_u8(22)?;
                bytes.write_u16::<BigEndian>(*value)?;
            }
            Instruction::Fload_w(value) => {
                bytes.write_u8(23)?;
                bytes.write_u16::<BigEndian>(*value)?;
            }
            Instruction::Dload_w(value) => {
                bytes.write_u8(24)?;
                bytes.write_u16::<BigEndian>(*value)?;
            }
            Instruction::Aload_w(value) => {
                bytes.write_u8(25)?;
                bytes.write_u16::<BigEndian>(*value)?;
            }
            Instruction::Istore_w(value) => {
                bytes.write_u8(54)?;
                bytes.write_u16::<BigEndian>(*value)?;
            }
            Instruction::Lstore_w(value) => {
                bytes.write_u8(55)?;
                bytes.write_u16::<BigEndian>(*value)?;
            }
            Instruction::Fstore_w(value) => {
                bytes.write_u8(56)?;
                bytes.write_u16::<BigEndian>(*value)?;
            }
            Instruction::Dstore_w(value) => {
                bytes.write_u8(57)?;
                bytes.write_u16::<BigEndian>(*value)?;
            }
            Instruction::Astore_w(value) => {
                bytes.write_u8(58)?;
                bytes.write_u16::<BigEndian>(*value)?;
            }
            Instruction::Iinc_w(value1, value2) => {
                bytes.write_u8(132)?;
                bytes.write_u16::<BigEndian>(*value1)?;
                bytes.write_i16::<BigEndian>(*value2)?;
            }
            Instruction::Ret_w(value) => {
                bytes.write_u8(169)?;
                bytes.write_u16::<BigEndian>(*value)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Write a signed 16-bit offset to the bytes.
    ///
    /// # Errors
    /// Returns an error if the offset is invalid.
    fn write_offset(bytes: &mut Cursor<Vec<u8>>, offset: u16) -> Result<()> {
        let current_position = i32::try_from(bytes.position())? - 1;
        let offset = i16::try_from(i32::from(offset) - current_position)?;
        bytes.write_i16::<BigEndian>(offset)?;
        Ok(())
    }
}

impl fmt::Display for Instruction {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Nop => write!(f, "nop"),
            Instruction::Aconst_null => write!(f, "aconst_null"),
            Instruction::Iconst_m1 => write!(f, "iconst_m1"),
            Instruction::Iconst_0 => write!(f, "iconst_0"),
            Instruction::Iconst_1 => write!(f, "iconst_1"),
            Instruction::Iconst_2 => write!(f, "iconst_2"),
            Instruction::Iconst_3 => write!(f, "iconst_3"),
            Instruction::Iconst_4 => write!(f, "iconst_4"),
            Instruction::Iconst_5 => write!(f, "iconst_5"),
            Instruction::Lconst_0 => write!(f, "lconst_0"),
            Instruction::Lconst_1 => write!(f, "lconst_1"),
            Instruction::Fconst_0 => write!(f, "fconst_0"),
            Instruction::Fconst_1 => write!(f, "fconst_1"),
            Instruction::Fconst_2 => write!(f, "fconst_2"),
            Instruction::Dconst_0 => write!(f, "dconst_0"),
            Instruction::Dconst_1 => write!(f, "dconst_1"),
            Instruction::Bipush(value) => write!(f, "bipush {value}"),
            Instruction::Sipush(value) => write!(f, "sipush {value}"),
            Instruction::Ldc(value) => write!(f, "ldc #{value}"),
            Instruction::Ldc_w(value) => write!(f, "ldc_w #{value}"),
            Instruction::Ldc2_w(value) => write!(f, "ldc2_w #{value}"),
            Instruction::Iload(value) => write!(f, "iload {value}"),
            Instruction::Lload(value) => write!(f, "lload {value}"),
            Instruction::Fload(value) => write!(f, "fload {value}"),
            Instruction::Dload(value) => write!(f, "dload {value}"),
            Instruction::Aload(value) => write!(f, "aload {value}"),
            Instruction::Iload_0 => write!(f, "iload_0"),
            Instruction::Iload_1 => write!(f, "iload_1"),
            Instruction::Iload_2 => write!(f, "iload_2"),
            Instruction::Iload_3 => write!(f, "iload_3"),
            Instruction::Lload_0 => write!(f, "lload_0"),
            Instruction::Lload_1 => write!(f, "lload_1"),
            Instruction::Lload_2 => write!(f, "lload_2"),
            Instruction::Lload_3 => write!(f, "lload_3"),
            Instruction::Fload_0 => write!(f, "fload_0"),
            Instruction::Fload_1 => write!(f, "fload_1"),
            Instruction::Fload_2 => write!(f, "fload_2"),
            Instruction::Fload_3 => write!(f, "fload_3"),
            Instruction::Dload_0 => write!(f, "dload_0"),
            Instruction::Dload_1 => write!(f, "dload_1"),
            Instruction::Dload_2 => write!(f, "dload_2"),
            Instruction::Dload_3 => write!(f, "dload_3"),
            Instruction::Aload_0 => write!(f, "aload_0"),
            Instruction::Aload_1 => write!(f, "aload_1"),
            Instruction::Aload_2 => write!(f, "aload_2"),
            Instruction::Aload_3 => write!(f, "aload_3"),
            Instruction::Iaload => write!(f, "iaload"),
            Instruction::Laload => write!(f, "laload"),
            Instruction::Faload => write!(f, "faload"),
            Instruction::Daload => write!(f, "daload"),
            Instruction::Aaload => write!(f, "aaload"),
            Instruction::Baload => write!(f, "baload"),
            Instruction::Caload => write!(f, "caload"),
            Instruction::Saload => write!(f, "saload"),
            Instruction::Istore(value) => write!(f, "istore {value}"),
            Instruction::Lstore(value) => write!(f, "lstore {value}"),
            Instruction::Fstore(value) => write!(f, "fstore {value}"),
            Instruction::Dstore(value) => write!(f, "dstore {value}"),
            Instruction::Astore(value) => write!(f, "astore {value}"),
            Instruction::Istore_0 => write!(f, "istore_0"),
            Instruction::Istore_1 => write!(f, "istore_1"),
            Instruction::Istore_2 => write!(f, "istore_2"),
            Instruction::Istore_3 => write!(f, "istore_3"),
            Instruction::Lstore_0 => write!(f, "lstore_0"),
            Instruction::Lstore_1 => write!(f, "lstore_1"),
            Instruction::Lstore_2 => write!(f, "lstore_2"),
            Instruction::Lstore_3 => write!(f, "lstore_3"),
            Instruction::Fstore_0 => write!(f, "fstore_0"),
            Instruction::Fstore_1 => write!(f, "fstore_1"),
            Instruction::Fstore_2 => write!(f, "fstore_2"),
            Instruction::Fstore_3 => write!(f, "fstore_3"),
            Instruction::Dstore_0 => write!(f, "dstore_0"),
            Instruction::Dstore_1 => write!(f, "dstore_1"),
            Instruction::Dstore_2 => write!(f, "dstore_2"),
            Instruction::Dstore_3 => write!(f, "dstore_3"),
            Instruction::Astore_0 => write!(f, "astore_0"),
            Instruction::Astore_1 => write!(f, "astore_1"),
            Instruction::Astore_2 => write!(f, "astore_2"),
            Instruction::Astore_3 => write!(f, "astore_3"),
            Instruction::Iastore => write!(f, "iastore"),
            Instruction::Lastore => write!(f, "lastore"),
            Instruction::Fastore => write!(f, "fastore"),
            Instruction::Dastore => write!(f, "dastore"),
            Instruction::Aastore => write!(f, "aastore"),
            Instruction::Bastore => write!(f, "bastore"),
            Instruction::Castore => write!(f, "castore"),
            Instruction::Sastore => write!(f, "sastore"),
            Instruction::Pop => write!(f, "pop"),
            Instruction::Pop2 => write!(f, "pop2"),
            Instruction::Dup => write!(f, "dup"),
            Instruction::Dup_x1 => write!(f, "dup_x1"),
            Instruction::Dup_x2 => write!(f, "dup_x2"),
            Instruction::Dup2 => write!(f, "dup2"),
            Instruction::Dup2_x1 => write!(f, "dup2_x1"),
            Instruction::Dup2_x2 => write!(f, "dup2_x2"),
            Instruction::Swap => write!(f, "swap"),
            Instruction::Iadd => write!(f, "iadd"),
            Instruction::Ladd => write!(f, "ladd"),
            Instruction::Fadd => write!(f, "fadd"),
            Instruction::Dadd => write!(f, "dadd"),
            Instruction::Isub => write!(f, "isub"),
            Instruction::Lsub => write!(f, "lsub"),
            Instruction::Fsub => write!(f, "fsub"),
            Instruction::Dsub => write!(f, "dsub"),
            Instruction::Imul => write!(f, "imul"),
            Instruction::Lmul => write!(f, "lmul"),
            Instruction::Fmul => write!(f, "fmul"),
            Instruction::Dmul => write!(f, "dmul"),
            Instruction::Idiv => write!(f, "idiv"),
            Instruction::Ldiv => write!(f, "ldiv"),
            Instruction::Fdiv => write!(f, "fdiv"),
            Instruction::Ddiv => write!(f, "ddiv"),
            Instruction::Irem => write!(f, "irem"),
            Instruction::Lrem => write!(f, "lrem"),
            Instruction::Frem => write!(f, "frem"),
            Instruction::Drem => write!(f, "drem"),
            Instruction::Ineg => write!(f, "ineg"),
            Instruction::Lneg => write!(f, "lneg"),
            Instruction::Fneg => write!(f, "fneg"),
            Instruction::Dneg => write!(f, "dneg"),
            Instruction::Ishl => write!(f, "ishl"),
            Instruction::Lshl => write!(f, "lshl"),
            Instruction::Ishr => write!(f, "ishr"),
            Instruction::Lshr => write!(f, "lshr"),
            Instruction::Iushr => write!(f, "iushr"),
            Instruction::Lushr => write!(f, "lushr"),
            Instruction::Iand => write!(f, "iand"),
            Instruction::Land => write!(f, "land"),
            Instruction::Ior => write!(f, "ior"),
            Instruction::Lor => write!(f, "lor"),
            Instruction::Ixor => write!(f, "ixor"),
            Instruction::Lxor => write!(f, "lxor"),
            Instruction::Iinc(value1, value2) => write!(f, "iinc {value1}, {value2}"),
            Instruction::I2l => write!(f, "i2l"),
            Instruction::I2f => write!(f, "i2f"),
            Instruction::I2d => write!(f, "i2d"),
            Instruction::L2i => write!(f, "l2i"),
            Instruction::L2f => write!(f, "l2f"),
            Instruction::L2d => write!(f, "l2d"),
            Instruction::F2i => write!(f, "f2i"),
            Instruction::F2l => write!(f, "f2l"),
            Instruction::F2d => write!(f, "f2d"),
            Instruction::D2i => write!(f, "d2i"),
            Instruction::D2l => write!(f, "d2l"),
            Instruction::D2f => write!(f, "d2f"),
            Instruction::I2b => write!(f, "i2b"),
            Instruction::I2c => write!(f, "i2c"),
            Instruction::I2s => write!(f, "i2s"),
            Instruction::Lcmp => write!(f, "lcmp"),
            Instruction::Fcmpl => write!(f, "fcmpl"),
            Instruction::Fcmpg => write!(f, "fcmpg"),
            Instruction::Dcmpl => write!(f, "dcmpl"),
            Instruction::Dcmpg => write!(f, "dcmpg"),
            Instruction::Ifeq(value) => write!(f, "ifeq {value}"),
            Instruction::Ifne(value) => write!(f, "ifne {value}"),
            Instruction::Iflt(value) => write!(f, "iflt {value}"),
            Instruction::Ifge(value) => write!(f, "ifge {value}"),
            Instruction::Ifgt(value) => write!(f, "ifgt {value}"),
            Instruction::Ifle(value) => write!(f, "ifle {value}"),
            Instruction::If_icmpeq(value) => write!(f, "if_icmpeq {value}"),
            Instruction::If_icmpne(value) => write!(f, "if_icmpne {value}"),
            Instruction::If_icmplt(value) => write!(f, "if_icmplt {value}"),
            Instruction::If_icmpge(value) => write!(f, "if_icmpge {value}"),
            Instruction::If_icmpgt(value) => write!(f, "if_icmpgt {value}"),
            Instruction::If_icmple(value) => write!(f, "if_icmple {value}"),
            Instruction::If_acmpeq(value) => write!(f, "if_acmpeq {value}"),
            Instruction::If_acmpne(value) => write!(f, "if_acmpne {value}"),
            Instruction::Goto(value) => write!(f, "goto {value}"),
            Instruction::Jsr(value) => write!(f, "jsr {value}"),
            Instruction::Ret(value) => write!(f, "ret {value}"),
            Instruction::Tableswitch {
                default,
                low,
                high,
                offsets,
            } => {
                // tableswitch   { // 3 to 5
                //            3: 52
                //            4: 62
                //            5: 72
                //      default: 82
                // }
                let width = 12;
                writeln!(f, "tableswitch   {{ // {low} to {high}")?;
                for (i, offset) in offsets.iter().enumerate() {
                    let value = low + i32::try_from(i).map_err(|_| fmt::Error)?;
                    writeln!(f, "{value:>width$}: {offset}")?;
                }
                writeln!(f, "{:>width$}: {default}", "default")?;
                write!(f, "}}")
            }
            Instruction::Lookupswitch { pairs, default } => {
                let first_pair = pairs.first().unwrap_or(&(0, 0));
                let (low, _) = first_pair;
                let width = 12;
                writeln!(f, "lookupswitch   {{ // {low}")?;
                for pair in pairs {
                    let (value, offset) = pair;
                    writeln!(f, "{value:>width$}: {offset}")?;
                }
                writeln!(f, "{:>width$}: {default}", "default")?;
                write!(f, "}}")
            }
            Instruction::Ireturn => write!(f, "ireturn"),
            Instruction::Lreturn => write!(f, "lreturn"),
            Instruction::Freturn => write!(f, "freturn"),
            Instruction::Dreturn => write!(f, "dreturn"),
            Instruction::Areturn => write!(f, "areturn"),
            Instruction::Return => write!(f, "return"),
            Instruction::Getstatic(value) => write!(f, "getstatic #{value}"),
            Instruction::Putstatic(value) => write!(f, "putstatic #{value}"),
            Instruction::Getfield(value) => write!(f, "getfield #{value}"),
            Instruction::Putfield(value) => write!(f, "putfield #{value}"),
            Instruction::Invokevirtual(value) => write!(f, "invokevirtual #{value}"),
            Instruction::Invokespecial(value) => write!(f, "invokespecial #{value}"),
            Instruction::Invokestatic(value) => write!(f, "invokestatic #{value}"),
            Instruction::Invokeinterface(value, count) => {
                write!(f, "invokeinterface #{value}, {count}")
            }
            Instruction::Invokedynamic(value) => write!(f, "invokedynamic #{value}"),
            Instruction::New(value) => write!(f, "new #{value}"),
            Instruction::Newarray(value) => write!(f, "newarray {value}"),
            Instruction::Anewarray(value) => write!(f, "anewarray #{value}"),
            Instruction::Arraylength => write!(f, "arraylength"),
            Instruction::Athrow => write!(f, "athrow"),
            Instruction::Checkcast(value) => write!(f, "checkcast #{value}"),
            Instruction::Instanceof(value) => write!(f, "instanceof #{value}"),
            Instruction::Monitorenter => write!(f, "monitorenter"),
            Instruction::Monitorexit => write!(f, "monitorexit"),
            Instruction::Wide => write!(f, "wide"),
            Instruction::Multianewarray(value, dim) => write!(f, "multianewarray #{value}, {dim}"),
            Instruction::Ifnull(value) => write!(f, "ifnull {value}"),
            Instruction::Ifnonnull(value) => write!(f, "ifnonnull {value}"),
            Instruction::Goto_w(value) => write!(f, "goto_w {value}"),
            Instruction::Jsr_w(value) => write!(f, "jsr_w {value}"),
            Instruction::Breakpoint => write!(f, "breakpoint"),
            // Wide instructions
            Instruction::Iload_w(value) => write!(f, "iload_w {value}"),
            Instruction::Lload_w(value) => write!(f, "lload_w {value}"),
            Instruction::Fload_w(value) => write!(f, "fload_w {value}"),
            Instruction::Dload_w(value) => write!(f, "dload_w {value}"),
            Instruction::Aload_w(value) => write!(f, "aload_w {value}"),
            Instruction::Istore_w(value) => write!(f, "istore_w {value}"),
            Instruction::Lstore_w(value) => write!(f, "lstore_w {value}"),
            Instruction::Fstore_w(value) => write!(f, "fstore_w {value}"),
            Instruction::Dstore_w(value) => write!(f, "dstore_w {value}"),
            Instruction::Astore_w(value) => write!(f, "astore_w {value}"),
            Instruction::Iinc_w(value1, value2) => {
                write!(f, "iinc_w {value1}, {value2}")
            }
            Instruction::Ret_w(value) => write!(f, "ret_w {value}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use std::io::Read;

    #[test]
    fn test_invalid_instruction() -> Result<()> {
        let mut bytes = Vec::new();
        let code = u8::MAX;
        bytes.write_u8(code)?;

        assert_eq!(
            Err(InvalidInstruction(code)),
            Instruction::from_bytes(&mut Cursor::new(bytes))
        );
        Ok(())
    }

    fn test_instruction(instruction: &Instruction, expected_bytes: &[u8], code: u8) -> Result<()> {
        assert_eq!(code, instruction.code());

        let mut buffer = Cursor::new(Vec::new());
        instruction.to_bytes(&mut buffer)?;
        let mut bytes = Vec::new();
        buffer.set_position(0);
        buffer.read_to_end(&mut bytes)?;
        let mut bytes = Cursor::new(expected_bytes.to_vec());
        assert_eq!(*instruction, Instruction::from_bytes(&mut bytes)?);
        Ok(())
    }

    #[test]
    fn test_nop() -> Result<()> {
        let instruction = Instruction::Nop;
        let code = 0;
        let expected_bytes = [code];

        assert_eq!("nop", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_aconst_null() -> Result<()> {
        let instruction = Instruction::Aconst_null;
        let code = 1;
        let expected_bytes = [code];

        assert_eq!("aconst_null", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iconst_m1() -> Result<()> {
        let instruction = Instruction::Iconst_m1;
        let code = 2;
        let expected_bytes = [code];

        assert_eq!("iconst_m1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iconst_0() -> Result<()> {
        let instruction = Instruction::Iconst_0;
        let code = 3;
        let expected_bytes = [code];

        assert_eq!("iconst_0", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iconst_1() -> Result<()> {
        let instruction = Instruction::Iconst_1;
        let code = 4;
        let expected_bytes = [code];

        assert_eq!("iconst_1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iconst_2() -> Result<()> {
        let instruction = Instruction::Iconst_2;
        let code = 5;
        let expected_bytes = [code];

        assert_eq!("iconst_2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iconst_3() -> Result<()> {
        let instruction = Instruction::Iconst_3;
        let code = 6;
        let expected_bytes = [code];

        assert_eq!("iconst_3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iconst_4() -> Result<()> {
        let instruction = Instruction::Iconst_4;
        let code = 7;
        let expected_bytes = [code];

        assert_eq!("iconst_4", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iconst_5() -> Result<()> {
        let instruction = Instruction::Iconst_5;
        let code = 8;
        let expected_bytes = [code];

        assert_eq!("iconst_5", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lconst_0() -> Result<()> {
        let instruction = Instruction::Lconst_0;
        let code = 9;
        let expected_bytes = [code];

        assert_eq!("lconst_0", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lconst_1() -> Result<()> {
        let instruction = Instruction::Lconst_1;
        let code = 10;
        let expected_bytes = [code];

        assert_eq!("lconst_1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fconst_0() -> Result<()> {
        let instruction = Instruction::Fconst_0;
        let code = 11;
        let expected_bytes = [code];

        assert_eq!("fconst_0", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fconst_1() -> Result<()> {
        let instruction = Instruction::Fconst_1;
        let code = 12;
        let expected_bytes = [code];

        assert_eq!("fconst_1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fconst_2() -> Result<()> {
        let instruction = Instruction::Fconst_2;
        let code = 13;
        let expected_bytes = [code];

        assert_eq!("fconst_2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dconst_0() -> Result<()> {
        let instruction = Instruction::Dconst_0;
        let code = 14;
        let expected_bytes = [code];

        assert_eq!("dconst_0", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dconst_1() -> Result<()> {
        let instruction = Instruction::Dconst_1;
        let code = 15;
        let expected_bytes = [code];

        assert_eq!("dconst_1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_bipush() -> Result<()> {
        let instruction = Instruction::Bipush(42);
        let code = 16;
        let expected_bytes = [code, 42];

        assert_eq!("bipush 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_sipush() -> Result<()> {
        let instruction = Instruction::Sipush(42);
        let code = 17;
        let expected_bytes = [code, 0, 42];

        assert_eq!("sipush 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ldc() -> Result<()> {
        let instruction = Instruction::Ldc(42);
        let code = 18;
        let expected_bytes = [code, 42];

        assert_eq!("ldc #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ldc_w() -> Result<()> {
        let instruction = Instruction::Ldc_w(42);
        let code = 19;
        let expected_bytes = [code, 0, 42];

        assert_eq!("ldc_w #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ldc2_w() -> Result<()> {
        let instruction = Instruction::Ldc2_w(42);
        let code = 20;
        let expected_bytes = [code, 0, 42];

        assert_eq!("ldc2_w #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iload() -> Result<()> {
        let instruction = Instruction::Iload(42);
        let code = 21;
        let expected_bytes = [code, 42];

        assert_eq!("iload 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lload() -> Result<()> {
        let instruction = Instruction::Lload(42);
        let code = 22;
        let expected_bytes = [code, 42];

        assert_eq!("lload 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fload() -> Result<()> {
        let instruction = Instruction::Fload(42);
        let code = 23;
        let expected_bytes = [code, 42];

        assert_eq!("fload 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dload() -> Result<()> {
        let instruction = Instruction::Dload(42);
        let code = 24;
        let expected_bytes = [code, 42];

        assert_eq!("dload 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_aload() -> Result<()> {
        let instruction = Instruction::Aload(42);
        let code = 25;
        let expected_bytes = [code, 42];

        assert_eq!("aload 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iload_0() -> Result<()> {
        let instruction = Instruction::Iload_0;
        let code = 26;
        let expected_bytes = [code];

        assert_eq!("iload_0", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iload_1() -> Result<()> {
        let instruction = Instruction::Iload_1;
        let code = 27;
        let expected_bytes = [code];

        assert_eq!("iload_1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iload_2() -> Result<()> {
        let instruction = Instruction::Iload_2;
        let code = 28;
        let expected_bytes = [code];

        assert_eq!("iload_2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iload_3() -> Result<()> {
        let instruction = Instruction::Iload_3;
        let code = 29;
        let expected_bytes = [code];

        assert_eq!("iload_3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lload_0() -> Result<()> {
        let instruction = Instruction::Lload_0;
        let code = 30;
        let expected_bytes = [code];

        assert_eq!("lload_0", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lload_1() -> Result<()> {
        let instruction = Instruction::Lload_1;
        let code = 31;
        let expected_bytes = [code];

        assert_eq!("lload_1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lload_2() -> Result<()> {
        let instruction = Instruction::Lload_2;
        let code = 32;
        let expected_bytes = [code];

        assert_eq!("lload_2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lload_3() -> Result<()> {
        let instruction = Instruction::Lload_3;
        let code = 33;
        let expected_bytes = [code];

        assert_eq!("lload_3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fload_0() -> Result<()> {
        let instruction = Instruction::Fload_0;
        let code = 34;
        let expected_bytes = [code];

        assert_eq!("fload_0", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fload_1() -> Result<()> {
        let instruction = Instruction::Fload_1;
        let code = 35;
        let expected_bytes = [code];

        assert_eq!("fload_1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fload_2() -> Result<()> {
        let instruction = Instruction::Fload_2;
        let code = 36;
        let expected_bytes = [code];

        assert_eq!("fload_2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fload_3() -> Result<()> {
        let instruction = Instruction::Fload_3;
        let code = 37;
        let expected_bytes = [code];

        assert_eq!("fload_3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dload_0() -> Result<()> {
        let instruction = Instruction::Dload_0;
        let code = 38;
        let expected_bytes = [code];

        assert_eq!("dload_0", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dload_1() -> Result<()> {
        let instruction = Instruction::Dload_1;
        let code = 39;
        let expected_bytes = [code];

        assert_eq!("dload_1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dload_2() -> Result<()> {
        let instruction = Instruction::Dload_2;
        let code = 40;
        let expected_bytes = [code];

        assert_eq!("dload_2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dload_3() -> Result<()> {
        let instruction = Instruction::Dload_3;
        let code = 41;
        let expected_bytes = [code];

        assert_eq!("dload_3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_aload_0() -> Result<()> {
        let instruction = Instruction::Aload_0;
        let code = 42;
        let expected_bytes = [code];

        assert_eq!("aload_0", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_aload_1() -> Result<()> {
        let instruction = Instruction::Aload_1;
        let code = 43;
        let expected_bytes = [code];

        assert_eq!("aload_1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_aload_2() -> Result<()> {
        let instruction = Instruction::Aload_2;
        let code = 44;
        let expected_bytes = [code];

        assert_eq!("aload_2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_aload_3() -> Result<()> {
        let instruction = Instruction::Aload_3;
        let code = 45;
        let expected_bytes = [code];

        assert_eq!("aload_3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iaload() -> Result<()> {
        let instruction = Instruction::Iaload;
        let code = 46;
        let expected_bytes = [code];

        assert_eq!("iaload", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_laload() -> Result<()> {
        let instruction = Instruction::Laload;
        let code = 47;
        let expected_bytes = [code];

        assert_eq!("laload", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_faload() -> Result<()> {
        let instruction = Instruction::Faload;
        let code = 48;
        let expected_bytes = [code];

        assert_eq!("faload", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_daload() -> Result<()> {
        let instruction = Instruction::Daload;
        let code = 49;
        let expected_bytes = [code];

        assert_eq!("daload", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_aaload() -> Result<()> {
        let instruction = Instruction::Aaload;
        let code = 50;
        let expected_bytes = [code];

        assert_eq!("aaload", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_baload() -> Result<()> {
        let instruction = Instruction::Baload;
        let code = 51;
        let expected_bytes = [code];

        assert_eq!("baload", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_caload() -> Result<()> {
        let instruction = Instruction::Caload;
        let code = 52;
        let expected_bytes = [code];

        assert_eq!("caload", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_saload() -> Result<()> {
        let instruction = Instruction::Saload;
        let code = 53;
        let expected_bytes = [code];

        assert_eq!("saload", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_istore() -> Result<()> {
        let instruction = Instruction::Istore(42);
        let code = 54;
        let expected_bytes = [code, 42];

        assert_eq!("istore 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lstore() -> Result<()> {
        let instruction = Instruction::Lstore(42);
        let code = 55;
        let expected_bytes = [code, 42];

        assert_eq!("lstore 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fstore() -> Result<()> {
        let instruction = Instruction::Fstore(42);
        let code = 56;
        let expected_bytes = [code, 42];

        assert_eq!("fstore 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dstore() -> Result<()> {
        let instruction = Instruction::Dstore(42);
        let code = 57;
        let expected_bytes = [code, 42];

        assert_eq!("dstore 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_astore() -> Result<()> {
        let instruction = Instruction::Astore(42);
        let code = 58;
        let expected_bytes = [code, 42];

        assert_eq!("astore 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_istore_0() -> Result<()> {
        let instruction = Instruction::Istore_0;
        let code = 59;
        let expected_bytes = [code];

        assert_eq!("istore_0", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_istore_1() -> Result<()> {
        let instruction = Instruction::Istore_1;
        let code = 60;
        let expected_bytes = [code];

        assert_eq!("istore_1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_istore_2() -> Result<()> {
        let instruction = Instruction::Istore_2;
        let code = 61;
        let expected_bytes = [code];

        assert_eq!("istore_2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_istore_3() -> Result<()> {
        let instruction = Instruction::Istore_3;
        let code = 62;
        let expected_bytes = [code];

        assert_eq!("istore_3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lstore_0() -> Result<()> {
        let instruction = Instruction::Lstore_0;
        let code = 63;
        let expected_bytes = [code];

        assert_eq!("lstore_0", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lstore_1() -> Result<()> {
        let instruction = Instruction::Lstore_1;
        let code = 64;
        let expected_bytes = [code];

        assert_eq!("lstore_1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lstore_2() -> Result<()> {
        let instruction = Instruction::Lstore_2;
        let code = 65;
        let expected_bytes = [code];

        assert_eq!("lstore_2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lstore_3() -> Result<()> {
        let instruction = Instruction::Lstore_3;
        let code = 66;
        let expected_bytes = [code];

        assert_eq!("lstore_3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fstore_0() -> Result<()> {
        let instruction = Instruction::Fstore_0;
        let code = 67;
        let expected_bytes = [code];

        assert_eq!("fstore_0", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fstore_1() -> Result<()> {
        let instruction = Instruction::Fstore_1;
        let code = 68;
        let expected_bytes = [code];

        assert_eq!("fstore_1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fstore_2() -> Result<()> {
        let instruction = Instruction::Fstore_2;
        let code = 69;
        let expected_bytes = [code];

        assert_eq!("fstore_2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fstore_3() -> Result<()> {
        let instruction = Instruction::Fstore_3;
        let code = 70;
        let expected_bytes = [code];

        assert_eq!("fstore_3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dstore_0() -> Result<()> {
        let instruction = Instruction::Dstore_0;
        let code = 71;
        let expected_bytes = [code];

        assert_eq!("dstore_0", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dstore_1() -> Result<()> {
        let instruction = Instruction::Dstore_1;
        let code = 72;
        let expected_bytes = [code];

        assert_eq!("dstore_1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dstore_2() -> Result<()> {
        let instruction = Instruction::Dstore_2;
        let code = 73;
        let expected_bytes = [code];

        assert_eq!("dstore_2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dstore_3() -> Result<()> {
        let instruction = Instruction::Dstore_3;
        let code = 74;
        let expected_bytes = [code];

        assert_eq!("dstore_3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_astore_0() -> Result<()> {
        let instruction = Instruction::Astore_0;
        let code = 75;
        let expected_bytes = [code];

        assert_eq!("astore_0", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_astore_1() -> Result<()> {
        let instruction = Instruction::Astore_1;
        let code = 76;
        let expected_bytes = [code];

        assert_eq!("astore_1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_astore_2() -> Result<()> {
        let instruction = Instruction::Astore_2;
        let code = 77;
        let expected_bytes = [code];

        assert_eq!("astore_2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_astore_3() -> Result<()> {
        let instruction = Instruction::Astore_3;
        let code = 78;
        let expected_bytes = [code];

        assert_eq!("astore_3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iastore() -> Result<()> {
        let instruction = Instruction::Iastore;
        let code = 79;
        let expected_bytes = [code];

        assert_eq!("iastore", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lastore() -> Result<()> {
        let instruction = Instruction::Lastore;
        let code = 80;
        let expected_bytes = [code];

        assert_eq!("lastore", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fastore() -> Result<()> {
        let instruction = Instruction::Fastore;
        let code = 81;
        let expected_bytes = [code];

        assert_eq!("fastore", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dastore() -> Result<()> {
        let instruction = Instruction::Dastore;
        let code = 82;
        let expected_bytes = [code];

        assert_eq!("dastore", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_aastore() -> Result<()> {
        let instruction = Instruction::Aastore;
        let code = 83;
        let expected_bytes = [code];

        assert_eq!("aastore", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_bastore() -> Result<()> {
        let instruction = Instruction::Bastore;
        let code = 84;
        let expected_bytes = [code];

        assert_eq!("bastore", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_castore() -> Result<()> {
        let instruction = Instruction::Castore;
        let code = 85;
        let expected_bytes = [code];

        assert_eq!("castore", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_sastore() -> Result<()> {
        let instruction = Instruction::Sastore;
        let code = 86;
        let expected_bytes = [code];

        assert_eq!("sastore", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_pop() -> Result<()> {
        let instruction = Instruction::Pop;
        let code = 87;
        let expected_bytes = [code];

        assert_eq!("pop", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_pop2() -> Result<()> {
        let instruction = Instruction::Pop2;
        let code = 88;
        let expected_bytes = [code];

        assert_eq!("pop2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dup() -> Result<()> {
        let instruction = Instruction::Dup;
        let code = 89;
        let expected_bytes = [code];

        assert_eq!("dup", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dup_x1() -> Result<()> {
        let instruction = Instruction::Dup_x1;
        let code = 90;
        let expected_bytes = [code];

        assert_eq!("dup_x1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dup_x2() -> Result<()> {
        let instruction = Instruction::Dup_x2;
        let code = 91;
        let expected_bytes = [code];

        assert_eq!("dup_x2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dup2() -> Result<()> {
        let instruction = Instruction::Dup2;
        let code = 92;
        let expected_bytes = [code];

        assert_eq!("dup2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dup2_x1() -> Result<()> {
        let instruction = Instruction::Dup2_x1;
        let code = 93;
        let expected_bytes = [code];

        assert_eq!("dup2_x1", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dup2_x2() -> Result<()> {
        let instruction = Instruction::Dup2_x2;
        let code = 94;
        let expected_bytes = [code];

        assert_eq!("dup2_x2", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_swap() -> Result<()> {
        let instruction = Instruction::Swap;
        let code = 95;
        let expected_bytes = [code];

        assert_eq!("swap", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iadd() -> Result<()> {
        let instruction = Instruction::Iadd;
        let code = 96;
        let expected_bytes = [code];

        assert_eq!("iadd", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ladd() -> Result<()> {
        let instruction = Instruction::Ladd;
        let code = 97;
        let expected_bytes = [code];

        assert_eq!("ladd", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fadd() -> Result<()> {
        let instruction = Instruction::Fadd;
        let code = 98;
        let expected_bytes = [code];

        assert_eq!("fadd", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dadd() -> Result<()> {
        let instruction = Instruction::Dadd;
        let code = 99;
        let expected_bytes = [code];

        assert_eq!("dadd", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_isub() -> Result<()> {
        let instruction = Instruction::Isub;
        let code = 100;
        let expected_bytes = [code];

        assert_eq!("isub", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lsub() -> Result<()> {
        let instruction = Instruction::Lsub;
        let code = 101;
        let expected_bytes = [code];

        assert_eq!("lsub", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fsub() -> Result<()> {
        let instruction = Instruction::Fsub;
        let code = 102;
        let expected_bytes = [code];

        assert_eq!("fsub", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dsub() -> Result<()> {
        let instruction = Instruction::Dsub;
        let code = 103;
        let expected_bytes = [code];

        assert_eq!("dsub", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_imul() -> Result<()> {
        let instruction = Instruction::Imul;
        let code = 104;
        let expected_bytes = [code];

        assert_eq!("imul", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lmul() -> Result<()> {
        let instruction = Instruction::Lmul;
        let code = 105;
        let expected_bytes = [code];

        assert_eq!("lmul", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fmul() -> Result<()> {
        let instruction = Instruction::Fmul;
        let code = 106;
        let expected_bytes = [code];

        assert_eq!("fmul", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dmul() -> Result<()> {
        let instruction = Instruction::Dmul;
        let code = 107;
        let expected_bytes = [code];

        assert_eq!("dmul", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_idiv() -> Result<()> {
        let instruction = Instruction::Idiv;
        let code = 108;
        let expected_bytes = [code];

        assert_eq!("idiv", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ldiv() -> Result<()> {
        let instruction = Instruction::Ldiv;
        let code = 109;
        let expected_bytes = [code];

        assert_eq!("ldiv", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fdiv() -> Result<()> {
        let instruction = Instruction::Fdiv;
        let code = 110;
        let expected_bytes = [code];

        assert_eq!("fdiv", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ddiv() -> Result<()> {
        let instruction = Instruction::Ddiv;
        let code = 111;
        let expected_bytes = [code];

        assert_eq!("ddiv", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_irem() -> Result<()> {
        let instruction = Instruction::Irem;
        let code = 112;
        let expected_bytes = [code];

        assert_eq!("irem", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lrem() -> Result<()> {
        let instruction = Instruction::Lrem;
        let code = 113;
        let expected_bytes = [code];

        assert_eq!("lrem", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_frem() -> Result<()> {
        let instruction = Instruction::Frem;
        let code = 114;
        let expected_bytes = [code];

        assert_eq!("frem", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_drem() -> Result<()> {
        let instruction = Instruction::Drem;
        let code = 115;
        let expected_bytes = [code];

        assert_eq!("drem", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ineg() -> Result<()> {
        let instruction = Instruction::Ineg;
        let code = 116;
        let expected_bytes = [code];

        assert_eq!("ineg", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lneg() -> Result<()> {
        let instruction = Instruction::Lneg;
        let code = 117;
        let expected_bytes = [code];

        assert_eq!("lneg", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fneg() -> Result<()> {
        let instruction = Instruction::Fneg;
        let code = 118;
        let expected_bytes = [code];

        assert_eq!("fneg", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dneg() -> Result<()> {
        let instruction = Instruction::Dneg;
        let code = 119;
        let expected_bytes = [code];

        assert_eq!("dneg", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ishl() -> Result<()> {
        let instruction = Instruction::Ishl;
        let code = 120;
        let expected_bytes = [code];

        assert_eq!("ishl", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lshl() -> Result<()> {
        let instruction = Instruction::Lshl;
        let code = 121;
        let expected_bytes = [code];

        assert_eq!("lshl", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ishr() -> Result<()> {
        let instruction = Instruction::Ishr;
        let code = 122;
        let expected_bytes = [code];

        assert_eq!("ishr", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lshr() -> Result<()> {
        let instruction = Instruction::Lshr;
        let code = 123;
        let expected_bytes = [code];

        assert_eq!("lshr", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iushr() -> Result<()> {
        let instruction = Instruction::Iushr;
        let code = 124;
        let expected_bytes = [code];

        assert_eq!("iushr", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lushr() -> Result<()> {
        let instruction = Instruction::Lushr;
        let code = 125;
        let expected_bytes = [code];

        assert_eq!("lushr", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iand() -> Result<()> {
        let instruction = Instruction::Iand;
        let code = 126;
        let expected_bytes = [code];

        assert_eq!("iand", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_land() -> Result<()> {
        let instruction = Instruction::Land;
        let code = 127;
        let expected_bytes = [code];

        assert_eq!("land", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ior() -> Result<()> {
        let instruction = Instruction::Ior;
        let code = 128;
        let expected_bytes = [code];

        assert_eq!("ior", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lor() -> Result<()> {
        let instruction = Instruction::Lor;
        let code = 129;
        let expected_bytes = [code];

        assert_eq!("lor", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ixor() -> Result<()> {
        let instruction = Instruction::Ixor;
        let code = 130;
        let expected_bytes = [code];

        assert_eq!("ixor", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lxor() -> Result<()> {
        let instruction = Instruction::Lxor;
        let code = 131;
        let expected_bytes = [code];

        assert_eq!("lxor", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iinc() -> Result<()> {
        let instruction = Instruction::Iinc(42, 3);
        let code = 132;
        let expected_bytes = [code, 42, 3];

        assert_eq!("iinc 42, 3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_i2l() -> Result<()> {
        let instruction = Instruction::I2l;
        let code = 133;
        let expected_bytes = [code];

        assert_eq!("i2l", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_i2f() -> Result<()> {
        let instruction = Instruction::I2f;
        let code = 134;
        let expected_bytes = [code];

        assert_eq!("i2f", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_i2d() -> Result<()> {
        let instruction = Instruction::I2d;
        let code = 135;
        let expected_bytes = [code];

        assert_eq!("i2d", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_l2i() -> Result<()> {
        let instruction = Instruction::L2i;
        let code = 136;
        let expected_bytes = [code];

        assert_eq!("l2i", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_l2f() -> Result<()> {
        let instruction = Instruction::L2f;
        let code = 137;
        let expected_bytes = [code];

        assert_eq!("l2f", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_l2d() -> Result<()> {
        let instruction = Instruction::L2d;
        let code = 138;
        let expected_bytes = [code];

        assert_eq!("l2d", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_f2i() -> Result<()> {
        let instruction = Instruction::F2i;
        let code = 139;
        let expected_bytes = [code];

        assert_eq!("f2i", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_f2l() -> Result<()> {
        let instruction = Instruction::F2l;
        let code = 140;
        let expected_bytes = [code];

        assert_eq!("f2l", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_f2d() -> Result<()> {
        let instruction = Instruction::F2d;
        let code = 141;
        let expected_bytes = [code];

        assert_eq!("f2d", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_d2i() -> Result<()> {
        let instruction = Instruction::D2i;
        let code = 142;
        let expected_bytes = [code];

        assert_eq!("d2i", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_d2l() -> Result<()> {
        let instruction = Instruction::D2l;
        let code = 143;
        let expected_bytes = [code];

        assert_eq!("d2l", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_d2f() -> Result<()> {
        let instruction = Instruction::D2f;
        let code = 144;
        let expected_bytes = [code];

        assert_eq!("d2f", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_i2b() -> Result<()> {
        let instruction = Instruction::I2b;
        let code = 145;
        let expected_bytes = [code];

        assert_eq!("i2b", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_i2c() -> Result<()> {
        let instruction = Instruction::I2c;
        let code = 146;
        let expected_bytes = [code];

        assert_eq!("i2c", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_i2s() -> Result<()> {
        let instruction = Instruction::I2s;
        let code = 147;
        let expected_bytes = [code];

        assert_eq!("i2s", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lcmp() -> Result<()> {
        let instruction = Instruction::Lcmp;
        let code = 148;
        let expected_bytes = [code];

        assert_eq!("lcmp", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fcmpl() -> Result<()> {
        let instruction = Instruction::Fcmpl;
        let code = 149;
        let expected_bytes = [code];

        assert_eq!("fcmpl", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_fcmpg() -> Result<()> {
        let instruction = Instruction::Fcmpg;
        let code = 150;
        let expected_bytes = [code];

        assert_eq!("fcmpg", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dcmpl() -> Result<()> {
        let instruction = Instruction::Dcmpl;
        let code = 151;
        let expected_bytes = [code];

        assert_eq!("dcmpl", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dcmpg() -> Result<()> {
        let instruction = Instruction::Dcmpg;
        let code = 152;
        let expected_bytes = [code];

        assert_eq!("dcmpg", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ifeq() -> Result<()> {
        let instruction = Instruction::Ifeq(42);
        let code = 153;
        let expected_bytes = [code, 0, 42];

        assert_eq!("ifeq 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ifne() -> Result<()> {
        let instruction = Instruction::Ifne(42);
        let code = 154;
        let expected_bytes = [code, 0, 42];

        assert_eq!("ifne 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_iflt() -> Result<()> {
        let instruction = Instruction::Iflt(42);
        let code = 155;
        let expected_bytes = [code, 0, 42];

        assert_eq!("iflt 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ifge() -> Result<()> {
        let instruction = Instruction::Ifge(42);
        let code = 156;
        let expected_bytes = [code, 0, 42];

        assert_eq!("ifge 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ifgt() -> Result<()> {
        let instruction = Instruction::Ifgt(42);
        let code = 157;
        let expected_bytes = [code, 0, 42];

        assert_eq!("ifgt 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ifle() -> Result<()> {
        let instruction = Instruction::Ifle(42);
        let code = 158;
        let expected_bytes = [code, 0, 42];

        assert_eq!("ifle 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_if_icmpeq() -> Result<()> {
        let instruction = Instruction::If_icmpeq(42);
        let code = 159;
        let expected_bytes = [code, 0, 42];

        assert_eq!("if_icmpeq 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_if_icmpne() -> Result<()> {
        let instruction = Instruction::If_icmpne(42);
        let code = 160;
        let expected_bytes = [code, 0, 42];

        assert_eq!("if_icmpne 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_if_icmplt() -> Result<()> {
        let instruction = Instruction::If_icmplt(42);
        let code = 161;
        let expected_bytes = [code, 0, 42];

        assert_eq!("if_icmplt 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_if_icmpge() -> Result<()> {
        let instruction = Instruction::If_icmpge(42);
        let code = 162;
        let expected_bytes = [code, 0, 42];

        assert_eq!("if_icmpge 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_if_icmpgt() -> Result<()> {
        let instruction = Instruction::If_icmpgt(42);
        let code = 163;
        let expected_bytes = [code, 0, 42];

        assert_eq!("if_icmpgt 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_if_icmple() -> Result<()> {
        let instruction = Instruction::If_icmple(42);
        let code = 164;
        let expected_bytes = [code, 0, 42];

        assert_eq!("if_icmple 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_if_acmpeq() -> Result<()> {
        let instruction = Instruction::If_acmpeq(42);
        let code = 165;
        let expected_bytes = [code, 0, 42];

        assert_eq!("if_acmpeq 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_if_acmpne() -> Result<()> {
        let instruction = Instruction::If_acmpne(42);
        let code = 166;
        let expected_bytes = [code, 0, 42];

        assert_eq!("if_acmpne 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_goto() -> Result<()> {
        let instruction = Instruction::Goto(42);
        let code = 167;
        let expected_bytes = [code, 0, 42];

        assert_eq!("goto 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_jsr() -> Result<()> {
        let instruction = Instruction::Jsr(42);
        let code = 168;
        let expected_bytes = [code, 0, 42];

        assert_eq!("jsr 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ret() -> Result<()> {
        let instruction = Instruction::Ret(42);
        let code = 169;
        let expected_bytes = [code, 42];

        assert_eq!("ret 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_tableswitch() -> Result<()> {
        let instruction = Instruction::Tableswitch {
            default: 42,
            low: 1,
            high: 2,
            offsets: vec![3, 4],
        };
        let code = 170;
        let expected_bytes = [
            170, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4,
        ];

        let expected = indoc! {"
            tableswitch   { // 1 to 2
                       1: 3
                       2: 4
                 default: 42
            }"};
        assert_eq!(expected, instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lookupswitch() -> Result<()> {
        let instruction = Instruction::Lookupswitch {
            default: 42,
            pairs: vec![(1, 2)],
        };
        let code = 171;
        let expected_bytes = [
            171, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 2,
        ];

        let expected = indoc! {"
            lookupswitch   { // 1
                       1: 2
                 default: 42
            }"};
        assert_eq!(expected, instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ireturn() -> Result<()> {
        let instruction = Instruction::Ireturn;
        let code = 172;
        let expected_bytes = [code];

        assert_eq!("ireturn", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_lreturn() -> Result<()> {
        let instruction = Instruction::Lreturn;
        let code = 173;
        let expected_bytes = [code];

        assert_eq!("lreturn", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_freturn() -> Result<()> {
        let instruction = Instruction::Freturn;
        let code = 174;
        let expected_bytes = [code];

        assert_eq!("freturn", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_dreturn() -> Result<()> {
        let instruction = Instruction::Dreturn;
        let code = 175;
        let expected_bytes = [code];

        assert_eq!("dreturn", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_areturn() -> Result<()> {
        let instruction = Instruction::Areturn;
        let code = 176;
        let expected_bytes = [code];

        assert_eq!("areturn", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_return() -> Result<()> {
        let instruction = Instruction::Return;
        let code = 177;
        let expected_bytes = [code];

        assert_eq!("return", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_getstatic() -> Result<()> {
        let instruction = Instruction::Getstatic(42);
        let code = 178;
        let expected_bytes = [code, 0, 42];

        assert_eq!("getstatic #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_putstatic() -> Result<()> {
        let instruction = Instruction::Putstatic(42);
        let code = 179;
        let expected_bytes = [code, 0, 42];

        assert_eq!("putstatic #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_getfield() -> Result<()> {
        let instruction = Instruction::Getfield(42);
        let code = 180;
        let expected_bytes = [code, 0, 42];

        assert_eq!("getfield #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_putfield() -> Result<()> {
        let instruction = Instruction::Putfield(42);
        let code = 181;
        let expected_bytes = [code, 0, 42];

        assert_eq!("putfield #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_invokevirtual() -> Result<()> {
        let instruction = Instruction::Invokevirtual(42);
        let code = 182;
        let expected_bytes = [code, 0, 42];

        assert_eq!("invokevirtual #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_invokespecial() -> Result<()> {
        let instruction = Instruction::Invokespecial(42);
        let code = 183;
        let expected_bytes = [code, 0, 42];

        assert_eq!("invokespecial #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_invokestatic() -> Result<()> {
        let instruction = Instruction::Invokestatic(42);
        let code = 184;
        let expected_bytes = [code, 0, 42];

        assert_eq!("invokestatic #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_invokeinterface() -> Result<()> {
        let instruction = Instruction::Invokeinterface(42, 3);
        let code = 185;
        let expected_bytes = [code, 0, 42, 3, 0];

        assert_eq!("invokeinterface #42, 3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_invokeinterface_error() {
        let bytes: [u8; 5] = [185, 0, 42, 3, 1];
        let mut cursor = Cursor::new(bytes.to_vec());
        assert_eq!(
            Instruction::from_bytes(&mut cursor),
            Err(InvalidInstruction(185))
        );
    }

    #[test]
    fn test_invokedynamic() -> Result<()> {
        let instruction = Instruction::Invokedynamic(42);
        let code = 186;
        let expected_bytes = [code, 0, 42];

        assert_eq!("invokedynamic #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_new() -> Result<()> {
        let instruction = Instruction::New(42);
        let code = 187;
        let expected_bytes = [code, 0, 42];

        assert_eq!("new #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_newarray() -> Result<()> {
        let instruction = Instruction::Newarray(ArrayType::Boolean);
        let code = 188;
        let expected_bytes = [code, 4];

        assert_eq!("newarray boolean", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_anewarray() -> Result<()> {
        let instruction = Instruction::Anewarray(42);
        let code = 189;
        let expected_bytes = [code, 0, 42];

        assert_eq!("anewarray #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_arraylength() -> Result<()> {
        let instruction = Instruction::Arraylength;
        let code = 190;
        let expected_bytes = [code];

        assert_eq!("arraylength", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_athrow() -> Result<()> {
        let instruction = Instruction::Athrow;
        let code = 191;
        let expected_bytes = [code];

        assert_eq!("athrow", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_checkcast() -> Result<()> {
        let instruction = Instruction::Checkcast(42);
        let code = 192;
        let expected_bytes = [code, 0, 42];

        assert_eq!("checkcast #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_instanceof() -> Result<()> {
        let instruction = Instruction::Instanceof(42);
        let code = 193;
        let expected_bytes = [code, 0, 42];

        assert_eq!("instanceof #42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_monitorenter() -> Result<()> {
        let instruction = Instruction::Monitorenter;
        let code = 194;
        let expected_bytes = [code];

        assert_eq!("monitorenter", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_monitorexit() -> Result<()> {
        let instruction = Instruction::Monitorexit;
        let code = 195;
        let expected_bytes = [code];

        assert_eq!("monitorexit", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_wide() {
        assert_eq!(196, Instruction::Wide.code());
        assert_eq!("wide", Instruction::Wide.to_string());
    }

    #[test]
    fn test_multianewarray() -> Result<()> {
        let instruction = Instruction::Multianewarray(42, 3);
        let code = 197;
        let expected_bytes = [code, 0, 42, 3];

        assert_eq!("multianewarray #42, 3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ifnull() -> Result<()> {
        let instruction = Instruction::Ifnull(42);
        let code = 198;
        let expected_bytes = [code, 0, 42];

        assert_eq!("ifnull 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_ifnonnull() -> Result<()> {
        let instruction = Instruction::Ifnonnull(42);
        let code = 199;
        let expected_bytes = [code, 0, 42];

        assert_eq!("ifnonnull 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_goto_w() -> Result<()> {
        let instruction = Instruction::Goto_w(42);
        let code = 200;
        let expected_bytes = [code, 0, 0, 0, 42];

        assert_eq!("goto_w 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_jsr_w() -> Result<()> {
        let instruction = Instruction::Jsr_w(42);
        let code = 201;
        let expected_bytes = [code, 0, 0, 0, 42];

        assert_eq!("jsr_w 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    #[test]
    fn test_breakpoint() -> Result<()> {
        let instruction = Instruction::Breakpoint;
        let code = 202;
        let expected_bytes = [code];

        assert_eq!("breakpoint", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, code)
    }

    // Wide instructions

    #[test]
    fn test_iload_w() -> Result<()> {
        let instruction = Instruction::Iload_w(42);
        let wide_code = 196;
        let code = 21;
        let expected_bytes = [wide_code, code, 0, 42];

        assert_eq!("iload_w 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, wide_code)
    }

    #[test]
    fn test_lload_w() -> Result<()> {
        let instruction = Instruction::Lload_w(42);
        let wide_code = 196;
        let code = 22;
        let expected_bytes = [wide_code, code, 0, 42];

        assert_eq!("lload_w 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, wide_code)
    }

    #[test]
    fn test_fload_w() -> Result<()> {
        let instruction = Instruction::Fload_w(42);
        let wide_code = 196;
        let code = 23;
        let expected_bytes = [wide_code, code, 0, 42];

        assert_eq!("fload_w 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, wide_code)
    }

    #[test]
    fn test_dload_w() -> Result<()> {
        let instruction = Instruction::Dload_w(42);
        let wide_code = 196;
        let code = 24;
        let expected_bytes = [wide_code, code, 0, 42];

        assert_eq!("dload_w 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, wide_code)
    }

    #[test]
    fn test_aload_w() -> Result<()> {
        let instruction = Instruction::Aload_w(42);
        let wide_code = 196;
        let code = 25;
        let expected_bytes = [wide_code, code, 0, 42];

        assert_eq!("aload_w 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, wide_code)
    }

    #[test]
    fn test_istore_w() -> Result<()> {
        let instruction = Instruction::Istore_w(42);
        let wide_code = 196;
        let code = 54;
        let expected_bytes = [wide_code, code, 0, 42];

        assert_eq!("istore_w 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, wide_code)
    }

    #[test]
    fn test_lstore_w() -> Result<()> {
        let instruction = Instruction::Lstore_w(42);
        let wide_code = 196;
        let code = 55;
        let expected_bytes = [wide_code, code, 0, 42];

        assert_eq!("lstore_w 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, wide_code)
    }

    #[test]
    fn test_fstore_w() -> Result<()> {
        let instruction = Instruction::Fstore_w(42);
        let wide_code = 196;
        let code = 56;
        let expected_bytes = [wide_code, code, 0, 42];

        assert_eq!("fstore_w 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, wide_code)
    }

    #[test]
    fn test_dstore_w() -> Result<()> {
        let instruction = Instruction::Dstore_w(42);
        let wide_code = 196;
        let code = 57;
        let expected_bytes = [wide_code, code, 0, 42];

        assert_eq!("dstore_w 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, wide_code)
    }

    #[test]
    fn test_astore_w() -> Result<()> {
        let instruction = Instruction::Astore_w(42);
        let wide_code = 196;
        let code = 58;
        let expected_bytes = [wide_code, code, 0, 42];

        assert_eq!("astore_w 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, wide_code)
    }

    #[test]
    fn test_iinc_w() -> Result<()> {
        let instruction = Instruction::Iinc_w(42, 3);
        let wide_code = 196;
        let code = 132;
        let expected_bytes = [wide_code, code, 0, 42, 0, 3];

        assert_eq!("iinc_w 42, 3", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, wide_code)
    }

    #[test]
    fn test_ret_w() -> Result<()> {
        let instruction = Instruction::Ret_w(42);
        let wide_code = 196;
        let code = 169;
        let expected_bytes = [wide_code, code, 0, 42];

        assert_eq!("ret_w 42", instruction.to_string());
        test_instruction(&instruction, &expected_bytes, wide_code)
    }

    #[test]
    fn test_wide_error() {
        let bytes: [u8; 4] = [196, 0, 1, 2];
        let mut cursor = Cursor::new(bytes.to_vec());
        assert_eq!(
            Instruction::from_bytes(&mut cursor),
            Err(InvalidWideInstruction(0))
        );
    }
}
