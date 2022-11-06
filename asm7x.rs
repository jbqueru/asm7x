// Copyright 2022 Jean-Baptiste M. "JBQ" "Djaybee" Queru
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
    source.push_str(" processor 6502\n");
    source.push_str(" org 32752\n");
    source.push_str(" byte 78\n");
    source.push_str(" byte 69\n");
    source.push_str(" byte 83\n");
    source.push_str(" byte 26\n");
    source.push_str(" byte 2\n");
    source.push_str(" byte 1\n");
    source.push_str(" byte 1\n");
    source.push_str(" byte 0\n");
    source.push_str(" byte 0\n");
    source.push_str(" byte 0\n");
    source.push_str(" byte 0\n");
    source.push_str(" byte 0\n");
    source.push_str(" byte 0\n");
    source.push_str(" byte 0\n");
    source.push_str(" byte 0\n");
    source.push_str(" byte 0\n");
    source.push_str("Reset:\n");
    source.push_str("\tLDX\t#255\n");
    source.push_str("\tTXS\t\t;set up stack\n");
    source.push_str("\tCLD\n");
    source.push_str("\tSEI\n");
    source.push_str("\tBIT\t8194\n");
    source.push_str("\tBCS\t32773\n");
    source.push_str("\tBIT\t8194\n");
    source.push_str("\tBCS\t32778\n");
    source.push_str("\tLDA\t#0\n");
    source.push_str("\tSTA\t8192\n");
    source.push_str("\tSTA\t8193\n");
    source.push_str("\tLDA\t#63\n");
    source.push_str("\tSTA\t8198\n");
    source.push_str("\tLDA\t#0\n");
    source.push_str("\tSTA\t8198\n");
    source.push_str("\tLDA\t#26\n");
    source.push_str("\tSTA\t8199\n");
    source.push_str("\tJMP\t32806\n");
    source.push_str("\torg 65529\n");
    source.push_str("\tRTI\n");
    source.push_str(" byte 249\n");
    source.push_str(" byte 255\n");
    source.push_str(" byte 0\n");
    source.push_str(" byte 128\n");
    source.push_str(" byte 249\n");
    source.push_str(" byte 255\n");
    let mut assembler = Parser::new(&source);
    let parsed = assembler.parse_source();
    list(&parsed);
    assemble(&parsed);
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
    file: String,
}

