use ras_x86_repr::{OperandKind, OperandRepr};

pub fn parse_instruction_column(inst: &str) -> (String, Vec<OperandRepr>) {
    if let Some(i) = inst.chars().position(|c| c == ' ') {
        let mnemonic = inst[..i].into();
        let operands = parse_operands(inst[i..].as_bytes());
        (mnemonic, operands)
    } else {
        (inst.into(), vec![])
    }
}

fn parse_operands(inst: &[u8]) -> Vec<OperandRepr> {
    let mut operands = vec![];
    let mut inst = skip_separators(inst, |c| c == b' ');
    loop {
        let (operand, remaining) = parse_operand(inst);
        inst = remaining;
        operands.push(operand);
        if inst.is_empty() {
            break;
        }
    }
    operands
}

fn parse_operand(inst: &[u8]) -> (OperandRepr, &[u8]) {
    let mut i = 0;
    while i < inst.len() && !is_separator(inst[i]) {
        i += 1;
    }

    let operand = match &inst[..i] {
        b"0" => OperandRepr::new(OperandKind::Zero, 8),
        b"1" => OperandRepr::new(OperandKind::One, 8),
        b"3" => OperandRepr::new(OperandKind::Three, 8),
        b"Sreg" => OperandRepr::new(OperandKind::Sreg, 0),
        b"CR0-CR7" => OperandRepr::new(OperandKind::Cr, 0),
        b"CR8" => OperandRepr::new(OperandKind::Cr8, 0),
        b"DR0-DR7" => OperandRepr::new(OperandKind::Dr, 0),
        b"CS" => OperandRepr::new(OperandKind::Cs, 16),
        b"DS" => OperandRepr::new(OperandKind::Ds, 16),
        b"ES" => OperandRepr::new(OperandKind::Es, 16),
        b"FS" => OperandRepr::new(OperandKind::Fs, 16),
        b"GS" => OperandRepr::new(OperandKind::Gs, 16),
        b"SS" => OperandRepr::new(OperandKind::Ss, 16),
        b"CL" => OperandRepr::new(OperandKind::Cl, 8),
        b"DX" => OperandRepr::new(OperandKind::Dx, 16),
        b"AL" => OperandRepr::new(OperandKind::Al, 8),
        b"AX" => OperandRepr::new(OperandKind::Al, 16),
        b"EAX" => OperandRepr::new(OperandKind::Al, 32),
        b"RAX" => OperandRepr::new(OperandKind::Al, 64),
        b"reg" => OperandRepr::new(OperandKind::Reg, 64),
        // XXX the size isn't right
        b"r32/m16" => OperandRepr::new(OperandKind::R32M16, 32),
        b"r64/m16" => OperandRepr::new(OperandKind::R64M16, 64),
        b"r/m8" => OperandRepr::new(OperandKind::ModRmRegMem, 8),
        b"r/m16" | b"r16/m16" => OperandRepr::new(OperandKind::ModRmRegMem, 16),
        b"r/m32" | b"r32/m32" => OperandRepr::new(OperandKind::ModRmRegMem, 32),
        b"r/m64" | b"r64/m64" => OperandRepr::new(OperandKind::ModRmRegMem, 64),
        b"r8" => OperandRepr::new(OperandKind::ModRmReg, 8),
        b"r16" => OperandRepr::new(OperandKind::ModRmReg, 16),
        b"r32" => OperandRepr::new(OperandKind::ModRmReg, 32),
        b"r64" => OperandRepr::new(OperandKind::ModRmReg, 64),
        b"imm8" => OperandRepr::new(OperandKind::Imm, 8),
        b"imm16" => OperandRepr::new(OperandKind::Imm, 16),
        b"imm32" => OperandRepr::new(OperandKind::Imm, 32),
        b"imm64" => OperandRepr::new(OperandKind::Imm, 64),
        b"moffs8" => OperandRepr::new(OperandKind::Moffs, 8),
        b"moffs16" => OperandRepr::new(OperandKind::Moffs, 16),
        b"moffs32" => OperandRepr::new(OperandKind::Moffs, 32),
        b"moffs64" => OperandRepr::new(OperandKind::Moffs, 64),
        b"m16&16" => OperandRepr::new(OperandKind::M16And16, 32),
        b"m16&32" => OperandRepr::new(OperandKind::M16And16, 48),
        b"m32&32" => OperandRepr::new(OperandKind::M16And16, 64),
        b"m16&64" => OperandRepr::new(OperandKind::M16And16, 80),
        b"rel8" => OperandRepr::new(OperandKind::Rel8, 8),
        b"rel16" => OperandRepr::new(OperandKind::Rel16, 16),
        b"rel32" => OperandRepr::new(OperandKind::Rel32, 32),
        b"m" | b"mem" => OperandRepr::new(OperandKind::M, 64),
        b"m8" => OperandRepr::new(OperandKind::M8, 8),
        b"m16" => OperandRepr::new(OperandKind::M16, 16),
        b"m32" => OperandRepr::new(OperandKind::M32, 32),
        b"m64" => OperandRepr::new(OperandKind::M64, 64),
        b"m128" => OperandRepr::new(OperandKind::M128, 128),
        // XXX check the sizes:
        b"ptr16:16" => OperandRepr::new(OperandKind::FarPointer16, 16),
        b"ptr16:32" => OperandRepr::new(OperandKind::FarPointer16, 32),
        b"m16:16" => OperandRepr::new(OperandKind::MemIndirectFarPointer16, 16),
        b"m16:32" => OperandRepr::new(OperandKind::MemIndirectFarPointer16, 32),
        b"m16:64" => OperandRepr::new(OperandKind::MemIndirectFarPointer16, 64),
        b"mm" => OperandRepr::new(OperandKind::Mm, 64),
        b"mm1" => OperandRepr::new(OperandKind::Mm1, 64),
        b"mm2" => OperandRepr::new(OperandKind::Mm2, 64),
        b"mm2/m64" => OperandRepr::new(OperandKind::Mm2M64, 64),
        b"mm/m64" => OperandRepr::new(OperandKind::MmM64, 64),
        b"xmm" => OperandRepr::new(OperandKind::Mm, 128),
        b"xmm/m64" => OperandRepr::new(OperandKind::XmmM64, 64),
        b"xmm/m128" => OperandRepr::new(OperandKind::XmmM128, 128),
        b"m32fp" => OperandRepr::new(OperandKind::M32Fp, 32),
        b"m64fp" => OperandRepr::new(OperandKind::M64Fp, 64),
        b"m80fp" => OperandRepr::new(OperandKind::M80Fp, 80),
        b"m16int" => OperandRepr::new(OperandKind::M16Int, 16),
        b"m32int" => OperandRepr::new(OperandKind::M32Int, 32),
        b"m64int" => OperandRepr::new(OperandKind::M64Int, 64),
        b"ST" | b"ST(0)" => OperandRepr::new(OperandKind::St0, 80),
        b"ST(i)" => OperandRepr::new(OperandKind::Sti, 80),
        // The CSV uses m80dec instead of m80bcd
        b"m80dec" | b"m80bcd" => OperandRepr::new(OperandKind::M80Bcd, 80),
        b"m2byte" => OperandRepr::new(OperandKind::M2Byte, 0),
        b"m14/28byte" => OperandRepr::new(OperandKind::M14M28Byte, 0),
        b"m94/108byte" => OperandRepr::new(OperandKind::M94M108Byte, 0),
        b"m512byte" => OperandRepr::new(OperandKind::M512Byte, 0),
        op => panic!("unkown operand repr: {}", std::str::from_utf8(op).unwrap()),
    };

    let inst = skip_separators(&inst[i..], &is_separator);

    (operand, inst)
}

fn is_separator(c: u8) -> bool {
    c == b' ' || c == b','
}

fn skip_separators(s: &[u8], is_sep: impl Fn(u8) -> bool) -> &[u8] {
    let mut i = 0;
    while i < s.len() && is_sep(s[i]) {
        i += 1;
    }
    &s[i..]
}
