# asm7x
A (future) assembler primarily for 70s CPUs and games consoles built with those CPUs

## Basic grammar

```
source : line source
       | Ø

line : LABEL OPTIONAL_SPACE after_label
     | SPACE after_label
     | OPTIONAL_COMMENT

after_label : optional_instruction OPTIONAL_SPACE OPTIONAL_COMMENT

optinal_instruction : INSTRUCTION optional_parameters
                    | Ø

optional_parameters : SPACE PARAMETER OPTIONAL_SPACE more_parameters
           | Ø

more_parameters : ',' OPTIONAL_SPACE PARAMETER OPTIONAL_SPACE more_parameters
                | Ø
```
