#![doc = r" This file was autogenerated by build.rs."]
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::str::FromStr;
#[derive(Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Mnemonic {
    CMOVNP,
    MOVDQ2Q,
    SAL,
    CMOVPE,
    ARPL,
    SIDT,
    LDS,
    CVTPI2PD,
    FIDIVR,
    JNA,
    XSAVES,
    FUCOM,
    FSIN,
    ADC,
    CDQE,
    OR,
    FTST,
    SYSEXIT,
    BTS,
    LLDT,
    CMPS,
    INSD,
    FICOM,
    XSETBV,
    FLDENV,
    CVTPD2PI,
    FXSAVE,
    FCMOVNE,
    STR,
    FCMOVNBE,
    CMOVLE,
    MOV,
    BSWAP,
    CVTTPS2PI,
    LODSB,
    JL,
    CMOVB,
    BSF,
    MOVNTI,
    CMOVNE,
    PUSHA,
    FXCH,
    PUSHF,
    STI,
    FDECSTP,
    CBW,
    FCMOVNB,
    FXRSTOR64,
    FLDLG2,
    FSUBR,
    FNSAVE,
    INT,
    FICOMP,
    FISUB,
    JNP,
    JNGE,
    SBB,
    SETNA,
    CMOVL,
    JG,
    FISTTP,
    SETNG,
    ADD,
    FNINIT,
    FINIT,
    NOP,
    OUTSD,
    FXRSTOR,
    INVD,
    CMPSW,
    DAS,
    FCOMI,
    CQO,
    JA,
    JRCXZ,
    JP,
    SMSW,
    STOSB,
    CMPSD,
    JNLE,
    LOOPNE,
    FUCOMP,
    CMPSQ,
    CMOVAE,
    FLDCW,
    CMOVNC,
    SETNGE,
    PREFETCHT2,
    SYSCALL,
    JNAE,
    STC,
    AAS,
    SYSENTER,
    FXTRACT,
    FLDLN2,
    JNO,
    SETNLE,
    POPA,
    LOOP,
    LGS,
    LSL,
    XRSTOR,
    MASKMOVQ,
    XSAVE64,
    XSAVE,
    XLATB,
    FPATAN,
    SETB,
    BOUND,
    PREFETCHT1,
    CMOVNS,
    XRSTORS,
    STOSW,
    SWAPGS,
    FRNDINT,
    XRSTORS64,
    FLDZ,
    FABS,
    FILD,
    JPO,
    POP,
    INTO,
    LMSW,
    LOCK,
    TEST,
    SETNC,
    CLI,
    LFENCE,
    FLD,
    JNE,
    STD,
    CALL,
    INC,
    FYL2XP1,
    POPCNT,
    FLDL2E,
    FDIVP,
    FISUBR,
    LODSQ,
    FSINCOS,
    SCASW,
    DEC,
    FBSTP,
    INSB,
    MOVSX,
    FSUBRP,
    SCASB,
    SCAS,
    SUB,
    SETNBE,
    OUTS,
    JNZ,
    JAE,
    WBINVD,
    CDQ,
    FFREE,
    LEAVE,
    SETNAE,
    SETNL,
    FSUB,
    LES,
    JS,
    FNSTCW,
    FSTENV,
    JCXZ,
    LODS,
    FCMOVU,
    LEA,
    FPREM1,
    SAR,
    SETE,
    VERW,
    XLAT,
    XOR,
    FSTP,
    FCMOVB,
    PUSHFQ,
    IN,
    MOVSQ,
    DAA,
    CMOVO,
    SHR,
    XSAVES64,
    F2XM1,
    JGE,
    SLDT,
    FYL2X,
    FUCOMPP,
    LTR,
    FADDP,
    CMPXCHG,
    ENTER,
    FCMOVNU,
    IRET,
    PUSHFD,
    FIDIV,
    UD2,
    CLFLUSH,
    BTR,
    FDIVRP,
    MOVSB,
    CMP,
    CMOVNO,
    XADD,
    MWAIT,
    JO,
    HLT,
    JNB,
    MOVS,
    SCASQ,
    JBE,
    WAIT,
    CMPXCHG16B,
    LODSD,
    SCASD,
    PREFETCHNTA,
    CMPXCHG8B,
    SHL,
    JZ,
    BT,
    CMOVNZ,
    CMOVC,
    FMULP,
    CMOVNL,
    SETA,
    FSUBP,
    FDIV,
    SETL,
    FCOMIP,
    CLC,
    CLTS,
    CVTPI2PS,
    FBLD,
    FUCOMIP,
    LSS,
    INS,
    IRETQ,
    FCOMP,
    LODSW,
    BSR,
    MOVQ2DQ,
    ROL,
    FCOMPP,
    BTC,
    CMOVNBE,
    FIADD,
    FSTSW,
    ROR,
    CVTTPD2PI,
    JE,
    FUCOMI,
    DIV,
    JNL,
    LAHF,
    LGDT,
    RDTSCP,
    SETGE,
    INVLPG,
    SETNB,
    XGETBV,
    XSAVEC,
    XRSTOR64,
    LIDT,
    FCHS,
    FCOM,
    FXAM,
    SAHF,
    JPE,
    FNSTSW,
    FLD1,
    JNBE,
    CMOVNB,
    FSCALE,
    MOVSW,
    MOVSXD,
    RDMSR,
    RET,
    CMOVBE,
    FNSTENV,
    MUL,
    SHLD,
    JECXZ,
    CWDE,
    MONITOR,
    FPTAN,
    CMC,
    CLFLUSHOPT,
    JB,
    JMP,
    SGDT,
    FISTP,
    SFENCE,
    FLDL2T,
    FCMOVBE,
    JNC,
    SYSRET,
    STOSQ,
    FSTCW,
    MOVBE,
    CMOVNAE,
    FLDPI,
    FXSAVE64,
    OUTSW,
    POPAD,
    XSAVEC64,
    STOS,
    AND,
    CMOVG,
    FDIVR,
    POPF,
    IMUL,
    OUT,
    AAD,
    FRSTOR,
    NEG,
    AAA,
    NOT,
    CMOVNA,
    FNOP,
    SETG,
    UD0,
    RCL,
    SETBE,
    VERR,
    FWAIT,
    JNG,
    FCLEX,
    FSQRT,
    LAR,
    RDTSC,
    SETC,
    WRMSR,
    INSW,
    XCHG,
    OUTSB,
    SETLE,
    RCR,
    EMMS,
    CMPSB,
    FNCLEX,
    FCMOVE,
    MOVNTQ,
    CMOVNG,
    SHRD,
    PTWRITE,
    UD1,
    FIST,
    CMOVE,
    AAM,
    FPREM,
    FADD,
    IRETD,
    PUSH,
    FIMUL,
    STOSD,
    JC,
    PUSHAD,
    CRC32,
    LFS,
    CMOVNGE,
    CMOVP,
    LOOPE,
    FINCSTP,
    CMOVA,
    CMOVGE,
    IDIV,
    JNS,
    CVTPS2PI,
    CLD,
    MOVSD,
    POPFD,
    POPFQ,
    MFENCE,
    MOVZX,
    PREFETCHT0,
    PSHUFW,
    CWD,
    FST,
    PAUSE,
    CPUID,
    RDPMC,
    FCOS,
    FSAVE,
    RSM,
    SETAE,
    JLE,
    FMUL,
    CMOVNLE,
    SETNE,
}
impl FromStr for Mnemonic {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CMOVNP" => Ok(Mnemonic::CMOVNP),
            "MOVDQ2Q" => Ok(Mnemonic::MOVDQ2Q),
            "SAL" => Ok(Mnemonic::SAL),
            "CMOVPE" => Ok(Mnemonic::CMOVPE),
            "ARPL" => Ok(Mnemonic::ARPL),
            "SIDT" => Ok(Mnemonic::SIDT),
            "LDS" => Ok(Mnemonic::LDS),
            "CVTPI2PD" => Ok(Mnemonic::CVTPI2PD),
            "FIDIVR" => Ok(Mnemonic::FIDIVR),
            "JNA" => Ok(Mnemonic::JNA),
            "XSAVES" => Ok(Mnemonic::XSAVES),
            "FUCOM" => Ok(Mnemonic::FUCOM),
            "FSIN" => Ok(Mnemonic::FSIN),
            "ADC" => Ok(Mnemonic::ADC),
            "CDQE" => Ok(Mnemonic::CDQE),
            "OR" => Ok(Mnemonic::OR),
            "FTST" => Ok(Mnemonic::FTST),
            "SYSEXIT" => Ok(Mnemonic::SYSEXIT),
            "BTS" => Ok(Mnemonic::BTS),
            "LLDT" => Ok(Mnemonic::LLDT),
            "CMPS" => Ok(Mnemonic::CMPS),
            "INSD" => Ok(Mnemonic::INSD),
            "FICOM" => Ok(Mnemonic::FICOM),
            "XSETBV" => Ok(Mnemonic::XSETBV),
            "FLDENV" => Ok(Mnemonic::FLDENV),
            "CVTPD2PI" => Ok(Mnemonic::CVTPD2PI),
            "FXSAVE" => Ok(Mnemonic::FXSAVE),
            "FCMOVNE" => Ok(Mnemonic::FCMOVNE),
            "STR" => Ok(Mnemonic::STR),
            "FCMOVNBE" => Ok(Mnemonic::FCMOVNBE),
            "CMOVLE" => Ok(Mnemonic::CMOVLE),
            "MOV" => Ok(Mnemonic::MOV),
            "BSWAP" => Ok(Mnemonic::BSWAP),
            "CVTTPS2PI" => Ok(Mnemonic::CVTTPS2PI),
            "LODSB" => Ok(Mnemonic::LODSB),
            "JL" => Ok(Mnemonic::JL),
            "CMOVB" => Ok(Mnemonic::CMOVB),
            "BSF" => Ok(Mnemonic::BSF),
            "MOVNTI" => Ok(Mnemonic::MOVNTI),
            "CMOVNE" => Ok(Mnemonic::CMOVNE),
            "PUSHA" => Ok(Mnemonic::PUSHA),
            "FXCH" => Ok(Mnemonic::FXCH),
            "PUSHF" => Ok(Mnemonic::PUSHF),
            "STI" => Ok(Mnemonic::STI),
            "FDECSTP" => Ok(Mnemonic::FDECSTP),
            "CBW" => Ok(Mnemonic::CBW),
            "FCMOVNB" => Ok(Mnemonic::FCMOVNB),
            "FXRSTOR64" => Ok(Mnemonic::FXRSTOR64),
            "FLDLG2" => Ok(Mnemonic::FLDLG2),
            "FSUBR" => Ok(Mnemonic::FSUBR),
            "FNSAVE" => Ok(Mnemonic::FNSAVE),
            "INT" => Ok(Mnemonic::INT),
            "FICOMP" => Ok(Mnemonic::FICOMP),
            "FISUB" => Ok(Mnemonic::FISUB),
            "JNP" => Ok(Mnemonic::JNP),
            "JNGE" => Ok(Mnemonic::JNGE),
            "SBB" => Ok(Mnemonic::SBB),
            "SETNA" => Ok(Mnemonic::SETNA),
            "CMOVL" => Ok(Mnemonic::CMOVL),
            "JG" => Ok(Mnemonic::JG),
            "FISTTP" => Ok(Mnemonic::FISTTP),
            "SETNG" => Ok(Mnemonic::SETNG),
            "ADD" => Ok(Mnemonic::ADD),
            "FNINIT" => Ok(Mnemonic::FNINIT),
            "FINIT" => Ok(Mnemonic::FINIT),
            "NOP" => Ok(Mnemonic::NOP),
            "OUTSD" => Ok(Mnemonic::OUTSD),
            "FXRSTOR" => Ok(Mnemonic::FXRSTOR),
            "INVD" => Ok(Mnemonic::INVD),
            "CMPSW" => Ok(Mnemonic::CMPSW),
            "DAS" => Ok(Mnemonic::DAS),
            "FCOMI" => Ok(Mnemonic::FCOMI),
            "CQO" => Ok(Mnemonic::CQO),
            "JA" => Ok(Mnemonic::JA),
            "JRCXZ" => Ok(Mnemonic::JRCXZ),
            "JP" => Ok(Mnemonic::JP),
            "SMSW" => Ok(Mnemonic::SMSW),
            "STOSB" => Ok(Mnemonic::STOSB),
            "CMPSD" => Ok(Mnemonic::CMPSD),
            "JNLE" => Ok(Mnemonic::JNLE),
            "LOOPNE" => Ok(Mnemonic::LOOPNE),
            "FUCOMP" => Ok(Mnemonic::FUCOMP),
            "CMPSQ" => Ok(Mnemonic::CMPSQ),
            "CMOVAE" => Ok(Mnemonic::CMOVAE),
            "FLDCW" => Ok(Mnemonic::FLDCW),
            "CMOVNC" => Ok(Mnemonic::CMOVNC),
            "SETNGE" => Ok(Mnemonic::SETNGE),
            "PREFETCHT2" => Ok(Mnemonic::PREFETCHT2),
            "SYSCALL" => Ok(Mnemonic::SYSCALL),
            "JNAE" => Ok(Mnemonic::JNAE),
            "STC" => Ok(Mnemonic::STC),
            "AAS" => Ok(Mnemonic::AAS),
            "SYSENTER" => Ok(Mnemonic::SYSENTER),
            "FXTRACT" => Ok(Mnemonic::FXTRACT),
            "FLDLN2" => Ok(Mnemonic::FLDLN2),
            "JNO" => Ok(Mnemonic::JNO),
            "SETNLE" => Ok(Mnemonic::SETNLE),
            "POPA" => Ok(Mnemonic::POPA),
            "LOOP" => Ok(Mnemonic::LOOP),
            "LGS" => Ok(Mnemonic::LGS),
            "LSL" => Ok(Mnemonic::LSL),
            "XRSTOR" => Ok(Mnemonic::XRSTOR),
            "MASKMOVQ" => Ok(Mnemonic::MASKMOVQ),
            "XSAVE64" => Ok(Mnemonic::XSAVE64),
            "XSAVE" => Ok(Mnemonic::XSAVE),
            "XLATB" => Ok(Mnemonic::XLATB),
            "FPATAN" => Ok(Mnemonic::FPATAN),
            "SETB" => Ok(Mnemonic::SETB),
            "BOUND" => Ok(Mnemonic::BOUND),
            "PREFETCHT1" => Ok(Mnemonic::PREFETCHT1),
            "CMOVNS" => Ok(Mnemonic::CMOVNS),
            "XRSTORS" => Ok(Mnemonic::XRSTORS),
            "STOSW" => Ok(Mnemonic::STOSW),
            "SWAPGS" => Ok(Mnemonic::SWAPGS),
            "FRNDINT" => Ok(Mnemonic::FRNDINT),
            "XRSTORS64" => Ok(Mnemonic::XRSTORS64),
            "FLDZ" => Ok(Mnemonic::FLDZ),
            "FABS" => Ok(Mnemonic::FABS),
            "FILD" => Ok(Mnemonic::FILD),
            "JPO" => Ok(Mnemonic::JPO),
            "POP" => Ok(Mnemonic::POP),
            "INTO" => Ok(Mnemonic::INTO),
            "LMSW" => Ok(Mnemonic::LMSW),
            "LOCK" => Ok(Mnemonic::LOCK),
            "TEST" => Ok(Mnemonic::TEST),
            "SETNC" => Ok(Mnemonic::SETNC),
            "CLI" => Ok(Mnemonic::CLI),
            "LFENCE" => Ok(Mnemonic::LFENCE),
            "FLD" => Ok(Mnemonic::FLD),
            "JNE" => Ok(Mnemonic::JNE),
            "STD" => Ok(Mnemonic::STD),
            "CALL" => Ok(Mnemonic::CALL),
            "INC" => Ok(Mnemonic::INC),
            "FYL2XP1" => Ok(Mnemonic::FYL2XP1),
            "POPCNT" => Ok(Mnemonic::POPCNT),
            "FLDL2E" => Ok(Mnemonic::FLDL2E),
            "FDIVP" => Ok(Mnemonic::FDIVP),
            "FISUBR" => Ok(Mnemonic::FISUBR),
            "LODSQ" => Ok(Mnemonic::LODSQ),
            "FSINCOS" => Ok(Mnemonic::FSINCOS),
            "SCASW" => Ok(Mnemonic::SCASW),
            "DEC" => Ok(Mnemonic::DEC),
            "FBSTP" => Ok(Mnemonic::FBSTP),
            "INSB" => Ok(Mnemonic::INSB),
            "MOVSX" => Ok(Mnemonic::MOVSX),
            "FSUBRP" => Ok(Mnemonic::FSUBRP),
            "SCASB" => Ok(Mnemonic::SCASB),
            "SCAS" => Ok(Mnemonic::SCAS),
            "SUB" => Ok(Mnemonic::SUB),
            "SETNBE" => Ok(Mnemonic::SETNBE),
            "OUTS" => Ok(Mnemonic::OUTS),
            "JNZ" => Ok(Mnemonic::JNZ),
            "JAE" => Ok(Mnemonic::JAE),
            "WBINVD" => Ok(Mnemonic::WBINVD),
            "CDQ" => Ok(Mnemonic::CDQ),
            "FFREE" => Ok(Mnemonic::FFREE),
            "LEAVE" => Ok(Mnemonic::LEAVE),
            "SETNAE" => Ok(Mnemonic::SETNAE),
            "SETNL" => Ok(Mnemonic::SETNL),
            "FSUB" => Ok(Mnemonic::FSUB),
            "LES" => Ok(Mnemonic::LES),
            "JS" => Ok(Mnemonic::JS),
            "FNSTCW" => Ok(Mnemonic::FNSTCW),
            "FSTENV" => Ok(Mnemonic::FSTENV),
            "JCXZ" => Ok(Mnemonic::JCXZ),
            "LODS" => Ok(Mnemonic::LODS),
            "FCMOVU" => Ok(Mnemonic::FCMOVU),
            "LEA" => Ok(Mnemonic::LEA),
            "FPREM1" => Ok(Mnemonic::FPREM1),
            "SAR" => Ok(Mnemonic::SAR),
            "SETE" => Ok(Mnemonic::SETE),
            "VERW" => Ok(Mnemonic::VERW),
            "XLAT" => Ok(Mnemonic::XLAT),
            "XOR" => Ok(Mnemonic::XOR),
            "FSTP" => Ok(Mnemonic::FSTP),
            "FCMOVB" => Ok(Mnemonic::FCMOVB),
            "PUSHFQ" => Ok(Mnemonic::PUSHFQ),
            "IN" => Ok(Mnemonic::IN),
            "MOVSQ" => Ok(Mnemonic::MOVSQ),
            "DAA" => Ok(Mnemonic::DAA),
            "CMOVO" => Ok(Mnemonic::CMOVO),
            "SHR" => Ok(Mnemonic::SHR),
            "XSAVES64" => Ok(Mnemonic::XSAVES64),
            "F2XM1" => Ok(Mnemonic::F2XM1),
            "JGE" => Ok(Mnemonic::JGE),
            "SLDT" => Ok(Mnemonic::SLDT),
            "FYL2X" => Ok(Mnemonic::FYL2X),
            "FUCOMPP" => Ok(Mnemonic::FUCOMPP),
            "LTR" => Ok(Mnemonic::LTR),
            "FADDP" => Ok(Mnemonic::FADDP),
            "CMPXCHG" => Ok(Mnemonic::CMPXCHG),
            "ENTER" => Ok(Mnemonic::ENTER),
            "FCMOVNU" => Ok(Mnemonic::FCMOVNU),
            "IRET" => Ok(Mnemonic::IRET),
            "PUSHFD" => Ok(Mnemonic::PUSHFD),
            "FIDIV" => Ok(Mnemonic::FIDIV),
            "UD2" => Ok(Mnemonic::UD2),
            "CLFLUSH" => Ok(Mnemonic::CLFLUSH),
            "BTR" => Ok(Mnemonic::BTR),
            "FDIVRP" => Ok(Mnemonic::FDIVRP),
            "MOVSB" => Ok(Mnemonic::MOVSB),
            "CMP" => Ok(Mnemonic::CMP),
            "CMOVNO" => Ok(Mnemonic::CMOVNO),
            "XADD" => Ok(Mnemonic::XADD),
            "MWAIT" => Ok(Mnemonic::MWAIT),
            "JO" => Ok(Mnemonic::JO),
            "HLT" => Ok(Mnemonic::HLT),
            "JNB" => Ok(Mnemonic::JNB),
            "MOVS" => Ok(Mnemonic::MOVS),
            "SCASQ" => Ok(Mnemonic::SCASQ),
            "JBE" => Ok(Mnemonic::JBE),
            "WAIT" => Ok(Mnemonic::WAIT),
            "CMPXCHG16B" => Ok(Mnemonic::CMPXCHG16B),
            "LODSD" => Ok(Mnemonic::LODSD),
            "SCASD" => Ok(Mnemonic::SCASD),
            "PREFETCHNTA" => Ok(Mnemonic::PREFETCHNTA),
            "CMPXCHG8B" => Ok(Mnemonic::CMPXCHG8B),
            "SHL" => Ok(Mnemonic::SHL),
            "JZ" => Ok(Mnemonic::JZ),
            "BT" => Ok(Mnemonic::BT),
            "CMOVNZ" => Ok(Mnemonic::CMOVNZ),
            "CMOVC" => Ok(Mnemonic::CMOVC),
            "FMULP" => Ok(Mnemonic::FMULP),
            "CMOVNL" => Ok(Mnemonic::CMOVNL),
            "SETA" => Ok(Mnemonic::SETA),
            "FSUBP" => Ok(Mnemonic::FSUBP),
            "FDIV" => Ok(Mnemonic::FDIV),
            "SETL" => Ok(Mnemonic::SETL),
            "FCOMIP" => Ok(Mnemonic::FCOMIP),
            "CLC" => Ok(Mnemonic::CLC),
            "CLTS" => Ok(Mnemonic::CLTS),
            "CVTPI2PS" => Ok(Mnemonic::CVTPI2PS),
            "FBLD" => Ok(Mnemonic::FBLD),
            "FUCOMIP" => Ok(Mnemonic::FUCOMIP),
            "LSS" => Ok(Mnemonic::LSS),
            "INS" => Ok(Mnemonic::INS),
            "IRETQ" => Ok(Mnemonic::IRETQ),
            "FCOMP" => Ok(Mnemonic::FCOMP),
            "LODSW" => Ok(Mnemonic::LODSW),
            "BSR" => Ok(Mnemonic::BSR),
            "MOVQ2DQ" => Ok(Mnemonic::MOVQ2DQ),
            "ROL" => Ok(Mnemonic::ROL),
            "FCOMPP" => Ok(Mnemonic::FCOMPP),
            "BTC" => Ok(Mnemonic::BTC),
            "CMOVNBE" => Ok(Mnemonic::CMOVNBE),
            "FIADD" => Ok(Mnemonic::FIADD),
            "FSTSW" => Ok(Mnemonic::FSTSW),
            "ROR" => Ok(Mnemonic::ROR),
            "CVTTPD2PI" => Ok(Mnemonic::CVTTPD2PI),
            "JE" => Ok(Mnemonic::JE),
            "FUCOMI" => Ok(Mnemonic::FUCOMI),
            "DIV" => Ok(Mnemonic::DIV),
            "JNL" => Ok(Mnemonic::JNL),
            "LAHF" => Ok(Mnemonic::LAHF),
            "LGDT" => Ok(Mnemonic::LGDT),
            "RDTSCP" => Ok(Mnemonic::RDTSCP),
            "SETGE" => Ok(Mnemonic::SETGE),
            "INVLPG" => Ok(Mnemonic::INVLPG),
            "SETNB" => Ok(Mnemonic::SETNB),
            "XGETBV" => Ok(Mnemonic::XGETBV),
            "XSAVEC" => Ok(Mnemonic::XSAVEC),
            "XRSTOR64" => Ok(Mnemonic::XRSTOR64),
            "LIDT" => Ok(Mnemonic::LIDT),
            "FCHS" => Ok(Mnemonic::FCHS),
            "FCOM" => Ok(Mnemonic::FCOM),
            "FXAM" => Ok(Mnemonic::FXAM),
            "SAHF" => Ok(Mnemonic::SAHF),
            "JPE" => Ok(Mnemonic::JPE),
            "FNSTSW" => Ok(Mnemonic::FNSTSW),
            "FLD1" => Ok(Mnemonic::FLD1),
            "JNBE" => Ok(Mnemonic::JNBE),
            "CMOVNB" => Ok(Mnemonic::CMOVNB),
            "FSCALE" => Ok(Mnemonic::FSCALE),
            "MOVSW" => Ok(Mnemonic::MOVSW),
            "MOVSXD" => Ok(Mnemonic::MOVSXD),
            "RDMSR" => Ok(Mnemonic::RDMSR),
            "RET" => Ok(Mnemonic::RET),
            "CMOVBE" => Ok(Mnemonic::CMOVBE),
            "FNSTENV" => Ok(Mnemonic::FNSTENV),
            "MUL" => Ok(Mnemonic::MUL),
            "SHLD" => Ok(Mnemonic::SHLD),
            "JECXZ" => Ok(Mnemonic::JECXZ),
            "CWDE" => Ok(Mnemonic::CWDE),
            "MONITOR" => Ok(Mnemonic::MONITOR),
            "FPTAN" => Ok(Mnemonic::FPTAN),
            "CMC" => Ok(Mnemonic::CMC),
            "CLFLUSHOPT" => Ok(Mnemonic::CLFLUSHOPT),
            "JB" => Ok(Mnemonic::JB),
            "JMP" => Ok(Mnemonic::JMP),
            "SGDT" => Ok(Mnemonic::SGDT),
            "FISTP" => Ok(Mnemonic::FISTP),
            "SFENCE" => Ok(Mnemonic::SFENCE),
            "FLDL2T" => Ok(Mnemonic::FLDL2T),
            "FCMOVBE" => Ok(Mnemonic::FCMOVBE),
            "JNC" => Ok(Mnemonic::JNC),
            "SYSRET" => Ok(Mnemonic::SYSRET),
            "STOSQ" => Ok(Mnemonic::STOSQ),
            "FSTCW" => Ok(Mnemonic::FSTCW),
            "MOVBE" => Ok(Mnemonic::MOVBE),
            "CMOVNAE" => Ok(Mnemonic::CMOVNAE),
            "FLDPI" => Ok(Mnemonic::FLDPI),
            "FXSAVE64" => Ok(Mnemonic::FXSAVE64),
            "OUTSW" => Ok(Mnemonic::OUTSW),
            "POPAD" => Ok(Mnemonic::POPAD),
            "XSAVEC64" => Ok(Mnemonic::XSAVEC64),
            "STOS" => Ok(Mnemonic::STOS),
            "AND" => Ok(Mnemonic::AND),
            "CMOVG" => Ok(Mnemonic::CMOVG),
            "FDIVR" => Ok(Mnemonic::FDIVR),
            "POPF" => Ok(Mnemonic::POPF),
            "IMUL" => Ok(Mnemonic::IMUL),
            "OUT" => Ok(Mnemonic::OUT),
            "AAD" => Ok(Mnemonic::AAD),
            "FRSTOR" => Ok(Mnemonic::FRSTOR),
            "NEG" => Ok(Mnemonic::NEG),
            "AAA" => Ok(Mnemonic::AAA),
            "NOT" => Ok(Mnemonic::NOT),
            "CMOVNA" => Ok(Mnemonic::CMOVNA),
            "FNOP" => Ok(Mnemonic::FNOP),
            "SETG" => Ok(Mnemonic::SETG),
            "UD0" => Ok(Mnemonic::UD0),
            "RCL" => Ok(Mnemonic::RCL),
            "SETBE" => Ok(Mnemonic::SETBE),
            "VERR" => Ok(Mnemonic::VERR),
            "FWAIT" => Ok(Mnemonic::FWAIT),
            "JNG" => Ok(Mnemonic::JNG),
            "FCLEX" => Ok(Mnemonic::FCLEX),
            "FSQRT" => Ok(Mnemonic::FSQRT),
            "LAR" => Ok(Mnemonic::LAR),
            "RDTSC" => Ok(Mnemonic::RDTSC),
            "SETC" => Ok(Mnemonic::SETC),
            "WRMSR" => Ok(Mnemonic::WRMSR),
            "INSW" => Ok(Mnemonic::INSW),
            "XCHG" => Ok(Mnemonic::XCHG),
            "OUTSB" => Ok(Mnemonic::OUTSB),
            "SETLE" => Ok(Mnemonic::SETLE),
            "RCR" => Ok(Mnemonic::RCR),
            "EMMS" => Ok(Mnemonic::EMMS),
            "CMPSB" => Ok(Mnemonic::CMPSB),
            "FNCLEX" => Ok(Mnemonic::FNCLEX),
            "FCMOVE" => Ok(Mnemonic::FCMOVE),
            "MOVNTQ" => Ok(Mnemonic::MOVNTQ),
            "CMOVNG" => Ok(Mnemonic::CMOVNG),
            "SHRD" => Ok(Mnemonic::SHRD),
            "PTWRITE" => Ok(Mnemonic::PTWRITE),
            "UD1" => Ok(Mnemonic::UD1),
            "FIST" => Ok(Mnemonic::FIST),
            "CMOVE" => Ok(Mnemonic::CMOVE),
            "AAM" => Ok(Mnemonic::AAM),
            "FPREM" => Ok(Mnemonic::FPREM),
            "FADD" => Ok(Mnemonic::FADD),
            "IRETD" => Ok(Mnemonic::IRETD),
            "PUSH" => Ok(Mnemonic::PUSH),
            "FIMUL" => Ok(Mnemonic::FIMUL),
            "STOSD" => Ok(Mnemonic::STOSD),
            "JC" => Ok(Mnemonic::JC),
            "PUSHAD" => Ok(Mnemonic::PUSHAD),
            "CRC32" => Ok(Mnemonic::CRC32),
            "LFS" => Ok(Mnemonic::LFS),
            "CMOVNGE" => Ok(Mnemonic::CMOVNGE),
            "CMOVP" => Ok(Mnemonic::CMOVP),
            "LOOPE" => Ok(Mnemonic::LOOPE),
            "FINCSTP" => Ok(Mnemonic::FINCSTP),
            "CMOVA" => Ok(Mnemonic::CMOVA),
            "CMOVGE" => Ok(Mnemonic::CMOVGE),
            "IDIV" => Ok(Mnemonic::IDIV),
            "JNS" => Ok(Mnemonic::JNS),
            "CVTPS2PI" => Ok(Mnemonic::CVTPS2PI),
            "CLD" => Ok(Mnemonic::CLD),
            "MOVSD" => Ok(Mnemonic::MOVSD),
            "POPFD" => Ok(Mnemonic::POPFD),
            "POPFQ" => Ok(Mnemonic::POPFQ),
            "MFENCE" => Ok(Mnemonic::MFENCE),
            "MOVZX" => Ok(Mnemonic::MOVZX),
            "PREFETCHT0" => Ok(Mnemonic::PREFETCHT0),
            "PSHUFW" => Ok(Mnemonic::PSHUFW),
            "CWD" => Ok(Mnemonic::CWD),
            "FST" => Ok(Mnemonic::FST),
            "PAUSE" => Ok(Mnemonic::PAUSE),
            "CPUID" => Ok(Mnemonic::CPUID),
            "RDPMC" => Ok(Mnemonic::RDPMC),
            "FCOS" => Ok(Mnemonic::FCOS),
            "FSAVE" => Ok(Mnemonic::FSAVE),
            "RSM" => Ok(Mnemonic::RSM),
            "SETAE" => Ok(Mnemonic::SETAE),
            "JLE" => Ok(Mnemonic::JLE),
            "FMUL" => Ok(Mnemonic::FMUL),
            "CMOVNLE" => Ok(Mnemonic::CMOVNLE),
            "SETNE" => Ok(Mnemonic::SETNE),
            s => Err(format!("unknown mnemonic: {}", s)),
        }
    }
}