impl SourceFile<'_> {
    fn new(s: &str) -> SourceFile {
        let mut iter = s.chars();
        SourceFile::<'_> {
            current: iter.next(),
            future: iter,
            line: 1,
            column: 1,
            file: String::from("<builtin>"),
        }
    }

    fn peek(&self) -> Option<char> {
        self.current
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
        self.current.is_none()
    }

    fn print_current(&self) {
        match self.current {
            None => print!("EOF at {}:{}:{}", self.file, self.line, self.column),
            Some(c) => print!(
                "character '{}' at {}:{}:{}",
                c.escape_default(),
                self.file,
                self.line,
                self.column
            ),
        }
    }

    fn print_location(&self) {
        print!("{}:{}:{}", self.file, self.line, self.column);
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

fn list(parsed: &Vec<CodeLine>) {
    for line in parsed {
        if let Some(l) = &line.label {
            print!("{}:", l);
        }
        print!(" ");
        if let Some(i) = &line.instruction {
            print!("{}", i.mnemonic);
            if let Some(p) = &i.parameter {
                match p {
                    crate::Number::Immediate(p) => print!(" #{}", p),
                    crate::Number::Address(p) => print!(" {}", p),
                }
            }
        }
        println!();
    }
    println!();
}

fn assemble(parsed: &Vec<CodeLine>) {
    let mut address = 0_u32;
    println!("#!/bin/bash");
    for line in parsed {
        if let Some(i) = &line.instruction {
            match i.mnemonic.as_str() {
                "byte" => {
                    if let Some(p) = &i.parameter {
                        match p {
                            crate::Number::Address(p) => {
                                match p {
                                    0..=255 => {
                                        println!("# emitting raw byte {} at {}", p, address);
                                        address += 1;
                                        println!("echo -en '\\x{:02x}'", p);
                                    },
                                    _ => {
                                        println!("invalid value for byte");
                                        panic!("unimplemented error handling")
                                    },
                                }
                            },
                            _ => {
                                println!("wrong parameter type for byte");
                                panic!("unimplemented error handling")
                            },
                        }
                    } else {
                        println!("missing parameter for byte");
                        panic!("unimplemented error handling")
                    }
                },
                "org" => {
                    if let Some(p) = &i.parameter {
                        match p {
                            crate::Number::Address(p) => {
                                match p {
                                    0..=65535 => {
                                        let newaddress = *p as u32;
                                        if address == 0 {
                                            println!("# setting origin to {}", newaddress);
                                            address = newaddress;
                                        } else if address < newaddress {
                                            println!("# advancing to {} ({} bytes)", p, newaddress - address);
                                            println!("for i in {{{}..{}}}", address, newaddress - 1);
                                            println!("do");
                                            println!("  echo -en '\\x{:02x}'", 0xEA);
                                            println!("done");
                                            address = newaddress;
                                        } else {
                                            println!("attempt to move origin backward");
                                            panic!("unimplemented error handling")
                                        }
                                    },
                                    _ => {
                                        println!("invalid address for org");
                                        panic!("unimplemented error handling")
                                    },
                                }
                            },
                            _ => {
                                println!("wrong parameter type for org");
                                panic!("unimplemented error handling")
                            },
                        }
                    } else {
                        println!("missing parameter for org");
                        panic!("unimplemented error handling")
                    }
                },
                "processor" => {
                    println!("# ignoring directive: {}", i.mnemonic);
                },
                "BCS" => {
                    if let Some(p) = &i.parameter {
                        match p {
                            crate::Number::Address(p) => {
                                match p {
                                    0..=65535 => {
                                        let destination = *p as u32;
                                        if destination > address || destination < address - 128 {
                                            println!("branch out of range for BCS");
                                            panic!("unimplemented error handling")
                                        }
                                        println!("# emitting BCS opcode 0x{:02X} at {}", 0xB0, address);
                                        address += 1;
                                        println!("# emitting BCS parameter {} at {}", destination + 256 - address, address);
                                        address += 1;
                                        println!("echo -en '\\x{:02x}\\x{:02x}'", 0xB0, destination + 256 - address);
                                    },
                                    _ => {
                                        println!("invalid parameter value for BCS");
                                        panic!("unimplemented error handling")
                                    },
                                }
                            },
                            _ => {
                                println!("wrong parameter type for BCS");
                                panic!("unimplemented error handling")
                            },
                        }
                    } else {
                        println!("missing parameter for BCS");
                        panic!("unimplemented error handling")
                    }
                },
                "BIT" => {
                    if let Some(p) = &i.parameter {
                        match p {
                            crate::Number::Address(p) => {
                                match p {
                                    0..=65535 => {
                                        println!("# emitting BIT opcode 0x{:02X} at {}", 0x2C, address);
                                        address += 1;
                                        println!("# emitting BIT parameter {} at {}", p, address);
                                        address += 2;
                                        println!("echo -en '\\x{:02x}\\x{:02x}\\x{:02x}'", 0x2C, p & 255, p >> 8);
                                    },
                                    _ => {
                                        println!("invalid parameter value for BIT");
                                        panic!("unimplemented error handling")
                                    },
                                }
                            },
                            _ => {
                                println!("wrong parameter type for BIT");
                                panic!("unimplemented error handling")
                            },
                        }
                    } else {
                        println!("missing parameter for BIT");
                        panic!("unimplemented error handling")
                    }
                },
                "BPL" => {
                    if let Some(p) = &i.parameter {
                        match p {
                            crate::Number::Address(p) => {
                                match p {
                                    0..=65535 => {
                                        let destination = *p as u32;
                                        if destination > address || destination < address - 128 {
                                            println!("branch out of range for BPL");
                                            panic!("unimplemented error handling")
                                        }
                                        println!("# emitting BPL opcode 0x{:02X} at {}", 0xD0, address);
                                        address += 1;
                                        println!("# emitting BPL parameter {} at {}", destination + 256 - address, address);
                                        address += 1;
                                        println!("echo -en '\\x{:02x}\\x{:02x}'", 0xD0, destination + 256 - address);
                                    },
                                    _ => {
                                        println!("invalid parameter value for BPL");
                                        panic!("unimplemented error handling")
                                    },
                                }
                            },
                            _ => {
                                println!("wrong parameter type for BPL");
                                panic!("unimplemented error handling")
                            },
                        }
                    } else {
                        println!("missing parameter for BPL");
                        panic!("unimplemented error handling")
                    }
                },
                "CLC" => {
                    if i.parameter.is_none() {
                        println!("# emitting CLC opcode 0x{:02X} at {}", 0x18, address);
                        address += 1;
                        println!("echo -en '\\x{:02x}'", 0x18);
                    } else {
                        println!("unexpected parameter for CLC");
                        panic!("unimplemented error handling")
                    }
                },
                "CLD" => {
                    if i.parameter.is_none() {
                        println!("# emitting CLD opcode 0x{:02X} at {}", 0xD8, address);
                        address += 1;
                        println!("echo -en '\\x{:02x}'", 0xD8);
                    } else {
                        println!("unexpected parameter for CLD");
                        panic!("unimplemented error handling")
                    }
                },
                "JMP" => {
                    if let Some(p) = &i.parameter {
                        match p {
                            crate::Number::Address(p) => {
                                match p {
                                    0..=65535 => {
                                        println!("# emitting JMP opcode 0x{:02X} at {}", 0x4C, address);
                                        address += 1;
                                        println!("# emitting JMP parameter {} at {}", p, address);
                                        address += 2;
                                        println!("echo -en '\\x{:02x}\\x{:02x}\\x{:02x}'", 0x4C, p & 255, p >> 8);
                                    },
                                    _ => {
                                        println!("invalid parameter value for JMP");
                                        panic!("unimplemented error handling")
                                    },
                                }
                            },
                            _ => {
                                println!("wrong parameter type for JMP");
                                panic!("unimplemented error handling")
                            },
                        }
                    } else {
                        println!("missing parameter for JMP");
                        panic!("unimplemented error handling")
                    }
                },
                "LDA" => {
                    if let Some(p) = &i.parameter {
                        match p {
                            crate::Number::Immediate(p) => {
                                match p {
                                    0..=255 => {
                                        println!("# emitting LDA opcode 0x{:02X} at {}", 0xA9, address);
                                        address += 1;
                                        println!("# emitting LDA parameter {} at {}", p, address);
                                        address += 1;
                                        println!("echo -en '\\x{:02x}\\x{:02x}'", 0xA9, p);
                                    },
                                    _ => {
                                        println!("invalid parameter value for LDA");
                                        panic!("unimplemented error handling")
                                    },
                                }
                            },
                            _ => {
                                println!("wrong parameter type for LDA");
                                panic!("unimplemented error handling")
                            },
                        }
                    } else {
                        println!("missing parameter for LDA");
                        panic!("unimplemented error handling")
                    }
                },
                "LDX" => {
                    if let Some(p) = &i.parameter {
                        match p {
                            crate::Number::Immediate(p) => {
                                match p {
                                    0..=255 => {
                                        println!("# emitting LDX opcode 0x{:02X} at {}", 0xA2, address);
                                        address += 1;
                                        println!("# emitting LDX parameter {} at {}", p, address);
                                        address += 1;
                                        println!("echo -en '\\x{:02x}\\x{:02x}'", 0xA2, p);
                                    },
                                    _ => {
                                        println!("invalid parameter value for LDX");
                                        panic!("unimplemented error handling")
                                    },
                                }
                            },
                            _ => {
                                println!("wrong parameter type for LDX");
                                panic!("unimplemented error handling")
                            },
                        }
                    } else {
                        println!("missing parameter for LDX");
                        panic!("unimplemented error handling")
                    }
                },
                "RTI" => {
                    if i.parameter.is_none() {
                        println!("# emitting RTI opcode 0x{:02X} at {}", 0x40, address);
                        address += 1;
                        println!("echo -en '\\x{:02x}'", 0x40);
                    } else {
                        println!("unexpected parameter for RTI");
                        panic!("unimplemented error handling")
                    }
                },
                "SEI" => {
                    if i.parameter.is_none() {
                        println!("# emitting SEI opcode 0x{:02X} at {}", 0x78, address);
                        address += 1;
                        println!("echo -en '\\x{:02x}'", 0x78);
                    } else {
                        println!("unexpected parameter for SEI");
                        panic!("unimplemented error handling")
                    }
                },
                "STA" => {
                    if let Some(p) = &i.parameter {
                        match p {
                            crate::Number::Address(p) => {
                                match p {
                                    0..=65535 => {
                                        println!("# emitting STA opcode 0x{:02X} at {}", 0x8D, address);
                                        address += 1;
                                        println!("# emitting STA parameter {} at {}", p, address);
                                        address += 2;
                                        println!("echo -en '\\x{:02x}\\x{:02x}\\x{:02x}'", 0x8D, p & 255, p >> 8);
                                    },
                                    _ => {
                                        println!("invalid parameter value for STA");
                                        panic!("unimplemented error handling")
                                    },
                                }
                            },
                            _ => {
                                println!("wrong parameter type for STA");
                                panic!("unimplemented error handling")
                            },
                        }
                    } else {
                        println!("missing parameter for STA");
                        panic!("unimplemented error handling")
                    }
                },
                "TXS" => {
                    if i.parameter.is_none() {
                        println!("# emitting TXS opcode 0x{:02X} at {}", 0x9A, address);
                        address += 1;
                        println!("echo -en '\\x{:02x}'", 0x9A);
                    } else {
                        println!("unexpected parameter for TXS");
                        panic!("unimplemented error handling")
                    }
                },
                _ => {
                    println!("unknown instruction: {}", i.mnemonic);
                    panic!("unimplemented error handling")
                },
            }
        }
    }
    println!();
}

struct Parser<'lt> {
    src: SourceFile<'lt>,
//    parsed: Vec<CodeLine>,
}

