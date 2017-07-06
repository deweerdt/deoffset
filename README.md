`deoffset` is a utility that makes it easier to compare disassembled
binaries by removing offsets from the output of `objdump -d`.

Doing so makes it practical to compare the binaries side by side:
```shell
$ diff -W 240 -y {old,new}
```
