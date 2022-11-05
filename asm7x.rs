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
    source.push_str("Reset:\tLDX\t#255\n");
    source.push_str("\tTXS\t\t;set up stack\n");
    source.push_str("\tLDA\t#0\n");
    source.push_str("\tSTA\t8192\n");
    let mut assembler = Assembler::new(&source);
    assembler.parse_source();
}

// Handling of source files, reading one character at a time
//
// knows the current character (if any), the remaining characaters,
// the source code location.
//
// TODO: Handle includes, i.e. nested source files/streams
//       that'll very probably require to take ownership of the characters
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

struct CodeLine {
    label: Option<String>,
    instruction: Option<Instruction>,
}

struct Instruction {
    mnemonic: String,
    parameter: Option<Number>,
}

enum Number {
    Immediate(i64),
    Address(i64),
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

    // Parse an entire source file
    //
    // A source file is made of lines, parse lines one at a time
    fn parse_source(&mut self) -> Vec<CodeLine> {
        let mut parsed_file: Vec<CodeLine> = Vec::new();
        while !self.src.is_eof() {
            let l = self.parse_line();
            if l.label.is_some() {
                println!("final label: {}", l.label.as_ref().unwrap());
            }
            if l.instruction.is_some() {
                let i = l.instruction.as_ref().unwrap();
                println!("final menmonic: {}", i.mnemonic);
                if i.parameter.is_some() {
                    match i.parameter.as_ref().unwrap() {
                        crate::Number::Immediate(p) => println!("final immediate: {}", p),
                        crate::Number::Address(p) => println!("final address: {}", p),
                    }
                }
            }
            parsed_file.push(l);
        }
        println!("");
        return parsed_file;
    }

    // Parse a line of source
    //
    // If line starts with a label, handle it and parse rest of line
    // If line starts with a space, skip it and parse rest of line
    // Otherwise, it must either be empty or a comment
    fn parse_line(&mut self) -> CodeLine {
        println!("parse_line");
        let mut ret = CodeLine {
            label: None,
            instruction: None,
        };
        ret.label = lex_label(&mut self.src);
        if ret.label.is_some() {
            println!("found label: {}", ret.label.as_ref().unwrap());
            skip_optional_space(&mut self.src);
            ret.instruction = self.parse_after_label();
            return ret;
        }
        if skip_space(&mut self.src) {
            ret.instruction = self.parse_after_label();
            return ret;
        }
        skip_optional_comment(&mut self.src);
        return ret;
    }

    // Parse the rest of the line after the optional label
    //
    // The actual instruction, followed by optional space, then optional comment
    fn parse_after_label(&mut self) -> Option<Instruction> {
        println!("parse_after_label");
        let ret = self.parse_instruction();
        skip_optional_space(&mut self.src);
        skip_optional_comment(&mut self.src);
        return ret;
    }

    // Parse the instruction on a line
    //
    // Look for the mnemonic, followed by the parameters
    fn parse_instruction(&mut self) -> Option<Instruction> {
        println!("parse_instruction");
        let inst = lex_instruction(&mut self.src);
        if inst.is_some() {
            let mut ret = Instruction {
                mnemonic: inst.unwrap(),
                parameter: None,
            };
            println!("found instruction: {}", ret.mnemonic);
            if !skip_space(&mut self.src) {
                return Some(ret);
            }
            ret.parameter = self.parse_parameters();
            return Some(ret);
        }
        return None;
    }

