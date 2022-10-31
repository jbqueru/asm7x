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

fn main() {
    let source = String::from("\n\nJBQ:;test1\nabc:   ;test2\n;test3\n   ;test4\n");
    let mut assembler = Assembler::new(&source);
    assembler.parse_source();
}

struct SourceFile<'lt> {
    current: Option<char>,
    future: std::str::Chars<'lt>,
    line: u32,
    column: u32,
}

impl SourceFile<'_> {
    fn new(s: &String) -> SourceFile {
        let mut iter = s.chars();
        return SourceFile::<'_> {
            current: iter.next(),
            future: iter,
            line: 1,
            column: 1,
        };
    }

    fn peek(&self) -> Option<char> {
        return self.current;
    }

    fn advance(&mut self) {
        self.column += 1;
        if self.current.expect("Should not advance beyond EOF") == '\n' {
            self.line += 1;
            self.column = 1;
        }
        self.current = self.future.next();
    }

    fn is_eof(&self) -> bool {
        return self.current.is_none();
    }

    fn print_current(&self) {
        match self.current {
            None => print!("EOF at line {}", self.line),
            Some(c) => print!(
                "character '{}' at {}:{}",
                c.escape_default(),
                self.line,
                self.column
            ),
        }
    }
}

struct Assembler<'lt> {
    src: SourceFile<'lt>,
}

impl Assembler<'_> {
    fn new(s: &String) -> Assembler {
        return Assembler {
            src: SourceFile::new(s),
        };
    }

    fn parse_source(&mut self) {
        while !self.src.is_eof() {
            self.parse_line();
        }
        println!("");
    }

    fn parse_line(&mut self) {
        println!("parse_line");
        let label = lex_label(&mut self.src);
        if label.is_some() {
            println!("found label: {}", label.unwrap());
            skip_optional_space(&mut self.src);
            self.parse_after_label();
            return;
        }
        if skip_space(&mut self.src) {
            self.parse_after_label();
            return;
        }
        skip_optional_comment(&mut self.src);
    }

    fn parse_after_label(&mut self) {
        println!("parse_after_label");
        skip_optional_comment(&mut self.src);
    }
}

enum LabelLexerState {
    BeforeLabel,
    InLabel,
}

fn lex_label(src: &mut SourceFile) -> Option<String> {
    use crate::LabelLexerState::*;

    let mut state = LabelLexerState::BeforeLabel;
    let mut ret = String::from("");
    loop {
        print!("lex_label loop, state: ");
        match state {
            BeforeLabel => print!("before label, "),
            InLabel => print!("in label, "),
        }
        src.print_current();
        println!("");
        match state {
            BeforeLabel => match src.peek() {
                None => return None,
                Some(c) => match c {
                    'a'..='z' | 'A'..='Z' => {
                        ret.push(c);
                        src.advance();
                        state = InLabel;
                    }
                    _ => return None,
                },
            },
            InLabel => match src.peek() {
                None => panic!("unimplemented"),
                Some(c) => match c {
                    'a'..='z' | 'A'..='Z' => {
                        ret.push(c);
                        src.advance();
                    }
                    ':' => {
                        src.advance();
                        return Some(ret);
                    }
                    _ => {
                        panic!("unimplemented");
                    }
                },
            },
        }
    }
}

fn skip_space(src: &mut SourceFile) -> bool {
    print!("skip_space, ");
    src.print_current();
    println!("");
    match src.peek() {
        None => panic!("unimplemented"),
        Some(c) => match c {
            ' ' | '\t' => {
                src.advance();
                skip_optional_space(src);
                return true;
            }
            _ => return false,
        },
    }
}

fn skip_optional_space(src: &mut SourceFile) {
    loop {
        print!("skip_optional_spaces loop, ");
        src.print_current();
        println!("");
        match src.peek() {
            None => return,
            Some(c) => match c {
                ' ' | '\t' => src.advance(),
                _ => return,
            },
        }
    }
}

fn skip_optional_comment(src: &mut SourceFile) {
    print!("skip_optional_comment, ");
    src.print_current();
    println!("");
    match src.peek() {
        None => panic!("unimplemented"),
        Some(c) => match c {
            '\n' => {
                src.advance();
                return;
            }
            ';' => src.advance(),
            _ => panic!("unimplemented"),
        },
    }
    loop {
        print!("skip_optional_comment loop, ");
        src.print_current();
        println!("");
        match src.peek() {
            None => panic!("unimplemented"),
            Some(c) => match c {
                '\n' => {
                    src.advance();
                    return;
                }
                _ => src.advance(),
            },
        }
    }
}

