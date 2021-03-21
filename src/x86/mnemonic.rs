# ! [doc = r" This file was autogenerated by build.rs."]use std::hash::Hash;
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Mnemonic {
    FRNDINT,
    PUSH,
    JMP,
    FSUBR,
    BSWAP,
    XSAVE,
    IMUL,
    FLD1,
    FYL2XP1,
    FCOMIP,
    FIST,
    SETNB,
    WRMSR,
    BOUND,
    CMOVNGE,
    JGE,
    JPO,
    MOVNTI,
    STOS,
    TEST,
    JNA,
    SHR,
    CALL,
    XLATB,
    SCASQ,
    CMOVO,
    SYSEXIT,
    NOP,
    RCR,
    WAIT,
    CMOVE,
    RDTSCP,
    SETGE,
    FLDPI,
    JNB,
    MOVSX,
    JA,
    FUCOMIP,
    LAHF,
    CMPS,
    FSINCOS,
    FADD,
    FISTTP,
    LGS,
    NEG,
    LODSD,
    MOVSXD,
    FMULP,
    CMPSD,
    FNINIT,
    LFS,
    JBE,
    FPTAN,
    AND,
    PAUSE,
    BTR,
    CMC,
    FDIV,
    CVTPS2PI,
    BSF,
    CMOVB,
    FLDL2E,
    CMPSB,
    FICOMP,
    JG,
    JNBE,
    JNGE,
    LSS,
    PUSHAD,
    IN,
    RSM,
    SETA,
    FSIN,
    FCMOVNBE,
    STOSW,
    FCOM,
    DAA,
    JRCXZ,
    JNE,
    FLD,
    FNSTCW,
    CLC,
    LEAVE,
    FXTRACT,
    FCLEX,
    CLTS,
    LODSB,
    OUTSD,
    SETBE,
    STC,
    FXAM,
    INTO,
    ROR,
    FXSAVE64,
    SAL,
    SCASB,
    XRSTOR,
    CMPXCHG,
    FCMOVBE,
    JP,
    SYSCALL,
    XSAVES,
    LES,
    FDIVRP,
    AAD,
    CLFLUSH,
    FSUB,
    STR,
    FSTSW,
    JZ,
    PREFETCHT1,
    FLDZ,
    VERR,
    VERW,
    INT,
    SMSW,
    CWDE,
    SETNAE,
    SIDT,
    INS,
    MFENCE,
    MOVNTQ,
    MWAIT,
    CBW,
    CQO,
    MONITOR,
    CMOVA,
    SETNE,
    STOSQ,
    AAS,
    FCMOVNE,
    FLDLN2,
    FISUBR,
    LAR,
    MOVSQ,
    INSB,
    AAA,
    XLAT,
    POPF,
    FSQRT,
    XRSTOR64,
    UD2,
    FCMOVNU,
    JE,
    FUCOMI,
    SETNBE,
    XSETBV,
    CMPXCHG8B,
    CMOVLE,
    XCHG,
    FLDENV,
    ADC,
    INC,
    JS,
    MOVSB,
    FIMUL,
    MOVZX,
    RDPMC,
    MOVQ2DQ,
    IRETD,
    POPFD,
    PUSHF,
    CMOVNS,
    FPREM,
    FCOMI,
    SETNLE,
    FSTP,
    FXRSTOR,
    FLDCW,
    CDQ,
    POPA,
    CVTPI2PS,
    ENTER,
    MOVBE,
    RDTSC,
    FCOS,
    SETG,
    FYL2X,
    FCMOVE,
    MOVDQ2Q,
    OUTS,
    ARPL,
    INSD,
    CLFLUSHOPT,
    LIDT,
    PREFETCHT2,
    SETNA,
    JNS,
    OR,
    LDS,
    SFENCE,
    SHRD,
    STOSD,
    SYSENTER,
    XGETBV,
    CMOVNP,
    XRSTORS,
    CWD,
    FUCOMPP,
    FIDIV,
    BT,
    FADDP,
    SCASW,
    SUB,
    SLDT,
    UD0,
    UD1,
    FILD,
    SHLD,
    XOR,
    XRSTORS64,
    MOVS,
    CMOVNB,
    BTC,
    LOOPNE,
    FDIVP,
    FUCOMP,
    FDECSTP,
    INSW,
    CVTTPD2PI,
    FPREM1,
    CMOVAE,
    IRET,
    JNL,
    FXSAVE,
    JNLE,
    FINCSTP,
    BTS,
    LOOP,
    PREFETCHT0,
    SETAE,
    JNC,
    SETNC,
    FTST,
    IDIV,
    FCMOVNB,
    FCOMPP,
    JL,
    STD,
    OUTSW,
    STOSB,
    PTWRITE,
    FBLD,
    FMUL,
    CMOVNA,
    PUSHA,
    LODS,
    FLDLG2,
    SYSRET,
    FSUBRP,
    CVTTPS2PI,
    LFENCE,
    CMOVGE,
    FDIVR,
    POPAD,
    RDMSR,
    SCAS,
    JECXZ,
    FINIT,
    XSAVE64,
    POP,
    SAHF,
    SETB,
    SHL,
    CMOVNE,
    HLT,
    JAE,
    CMOVC,
    SGDT,
    FIADD,
    CMOVNBE,
    F2XM1,
    FPATAN,
    JNAE,
    MASKMOVQ,
    CMOVL,
    SCASD,
    FCOMP,
    DIV,
    FISUB,
    CMOVNLE,
    FNCLEX,
    FFREE,
    JCXZ,
    POPCNT,
    CMPXCHG16B,
    FNOP,
    FSCALE,
    FNSTENV,
    CMOVPE,
    STI,
    FNSAVE,
    AAM,
    CMOVBE,
    INVD,
    CMPSQ,
    FRSTOR,
    CMOVNO,
    CMOVNL,
    FXRSTOR64,
    FABS,
    CPUID,
    MUL,
    PREFETCHNTA,
    CMOVNC,
    PUSHFD,
    CLI,
    FUCOM,
    SETLE,
    CMOVNG,
    JO,
    DAS,
    ADD,
    CMOVG,
    BSR,
    LODSW,
    OUTSB,
    POPFQ,
    CMOVNAE,
    RCL,
    ROL,
    FCMOVB,
    SETL,
    SETNG,
    SETC,
    XSAVEC,
    JNP,
    XSAVEC64,
    LEA,
    LGDT,
    CRC32,
    CVTPI2PD,
    FXCH,
    NOT,
    JB,
    PUSHFQ,
    LODSQ,
    RET,
    JPE,
    SETE,
    SWAPGS,
    XSAVES64,
    FSAVE,
    MOVSD,
    JLE,
    LOCK,
    SBB,
    IRETQ,
    LTR,
    FWAIT,
    WBINVD,
    FSUBP,
    FLDL2T,
    XADD,
    CMP,
    JNZ,
    FIDIVR,
    CMPSW,
    FSTENV,
    CDQE,
    FISTP,
    FST,
    DEC,
    FCMOVU,
    EMMS,
    JNG,
    LSL,
    MOVSW,
    SETNL,
    FNSTSW,
    CLD,
    FBSTP,
    LOOPE,
    CMOVP,
    FICOM,
    FSTCW,
    INVLPG,
    JC,
    JNO,
    LLDT,
    SAR,
    LMSW,
    CVTPD2PI,
    CMOVNZ,
    MOV,
    FCHS,
    PSHUFW,
    OUT,
    SETNGE,
}
