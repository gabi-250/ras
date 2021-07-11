use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperandKind {
    /// reg
    Reg,
    /// XXX the size of the operand can be 32 if it is a register, or 16 if it's a memory operand
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
