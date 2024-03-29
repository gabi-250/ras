# `ras-x86-repr` - the internal representation of an x86 instruction

The `InstructionRepr` is the most important structure in this crate. It has the following components:

* an `InstructionEncoding`, which specifies how the instruction should be
  encoded (see [instruction.rs])
* a list of `OperandRepr`s (the representation of its operands - see
  [operand.rs]), and
* the assembly `Mode`s in which its encoding is possible

[instruction.rs]: https://github.com/gabi-250/ras/blob/master/ras-x86-repr/src/instruction.rs
[operand.rs]: https://github.com/gabi-250/ras/blob/master/ras-x86-repr/src/operand.rs

## x86 instruction encoding

An `InstructionEncoding` consists of one or more `EncodingBytecodes`, which
are extracted from the `Opcode` column of the [x86-csv]. Section `3.1.1.1 Opcode
Column in the Instruction Summary Table (Instructions without VEX Prefix)` of
the [Intel® 64 and IA-32 architectures software developer's manual volume 2]
describes these opcodes as follows:
> * `NP` - Indicates the use of `66/F2/F3` prefixes (beyond those already part
>   of the instructions opcode) are not allowed with the instruction. Such use
>   will either cause an invalid-opcode exception (#UD) or result in the
>   encoding for a different instruction.
> * `NFx` - Indicates the use of `F2/F3` prefixes (beyond those already part
>   of the instructions opcode) are not allowed with the instruction. Such use
>   will either cause an invalid-opcode exception (#UD) or result in the
>   encoding for a different instruction.
> * `REX.W` - Indicates the use of a `REX` prefix that affects operand size or
>   instruction semantics. The ordering of the REX prefix and other
>   optional/mandatory instruction prefixes are discussed Chapter 2. Note that
>   REX prefixes that promote legacy instructions to 64-bit behavior are not
>   listed explicitly in the opcode column.
> * `/digit` - A digit between 0 and 7 indicates that the `ModR/M` byte of the
>   instruction uses only the r/m (register or memory) operand. The reg field
>   contains the digit that provides an extension to the instruction's opcode.
> * `/r` - Indicates that the ModR/M byte of the instruction contains a
>   register operand and an r/m operand.
> * `cb`, `cw`, `cd`, `cp`, `co`, `ct` - A 1-byte (`cb`), 2-byte (`cw`),
>   4-byte (`cd`), 6-byte (`cp`), 8-byte (`co`) or 10-byte (`ct`) value
>   following the opcode. This value is used to specify a code offset and
>   possibly a new value for the code segment register.
> * `ib`, `iw`, `id`, `io` - A 1-byte (`ib`), 2-byte (`iw`), 4-byte (`id`) or
>   8-byte (`io`) immediate operand to the instruction that follows the
>   opcode, `ModR/M` bytes or scale-indexing bytes. The opcode determines if
>   the operand is a signed value. All words, doublewords and quadwords are
>   given with the low-order byte first.
> * `+rb`, `+rw`, `+rd`, `+ro` - Indicated the lower 3 bits of the opcode byte
>   is used to encode the register operand without a `ModR/M` byte. The
>   instruction lists the corresponding hexadecimal value of the opcode byte
>   with low 3 bits as `000b`. In non-64-bit mode, a register code, from 0
>   through 7, is added to the hexadecimal value of the opcode byte. In 64-bit
>   mode, indicates the four bit field of `REX.b` and `opcode[2:0]` field
>   encodes the register operand of the instruction. `+ro` is applicable only
>   in 64-bit mode. See Table `3-1` for the codes.
> * `+i` - A number used in floating-point instructions when one of the
>   operands is ST(i) from the FPU register stack. The number i (which can
>   range from 0 to 7) is added to the hexadecimal byte given at the left of
>   the plus sign to form a single opcode byte.

[x86-csv]: https://github.com/GregoryComer/x86-csv/tree/c638bbbaa17f0c81abaa7e84a968335c985542fa
[Intel® 64 and IA-32 architectures software developer's manual volume 2]: https://software.intel.com/content/www/us/en/develop/articles/intel-sdm.html
