# asm7x
A (future) assembler primarily for 70s CPUs and games consoles built with those CPUs

## Basic grammar

```
source : line source
       | Ø

line : LABEL after_label
     | SPACES after_label
     | COMMENT '\n'
     | '\n'

after_label : optional_instruction optional_space optional_comment '\n'

optinal_instruction : INSTRUCTION optional_parameters
                    | Ø

optional_parameters : SPACE PARAMETER optional_space more_parameters
           | Ø

more_parameters : ',' optional_space PARAMETER optional_space more_parameters
                | Ø

optional_comment : COMMENT '\n'
                 | '\n'

optional_space : SPACES
               | Ø

```
