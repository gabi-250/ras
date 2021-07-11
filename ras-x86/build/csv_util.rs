#[macro_export]
macro_rules! get_header {
    ($rec:expr, $hdr:ident) => {
        $rec.get($crate::CsvHeader::$hdr as usize).unwrap()
    };
}

#[allow(unused)]
#[repr(u8)]
pub enum CsvHeader {
    Instruction,
    Opcode,
    Valid64,
    Valid32,
    Valid16,
    FeatureFlags,
    Operand1,
    Operand2,
    Operand3,
    Operand4,
    TupleType,
    Description,
}
