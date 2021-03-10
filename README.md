# About __nabby__
nabby is a __Work in progress, partial implementation__ of the [NES spec](http://wiki.nesdev.com/w/index.php/NES_reference_guide).

## Instructions implemented:

| Instruction | Bytecode | Description | Version Introduced |
| ----------- | -------- | ----------- | ------------------ |
| NOP | 00000000 | No operation | 0.1 |
| JMP | 1000LLHH | Sets instruction pointer to HHLL | 0.1 |
| JMC | 1100LLHH | Jump to address if carry flag is raised | 0.1 |
