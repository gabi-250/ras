# ras-x86

## `Mnemonic` and instruction map generation

The `Mnemonic` enum containing all the x86-64 mnemonics is generated by a build
script, which uses Gregory Comer's [x86-csv]. The build script also emits
`inst_map.json`, a serialized mapping of mnemonics to instruction encodings.

[x86-csv]: https://github.com/GregoryComer/x86-csv
