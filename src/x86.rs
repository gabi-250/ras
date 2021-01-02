pub mod assembler;
pub mod instruction;
pub mod instruction_encoding;
pub mod register;

enum Mode {
    Real,
    Protected,
    Long,
}