    fn parse_parameters(&mut self) -> Option<Number> {
        println!("parse_parameters");
        match self.src.peek() {
            None => {
                print!("unexpected end of file at ");
                self.src.print_location();
                println!("");
                panic!("unimplemented error handling");
            }
            Some(c) => match c {
                '#' => {
                    self.src.advance();
                    skip_optional_space(&mut self.src);
                    match lex_number(&mut self.src) {
                        None => return None,
                        Some(n) => {
                            println!("found immediate: {}", n);
                            return Some(Number::Immediate(n));
                        }
                    }
                }
                _ => match lex_number(&mut self.src) {
                    None => return None,
                    Some(n) => {
                        println!("found address: {}", n);
                        return Some(Number::Address(n));
                    }
                },
            },
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

enum NumberLexerState {
    BeforeNumber,
    InNumber,
}

fn lex_number(src: &mut SourceFile) -> Option<i64> {
    use crate::NumberLexerState::*;

    let mut state = BeforeNumber;
    let mut ret = 0_i64;
    loop {
        print!("lex_number loop, state: ");
        match state {
            BeforeNumber => print!("before number, "),
            InNumber => print!("in number, "),
        }
        src.print_current();
        println!("");
        match state {
            BeforeNumber => match src.peek() {
                None => return None,
                Some(c) => match c {
                    '0'..='9' => {
                        src.advance();
                        ret = ret * 10 + i64::from(c.to_digit(10).unwrap());
                        state = InNumber;
                    }
                    _ => return None,
                },
            },
            InNumber => match src.peek() {
                None => {
                    print!("unexpected end of file at ");
                    src.print_location();
                    println!("");
                    panic!("unimplemented error handling")
                }
                Some(c) => match c {
                    '0'..='9' => {
                        src.advance();
                        ret = ret * 10 + i64::from(c.to_digit(10).unwrap());
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
/*
enum ParameterType {
    Absolute,     // 2 types
    Arithmetic,   // 8 types
    Branch,       // 1 type
    Bit,          // 2 types
    CompareIndex, // 3 types
    Implied,      // 1 type (!)
    IncDec,       // 4 types
    Jmp,          // 2 types
    Jsr,          // 1 type
    LoadIndex,    // 5 types (LDX and LDY different)
    Shift,        // 5 types
    Store,        // 7 types
    StoreIndex,   // 3 types (LDX and LDY different)
}

struct ParameterInfo {
    ptype: ParameterType,
}

impl ParameterInfo {
    fn new(p: ParameterType) -> ParameterInfo {
        return ParameterInfo { ptype: p };
    }
}

fn prepare_parameter_info() {
    use crate::ParameterType::*;
    let mut pinfo = std::collections::HashMap::new();
    pinfo.insert("ADC", ParameterInfo::new(Arithmetic));
    pinfo.insert("AND", ParameterInfo::new(Arithmetic));
    pinfo.insert("ASL", ParameterInfo::new(Shift));
    pinfo.insert("BCC", ParameterInfo::new(Branch));
    pinfo.insert("BCS", ParameterInfo::new(Branch));
    pinfo.insert("BEQ", ParameterInfo::new(Branch));
    pinfo.insert("BIT", ParameterInfo::new(Bit));
    pinfo.insert("BMI", ParameterInfo::new(Branch));
    pinfo.insert("BNE", ParameterInfo::new(Branch));
    pinfo.insert("BPL", ParameterInfo::new(Branch));
    pinfo.insert("BRK", ParameterInfo::new(Implied));
    pinfo.insert("BVC", ParameterInfo::new(Branch));
    pinfo.insert("BVS", ParameterInfo::new(Branch));
    pinfo.insert("CLC", ParameterInfo::new(Implied));
    pinfo.insert("CLD", ParameterInfo::new(Implied));
    pinfo.insert("CLI", ParameterInfo::new(Implied));
    pinfo.insert("CLV", ParameterInfo::new(Implied));
    pinfo.insert("CMP", ParameterInfo::new(Arithmetic));
    pinfo.insert("CPX", ParameterInfo::new(CompareIndex));
    pinfo.insert("CPY", ParameterInfo::new(CompareIndex));
    pinfo.insert("DEC", ParameterInfo::new(IncDec));
    pinfo.insert("DEX", ParameterInfo::new(Implied));
    pinfo.insert("DEY", ParameterInfo::new(Implied));
    pinfo.insert("EOR", ParameterInfo::new(Arithmetic));
    pinfo.insert("INC", ParameterInfo::new(IncDec));
    pinfo.insert("INX", ParameterInfo::new(Implied));
    pinfo.insert("INY", ParameterInfo::new(Implied));
    pinfo.insert("JMP", ParameterInfo::new(Jmp));
    pinfo.insert("JSR", ParameterInfo::new(Jsr));
    pinfo.insert("LDA", ParameterInfo::new(Arithmetic));
    pinfo.insert("LDX", ParameterInfo::new(LoadIndex));
    pinfo.insert("LDY", ParameterInfo::new(LoadIndex));
    pinfo.insert("LSR", ParameterInfo::new(Shift));
    pinfo.insert("NOP", ParameterInfo::new(Implied));
    pinfo.insert("ORA", ParameterInfo::new(Arithmetic));
    pinfo.insert("PHA", ParameterInfo::new(Implied));
    pinfo.insert("PHP", ParameterInfo::new(Implied));
    pinfo.insert("PLA", ParameterInfo::new(Implied));
    pinfo.insert("PLP", ParameterInfo::new(Implied));
    pinfo.insert("ROL", ParameterInfo::new(Shift));
    pinfo.insert("ROR", ParameterInfo::new(Shift));
    pinfo.insert("RTI", ParameterInfo::new(Implied));
    pinfo.insert("RTS", ParameterInfo::new(Implied));
    pinfo.insert("SBC", ParameterInfo::new(Arithmetic));
    pinfo.insert("SEC", ParameterInfo::new(Implied));
    pinfo.insert("SED", ParameterInfo::new(Implied));
    pinfo.insert("SEI", ParameterInfo::new(Implied));
    pinfo.insert("STA", ParameterInfo::new(Store));
    pinfo.insert("STX", ParameterInfo::new(StoreIndex));
    pinfo.insert("STY", ParameterInfo::new(StoreIndex));
    pinfo.insert("TAX", ParameterInfo::new(Implied));
    pinfo.insert("TAY", ParameterInfo::new(Implied));
    pinfo.insert("TSX", ParameterInfo::new(Implied));
    pinfo.insert("TXA", ParameterInfo::new(Implied));
    pinfo.insert("TXS", ParameterInfo::new(Implied));
    pinfo.insert("TYA", ParameterInfo::new(Implied));
}
*/

/*
12345678901234567890123456789012345678901234567890123456789012345678901234567890
*/
