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
    println!("asm7x version 0.0.a20221028");

    let source = "begin:\n\torg $8000\n\tmove d0,d1\n";

    let mut source_line = 1;
    let mut source_column = 1;

    let mut parser_state = ParserState::LineStart;

    for current_char in source.chars() {
        println!("");

        match parser_state {
            ParserState::LineStart => {
                println!("at line start");
            },
            ParserState::InLabel => {
                println!("in label");
            },
            ParserState::BeforeInstruction => {
                println!("before instruction");
            },
            ParserState::InInstruction => {
                println!("in instruction");
            },
            ParserState::InComment => {
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
            ParserState::LineStart => {
                if current_char == ';' {
                    println!("starting comment");
                    parser_state = ParserState::InComment;
                } else if current_char == ' ' || current_char == '\t' {
                    println!("waiting for instruction");
                    parser_state = ParserState::BeforeInstruction;
                } else if current_char == '\n' {
                    println!("empty line");
                } else {
                    println!("starting label");
                    parser_state = ParserState::InLabel;
                }
            },
            ParserState::InLabel => {
                if current_char == ';' {
                    println!("starting comment");
                    parser_state = ParserState::InComment;
                } else if current_char == ' ' || current_char == '\t' {
                    println!("end of label, waiting for instruction");
                    parser_state = ParserState::BeforeInstruction;
                } else if current_char == '\n' {
                    println!("end of line");
                    parser_state = ParserState::LineStart;
                }
            },
            ParserState::BeforeInstruction => {
                if current_char == ';' {
                    println!("starting comment");
                    parser_state = ParserState::InComment;
                } else if current_char == ' ' || current_char == '\t' {
                    println!("still waiting for instruction");
                    parser_state = ParserState::BeforeInstruction;
                } else if current_char == '\n' {
                    println!("end of line");
                    parser_state = ParserState::LineStart;
                } else {
                    println!("starting instruction");
                    parser_state = ParserState::InInstruction;
                }
            },
            ParserState::InInstruction => {
                if current_char == ';' {
                    println!("starting comment");
                    parser_state = ParserState::InComment;
                } else if current_char == ' ' || current_char == '\t' || current_char == ',' {
                    println!("end of instruction, waiting for next instruction");
                    parser_state = ParserState::BeforeInstruction;
                } else if current_char == '\n' {
                    println!("end of line");
                    parser_state = ParserState::LineStart;
                }
            },
            ParserState::InComment => {
                if current_char == '\n' {
                    println!("new line, end comment");
                    parser_state = ParserState::LineStart;
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
}
