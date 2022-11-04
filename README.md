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

optional_instruction : INSTRUCTION parameters
                    | Ø

parameters : expression_list
           | Ø

expression_list : expression
                | expression ',' expression

expression : '#' sum
           | sum

sum : product
    | product '+' product

product : operand
        | operand '*' operand

operand : IDENTIFIER
        | NUMBER
        | '(' expression_list ')'

```
