use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// The internal representation of x86 instruction operand.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperandRepr {
    pub kind: OperandKind,
    pub size: u32,
}

impl OperandRepr {
    pub fn new(kind: OperandKind, size: u32) -> Self {
        Self { kind, size }
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

/// The type of the operand, as described in section `3.1.1.3 Instruction Column in the Opcode
/// Summary Table` of the [Intel® 64 and IA-32 architectures software developer's manual volume 2].
///
/// [Intel® 64 and IA-32 architectures software developer's manual volume 2]: https://software.intel.com/content/www/us/en/develop/articles/intel-sdm.html
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperandKind {
    /// reg
    Reg,
    // XXX the size of the operand can be 32 if it is a register, or 16 if it's a memory operand
    R32M16,
    R64M16,
    /// ModRM:reg
    ModRmReg,
    /// ModRM:r/m
    ModRmRegMem,
    /// imm8/16/32
    Imm,
    /// Moffs
    Moffs,
    /// AL/AX/EAX/RAX
    Al,
    /// opcode + rd
    OpcodeRd,
    /// 0
    Zero,
    /// 1
    One,
    /// 3
    Three,
    /// Sreg
    Sreg,
    /// CR0-CR7
    Cr,
    /// CR8
    Cr8,
    /// DR0-DR7
    Dr,
    /// CS
    Cs,
    /// DS
    Ds,
    /// ES
    Es,
    /// FS
    Fs,
    /// GS
    Gs,
    /// SS
    Ss,
    /// CL
    Cl,
    /// DX
    Dx,
    /// m16&16
    M16And16,
    /// m16&32
    M16And32,
    /// m32&32
    M32And32,
    /// m16&64
    M16And64,
    /// rel8
    Rel8,
    /// rel16
    Rel16,
    /// rel32
    Rel32,
    /// m
    M,
    /// m8
    M8,
    /// m16
    M16,
    /// m32
    M32,
    /// m64
    M64,
    /// m128
    M128,
    /// ptr16:16
    FarPointer16,
    /// ptr16:32
    FarPointer32,
    /// m16:16
    MemIndirectFarPointer16,
    /// m16:32
    MemIndirectFarPointer32,
    /// m16:64
    MemIndirectFarPointer64,
    /// mm
    Mm,
    /// mm1
    Mm1,
    /// mm2
    Mm2,
    /// mm2/m64
    Mm2M64,
    /// mm/m64
    MmM64,
    /// xmm
    Xmm,
    /// xmm/m64
    XmmM64,
    /// xmm/m128
    XmmM128,
    /// m32fp
    M32Fp,
    /// m64fp
    M64Fp,
    /// m80fp
    M80Fp,
    /// m16int
    M16Int,
    /// m32int
    M32Int,
    /// m64int
    M64Int,
    /// ST or ST(0)
    St0,
    /// ST(i)
    Sti,
    /// m80bcd
    M80Bcd,
    /// m2byte
    M2Byte,
    /// m14/28byte
    M14M28Byte,
    /// m94/108byte
    M94M108Byte,
    /// m512byte
    M512Byte,
}

impl FromStr for OperandRepr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(OperandRepr::new(OperandKind::Zero, 8)),
            "1" => Ok(OperandRepr::new(OperandKind::One, 8)),
            "3" => Ok(OperandRepr::new(OperandKind::Three, 8)),
            "Sreg" => Ok(OperandRepr::new(OperandKind::Sreg, 0)),
            "CR0-CR7" => Ok(OperandRepr::new(OperandKind::Cr, 0)),
            "CR8" => Ok(OperandRepr::new(OperandKind::Cr8, 0)),
            "DR0-DR7" => Ok(OperandRepr::new(OperandKind::Dr, 0)),
            "CS" => Ok(OperandRepr::new(OperandKind::Cs, 16)),
            "DS" => Ok(OperandRepr::new(OperandKind::Ds, 16)),
            "ES" => Ok(OperandRepr::new(OperandKind::Es, 16)),
            "FS" => Ok(OperandRepr::new(OperandKind::Fs, 16)),
            "GS" => Ok(OperandRepr::new(OperandKind::Gs, 16)),
            "SS" => Ok(OperandRepr::new(OperandKind::Ss, 16)),
            "CL" => Ok(OperandRepr::new(OperandKind::Cl, 8)),
            "DX" => Ok(OperandRepr::new(OperandKind::Dx, 16)),
            "AL" => Ok(OperandRepr::new(OperandKind::Al, 8)),
            "AX" => Ok(OperandRepr::new(OperandKind::Al, 16)),
            "EAX" => Ok(OperandRepr::new(OperandKind::Al, 32)),
            "RAX" => Ok(OperandRepr::new(OperandKind::Al, 64)),
            "reg" => Ok(OperandRepr::new(OperandKind::Reg, 64)),
            // XXX the size isn't right
            "r32/m16" => Ok(OperandRepr::new(OperandKind::R32M16, 32)),
            "r64/m16" => Ok(OperandRepr::new(OperandKind::R64M16, 64)),
            "r/m8" => Ok(OperandRepr::new(OperandKind::ModRmRegMem, 8)),
            "r/m16" | "r16/m16" => Ok(OperandRepr::new(OperandKind::ModRmRegMem, 16)),
            "r/m32" | "r32/m32" => Ok(OperandRepr::new(OperandKind::ModRmRegMem, 32)),
            "r/m64" | "r64/m64" => Ok(OperandRepr::new(OperandKind::ModRmRegMem, 64)),
            "r8" => Ok(OperandRepr::new(OperandKind::ModRmReg, 8)),
            "r16" => Ok(OperandRepr::new(OperandKind::ModRmReg, 16)),
            "r32" => Ok(OperandRepr::new(OperandKind::ModRmReg, 32)),
            "r64" => Ok(OperandRepr::new(OperandKind::ModRmReg, 64)),
            "imm8" => Ok(OperandRepr::new(OperandKind::Imm, 8)),
            "imm16" => Ok(OperandRepr::new(OperandKind::Imm, 16)),
            "imm32" => Ok(OperandRepr::new(OperandKind::Imm, 32)),
            "imm64" => Ok(OperandRepr::new(OperandKind::Imm, 64)),
            "moffs8" => Ok(OperandRepr::new(OperandKind::Moffs, 8)),
            "moffs16" => Ok(OperandRepr::new(OperandKind::Moffs, 16)),
            "moffs32" => Ok(OperandRepr::new(OperandKind::Moffs, 32)),
            "moffs64" => Ok(OperandRepr::new(OperandKind::Moffs, 64)),
            "m16&16" => Ok(OperandRepr::new(OperandKind::M16And16, 32)),
            "m16&32" => Ok(OperandRepr::new(OperandKind::M16And16, 48)),
            "m32&32" => Ok(OperandRepr::new(OperandKind::M16And16, 64)),
            "m16&64" => Ok(OperandRepr::new(OperandKind::M16And16, 80)),
            "rel8" => Ok(OperandRepr::new(OperandKind::Rel8, 8)),
            "rel16" => Ok(OperandRepr::new(OperandKind::Rel16, 16)),
            "rel32" => Ok(OperandRepr::new(OperandKind::Rel32, 32)),
            "m" | "mem" => Ok(OperandRepr::new(OperandKind::M, 64)),
            "m8" => Ok(OperandRepr::new(OperandKind::M8, 8)),
            "m16" => Ok(OperandRepr::new(OperandKind::M16, 16)),
            "m32" => Ok(OperandRepr::new(OperandKind::M32, 32)),
            "m64" => Ok(OperandRepr::new(OperandKind::M64, 64)),
            "m128" => Ok(OperandRepr::new(OperandKind::M128, 128)),
            // XXX check the sizes:
            "ptr16:16" => Ok(OperandRepr::new(OperandKind::FarPointer16, 16)),
            "ptr16:32" => Ok(OperandRepr::new(OperandKind::FarPointer16, 32)),
            "m16:16" => Ok(OperandRepr::new(OperandKind::MemIndirectFarPointer16, 16)),
            "m16:32" => Ok(OperandRepr::new(OperandKind::MemIndirectFarPointer16, 32)),
            "m16:64" => Ok(OperandRepr::new(OperandKind::MemIndirectFarPointer16, 64)),
            "mm" => Ok(OperandRepr::new(OperandKind::Mm, 64)),
            "mm1" => Ok(OperandRepr::new(OperandKind::Mm1, 64)),
            "mm2" => Ok(OperandRepr::new(OperandKind::Mm2, 64)),
            "mm2/m64" => Ok(OperandRepr::new(OperandKind::Mm2M64, 64)),
            "mm/m64" => Ok(OperandRepr::new(OperandKind::MmM64, 64)),
            "xmm" => Ok(OperandRepr::new(OperandKind::Mm, 128)),
            "xmm/m64" => Ok(OperandRepr::new(OperandKind::XmmM64, 64)),
            "xmm/m128" => Ok(OperandRepr::new(OperandKind::XmmM128, 128)),
            "m32fp" => Ok(OperandRepr::new(OperandKind::M32Fp, 32)),
            "m64fp" => Ok(OperandRepr::new(OperandKind::M64Fp, 64)),
            "m80fp" => Ok(OperandRepr::new(OperandKind::M80Fp, 80)),
            "m16int" => Ok(OperandRepr::new(OperandKind::M16Int, 16)),
            "m32int" => Ok(OperandRepr::new(OperandKind::M32Int, 32)),
            "m64int" => Ok(OperandRepr::new(OperandKind::M64Int, 64)),
            "ST" | "ST(0)" => Ok(OperandRepr::new(OperandKind::St0, 80)),
            "ST(i)" => Ok(OperandRepr::new(OperandKind::Sti, 80)),
            // The CSV uses m80dec instead of m80bcd
            "m80dec" | "m80bcd" => Ok(OperandRepr::new(OperandKind::M80Bcd, 80)),
            "m2byte" => Ok(OperandRepr::new(OperandKind::M2Byte, 0)),
            "m14/28byte" => Ok(OperandRepr::new(OperandKind::M14M28Byte, 0)),
            "m94/108byte" => Ok(OperandRepr::new(OperandKind::M94M108Byte, 0)),
            "m512byte" => Ok(OperandRepr::new(OperandKind::M512Byte, 0)),
            _ => Err(format!("failed to parse OperandKind: {}", s)),
        }
    }
}
