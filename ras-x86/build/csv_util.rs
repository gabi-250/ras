#[macro_export]
macro_rules! get_header {
    ($rec:expr, $hdr:ident) => {
        $rec.get($crate::CsvHeader::$hdr as usize).unwrap()
    };
}

pub fn is_valid_mode(mode_rec: &str) -> bool {
    mode_rec == "Valid"
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
}
