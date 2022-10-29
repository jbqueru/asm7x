// Copyright 2022 Jean-Baptiste "JBQ" "Djaybee" Queru
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

enum ParserState {
    LineStart,
    InLabel,
    BeforeInstruction,
    InInstruction,
    InComment,
}

fn main() {
    use crate::ParserState::*;

    println!("asm7x version 0.0.a20221028");

    let source = "begin:\n\torg $8000\n\tmove d0,d1\n";

    let mut source_line = 1;
    let mut source_column = 1;

    let mut parser_state = LineStart;

    for current_char in source.chars() {
        println!("");

        match parser_state {
            LineStart => {
                println!("at line start");
            },
            InLabel => {
                println!("in label");
            },
            BeforeInstruction => {
                println!("before instruction");
            },
            InInstruction => {
                println!("in instruction");
            },
            InComment => {
                println!("in comment");
            },
        }

        print!("character {}", current_char.escape_unicode());
        if current_char.is_ascii() && !current_char.is_control() {
            print!(" [{}]", current_char.to_string());
        }
        print!(" at line {} column {}", source_line, source_column);
        println!("");

        match parser_state {
            LineStart => {
                if current_char == ';' {
                    println!("starting comment");
                    parser_state = InComment;
                } else if current_char == ' ' || current_char == '\t' {
                    println!("waiting for instruction");
                    parser_state = BeforeInstruction;
                } else if current_char == '\n' {
                    println!("empty line");
                } else if current_char == '.' || current_char == '_' || (current_char.is_ascii() && current_char.is_alphabetic()) {
                    println!("starting label");
                    parser_state = InLabel;
                } else {
                    println!("ERROR invalid label character at line {} column {}", source_line, source_column);
                    return;
                }
            },
            InLabel => {
                if current_char == ';' {
                    println!("starting comment");
                    parser_state = InComment;
                } else if current_char == ':' {
                    println!("end of label, waiting for instruction");
                    parser_state = BeforeInstruction;
                } else if current_char == '\n' {
                    println!("end of line");
                    parser_state = LineStart;
                } else if !(current_char == '_' || (current_char.is_ascii() && current_char.is_alphanumeric())) {
                    println!("ERROR invalid label character at line {} column {}", source_line, source_column);
                    return;
                }
            },
            BeforeInstruction => {
                if current_char == ';' {
                    println!("starting comment");
                    parser_state = InComment;
                } else if current_char == ' ' || current_char == '\t' {
                    println!("still waiting for instruction");
                    parser_state = BeforeInstruction;
                } else if current_char == '\n' {
                    println!("end of line");
                    parser_state = LineStart;
                } else {
                    println!("starting instruction");
                    parser_state = InInstruction;
                }
            },
            InInstruction => {
                if current_char == ';' {
                    println!("starting comment");
                    parser_state = InComment;
                } else if current_char == ' ' || current_char == '\t' || current_char == ',' {
                    println!("end of instruction, waiting for next instruction");
                    parser_state = BeforeInstruction;
                } else if current_char == '\n' {
                    println!("end of line");
                    parser_state = LineStart;
                }
            },
            InComment => {
                if current_char == '\n' {
                    println!("new line, end comment");
                    parser_state = LineStart;
                }
            },
        }

        if current_char == '\n' {
            source_line += 1;
            source_column = 1;
        } else if current_char == '\t' {
            source_column = source_column - source_column % 8 + 9;
        } else {
            source_column += 1;
        }
    }

    if !matches!(parser_state, LineStart) {
        println!("");
        println!("ERROR: incomplete line at end of source file");
    }
}
