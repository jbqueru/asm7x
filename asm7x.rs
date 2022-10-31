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
    let mut source = String::from("");
    source.push_str("JBQ:\tTAX D2, D5\t\t;test1\n");
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
        let previous = self.current.expect("attempting to advance beyond EOF");
        self.current = self.future.next();
        if self.current.is_some() {
            if previous == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }

    fn is_eof(&self) -> bool {
        return self.current.is_none();
    }

    fn print_current(&self) {
        match self.current {
            None => print!("EOF at {}:{}", self.line, self.column),
            Some(c) => print!(
                "character '{}' at {}:{}",
                c.escape_default(),
                self.line,
                self.column
            ),
        }
    }

    fn print_location(&self) {
        print!("{}:{}:{}", "<stdin>", self.line, self.column);
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
        self.parse_optional_instruction();
        skip_optional_space(&mut self.src);
        skip_optional_comment(&mut self.src);
    }

    fn parse_optional_instruction(&mut self) {
        println!("parse_optional_instruction");
        let inst = lex_instruction(&mut self.src);
        if inst.is_some() {
            println!("found instruction: {}", inst.unwrap());
            if !skip_space(&mut self.src) {
                return;
            }
            self.parse_optional_parameters();
        }
        return;
    }

    fn parse_optional_parameters(&mut self) {
        println!("parse_optional_parameters");
        let param = lex_parameter(&mut self.src);
        if param.is_some() {
            println!("found parameter: {}", param.unwrap());
            skip_optional_space(&mut self.src);
            self.parse_more_parameters();
        }
    }

    fn parse_more_parameters(&mut self) {
        println!("parse_more_parameters");
        match self.src.peek() {
            None => {
                print!("unexpected end of file at ");
                self.src.print_location();
                println!("");
                panic!("unimplemented error handling");
            }
            Some(c) => match c {
                ',' => {
                    self.src.advance();
                    skip_optional_space(&mut self.src);
                }
                _ => {
                    return;
                }
            },
        }
        let param = lex_parameter(&mut self.src);
        match param {
            None => {
                print!("didn't find require parameter at ");
                self.src.print_location();
                println!("");
                panic!("unimplemented error handling");
            }
            Some(p) => {
                println!("found parameter: {}", p);
                skip_optional_space(&mut self.src);
                self.parse_more_parameters();
            }
        }
    }
}

enum LabelLexerState {
    BeforeLabel,
    InLabel,
}

fn lex_label(src: &mut SourceFile) -> Option<String> {
    use crate::LabelLexerState::*;

    let mut state = BeforeLabel;
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
                None => {
                    print!("unexpected end of file at ");
                    src.print_location();
                    println!("");
                    panic!("unimplemented error handling");
                }
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
                        print!("invalid label character at ");
                        src.print_location();
                        println!("");
                        panic!("unimplemented error handling");
                    }
                },
            },
        }
    }
}

enum InstructionLexerState {
    BeforeInstruction,
    InInstruction,
}

fn lex_instruction(src: &mut SourceFile) -> Option<String> {
    use crate::InstructionLexerState::*;

    let mut state = BeforeInstruction;
    let mut ret = String::from("");
    loop {
        print!("lex_instruction loop, state: ");
        match state {
            BeforeInstruction => print!("before instruction, "),
            InInstruction => print!("in instruction, "),
        }
        src.print_current();
        println!("");
        match state {
            BeforeInstruction => match src.peek() {
                None => return None,
                Some(c) => match c {
                    'a'..='z' | 'A'..='Z' => {
                        ret.push(c);
                        src.advance();
                        state = InInstruction;
                    }
                    _ => return None,
                },
            },
            InInstruction => match src.peek() {
                None => {
                    print!("unexpected end of file at ");
                    src.print_location();
                    println!("");
                    panic!("unimplemented error handling");
                }
                Some(c) => match c {
                    'a'..='z' | 'A'..='Z' | '0'..='9' => {
                        ret.push(c);
                        src.advance();
                    }
                    _ => {
                        return Some(ret);
                    }
                },
            },
        }
    }
}

enum ParameterLexerState {
    BeforeParameter,
    InParameter,
}

fn lex_parameter(src: &mut SourceFile) -> Option<String> {
    use crate::ParameterLexerState::*;

    let mut state = BeforeParameter;
    let mut ret = String::from("");
    loop {
        print!("lex_parameter loop, state: ");
        match state {
            BeforeParameter => print!("before parameter, "),
            InParameter => print!("in parameter, "),
        }
        src.print_current();
        println!("");
        match state {
            BeforeParameter => match src.peek() {
                None => return None,
                Some(c) => match c {
                    'a'..='z' | 'A'..='Z' => {
                        ret.push(c);
                        src.advance();
                        state = InParameter;
                    }
                    _ => return None,
                },
            },
            InParameter => match src.peek() {
                None => {
                    print!("unexpected end of file at ");
                    src.print_location();
                    println!("");
                    panic!("unimplemented error handling")
                }
                Some(c) => match c {
                    'a'..='z' | 'A'..='Z' | '0'..='9' => {
                        ret.push(c);
                        src.advance();
                    }
                    _ => {
                        return Some(ret);
                    }
                },
            },
        }
    }
}

// Skip spaces in a location where spaces are mandatory
// return whether spaces were skipped
fn skip_space(src: &mut SourceFile) -> bool {
    print!("skip_space, ");
    src.print_current();
    println!("");
    match src.peek() {
        None => {
            print!("unexpected end of file at ");
            src.print_location();
            println!("");
            panic!("unimplemented error handling");
        }
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
            None => {
                print!("unexpected end of file at ");
                src.print_location();
                println!("");
                panic!("unimplemented error handling");
            }
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
        None => {
            print!("unexpected end of file at ");
            src.print_location();
            println!("");
            panic!("unimplemented error handling");
        }
        Some(c) => match c {
            '\n' => {
                src.advance();
                return;
            }
            ';' => src.advance(),
            _ => {
                print!("expected comment or end of line at ");
                src.print_location();
                println!("");
                panic!("unimplemented error handling");
            }
        },
    }
    loop {
        print!("skip_optional_comment loop, ");
        src.print_current();
        println!("");
        match src.peek() {
            None => {
                print!("unexpected end of file at ");
                src.print_location();
                println!("");
                panic!("unimplemented error handling");
            }
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