impl Parser<'_> {
    fn new(s: &str) -> Parser {
        return Parser {
            src: SourceFile::new(s),
//            parsed: Vec::new(),
        };
    }

    // Parse an entire source file
    //
    // A source file is made of lines, parse lines one at a time
    fn parse_source(&mut self) -> Vec<CodeLine> {
        let mut ret = Vec::new();
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
            ret.push(l);
        }
        println!();
        ret
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
        ret
    }

    // Parse the rest of the line after the optional label
    //
    // The actual instruction, followed by optional space, then optional comment
    fn parse_after_label(&mut self) -> Option<Instruction> {
        println!("parse_after_label");
        let ret = self.parse_instruction();
        skip_optional_space(&mut self.src);
        skip_optional_comment(&mut self.src);
        ret
    }

    // Parse the instruction on a line
    //
    // Look for the mnemonic, followed by the parameters
    fn parse_instruction(&mut self) -> Option<Instruction> {
        println!("parse_instruction");
        let inst = lex_instruction(&mut self.src);
        if let Some(i) = inst {
            let mut ret = Instruction {
                mnemonic: i,
                parameter: None,
            };
            println!("found instruction: {}", ret.mnemonic);
            if !skip_space(&mut self.src) {
                return Some(ret);
            }
            ret.parameter = self.parse_parameters();
            return Some(ret);
        }
        None
    }

    fn parse_parameters(&mut self) -> Option<Number> {
        println!("parse_parameters");
        match self.src.peek() {
            None => {
                print!("unexpected end of file at ");
                self.src.print_location();
                println!();
                panic!("unimplemented error handling");
            }
            Some(c) => match c {
                '#' => {
                    self.src.advance();
                    skip_optional_space(&mut self.src);
                    match lex_number(&mut self.src) {
                        None => None,
                        Some(n) => {
                            println!("found immediate: {}", n);
                            Some(Number::Immediate(n))
                        }
                    }
                }
                _ => match lex_number(&mut self.src) {
                    None => None,
                    Some(n) => {
                        println!("found address: {}", n);
                        Some(Number::Address(n))
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
        println!();
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
                    println!();
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
                        println!();
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
        println!();
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
                    println!();
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

/*
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
        println!();
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
                    println!();
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
*/

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
        println!();
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
                    println!();
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
    println!();
    match src.peek() {
        None => {
            print!("unexpected end of file at ");
            src.print_location();
            println!();
            panic!("unimplemented error handling");
        }
        Some(c) => match c {
            ' ' | '\t' => {
                src.advance();
                skip_optional_space(src);
                true
            }
            _ => false,
        },
    }
}

fn skip_optional_space(src: &mut SourceFile) {
    loop {
        print!("skip_optional_spaces loop, ");
        src.print_current();
        println!();
        match src.peek() {
            None => {
                print!("unexpected end of file at ");
                src.print_location();
                println!();
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
    println!();
    match src.peek() {
        None => {
            print!("unexpected end of file at ");
            src.print_location();
            println!();
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
                println!();
                panic!("unimplemented error handling");
            }
        },
    }
    loop {
        print!("skip_optional_comment loop, ");
        src.print_current();
        println!();
        match src.peek() {
            None => {
                print!("unexpected end of file at ");
                src.print_location();
                println!();
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
12345678901234567890123456789012345678901234567890123456789012345678901234567890
*/
