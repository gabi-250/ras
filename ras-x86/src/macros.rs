#[macro_export]
macro_rules! i {
    ($opcode:ident $(, $operands:expr)*) => {
        $crate::instruction::Instruction::new(
            $crate::mnemonic::Mnemonic::$opcode,
            vec![$($operands),*]
        )
    }
}

#[macro_export]
macro_rules! reg {
    ($reg:expr) => {
        $crate::operand::Operand::Register(*$reg)
    };
}

#[macro_export]
macro_rules! imm8 {
    ($imm:expr) => {
        $crate::operand::Operand::Immediate($crate::operand::Immediate::Imm8($imm))
    };
}

#[macro_export]
macro_rules! imm16 {
    ($imm:expr) => {
        $crate::operand::Operand::Immediate($crate::operand::Immediate::Imm16($imm))
    };
}

#[macro_export]
macro_rules! imm32 {
    ($imm:expr) => {
        $crate::operand::Operand::Immediate($crate::operand::Immediate::Imm32($imm))
    };
}

#[macro_export]
macro_rules! imm64 {
    ($imm:expr) => {
        $crate::operand::Operand::Immediate($crate::operand::Immediate::Imm64($imm))
    };
}

#[macro_export]
macro_rules! sib {
    ($($seg:expr)?; $($disp:expr)?; ($($base:expr)?, $($index:expr)?, $($scale:expr)?)) => {{
        let _seg: Option<$crate::register::Register> = None;
        $(
            let _seg = Some(*$seg);
        )*

        let _disp: Option<u64> = None;
        $(
            let _disp = Some($disp);
        )*

        let _base: Option<$crate::register::Register> = None;
        $(
            let _base = Some(*$base);
        )*

        let _index: Option<$crate::register::Register> = None;
        $(
            let _index = Some(*$index);
        )*

        let _scale = Scale::Byte;
        $(
            let _scale = $scale;
        )*

        $crate::operand::Operand::Memory($crate::operand::Memory::sib(_seg, _base, _index, _scale, _disp))
    }};
}

#[macro_export]
macro_rules! label {
    ($label:expr) => {
        $crate::operand::Operand::Memory($crate::operand::Memory::Relative(
            $crate::operand::MemoryRel::Label($label),
        ))
    };
}
