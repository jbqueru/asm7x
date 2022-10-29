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
    LineBegin,
    Comment,
}

fn main() {
    println!("asm7x version 0.0.a20221028"); 

    let source = "Hello, World!\n;\tThis is Djaybee.\n";

    let mut source_line = 1;
    let mut source_column = 1;

    let mut parser_state = ParserState::LineBegin;

    for current_char in source.chars() {
        print!("character {}", current_char.escape_unicode());
        if current_char.is_ascii() && !current_char.is_control() {
            print!(" [{}]", current_char.to_string());
        }
        print!(" at line {} column {}", source_line, source_column);
        println!("");

        match parser_state {
            ParserState::LineBegin => todo!(),
            ParserState::Comment => {
                if current_char == '\n' {
                    println!("end comment");
                } else {
                    println!("in comment")
                }
            },
        }

        if current_char == ';' {
            parser_state = ParserState::Comment;
            println!("starting comment");
        }

        if current_char == '\n' {
            source_line += 1;
            source_column = 1;
            parser_state = ParserState::LineBegin;
            println!("new line");
        } else if current_char == '\t' {
            source_column = source_column - source_column % 8 + 9;
        } else {
            source_column += 1;
        }
    }
}
