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
            Some(c) => print!("character '{}' at {}:{}", c.escape_default(), self.line, self.column),
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

    fn run(&mut self) {
        while !self.src.is_eof() {
            self.parse_line();
        }
        println!("");
    }

    fn parse_line(&mut self) {
        println!("parse_line");
        let label = lex_label(&mut self.src);
        match label {
            None => println!("no label found"),
            Some(l) => println!("found label: {}", l),
        }
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
                    },
                    // TODO - space, everything else
                    _ => {
                        panic!("unimplemented");
                    },
                }
            },
            InLabel => match src.peek() {
                None => panic!("unimplemented"),
                Some(c) => match c {
                    'a'..='z' | 'A'..='Z' => {
                        ret.push(c);
                        src.advance();
                    },
                    ':' => {
                        src.advance();
                        return Some(ret);
                    },
                    _ => {
                        panic!("unimplemented");
                    },
                }
            },
        }
    }
}

enum OldParserState {
    LineStart,
    InLabel,
    BeforeInstruction,
    InInstruction,
    BeforeParameter,
    InParameter,
    AfterParameter,
    InComment,
}

struct CodeLine {
    label: Option<String>,
    instruction: String,
    parameters: Vec<String>,
}

fn main() {
    main1();
}

fn main1() {
    let source = String::from("JBQ:");
    let mut assembler = Assembler::new(&source);

    assembler.run();

    main2();
}

fn main2() {
    use crate::OldParserState::*;

    println!("asm7x version 0.0.a20221029");

    let source = concat!("\t.org $8000\nreset:\n\tLDX\t#$00\n\tLDX\t#$FF\n", '\n');

    let mut parsed_file: Vec<CodeLine> = Vec::new();
    let mut parsed_line = CodeLine {
        label: Some(String::from("")),
        instruction: String::from(""),
        parameters: Vec::new(),
    };

    let mut source_line = 1;
    let mut source_column = 1;

    let mut parser_state = LineStart;

    let mut token = String::from("");

    for current_char in source.chars() {
        println!("");

        match parser_state {
            LineStart => {
                println!("at line start");
            }
            InLabel => {
                println!("in label");
            }
            BeforeInstruction => {
                println!("before instruction");
            }
            InInstruction => {
                println!("in instruction");
            }
            BeforeParameter => {
                println!("before parameter");
            }
            InParameter => {
                println!("in parameter");
            }
            AfterParameter => {
                println!("after parameter");
            }
            InComment => {
                println!("in comment");
            }
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
                    // do nothing, empty line
                } else if current_char == '.'
                    || current_char == '_'
                    || (current_char.is_ascii() && current_char.is_alphabetic())
                {
                    println!("starting label");
                    token.push(current_char);
                    parser_state = InLabel;
                } else {
                    println!(
                        "ERROR invalid label character at line {} column {}",
                        source_line, source_column
                    );
                    return;
                }
            }
            InLabel => {
                if current_char == ':' {
                    println!("label: {}", token);
                    parsed_line.label = Some(token);
                    token = String::from("");
                    println!("end of label, waiting for instruction");
                    parser_state = BeforeInstruction;
                } else if current_char == '_'
                    || (current_char.is_ascii() && current_char.is_alphanumeric())
                {
                    token.push(current_char);
                } else {
                    println!(
                        "ERROR invalid label character at line {} column {}",
                        source_line, source_column
                    );
                    return;
                }
            }
            BeforeInstruction => {
                if current_char == ';' {
                    println!("starting comment");
                    parser_state = InComment;
                } else if current_char == ' ' || current_char == '\t' {
                    // do nothing, still waiting for instruction
                } else if current_char == '\n' {
                    println!("end of line");
                    println!(
                        "parsed line: label: {} instruction: {}",
                        parsed_line.label.as_ref().unwrap(),
                        parsed_line.instruction
                    );
                    parsed_file.push(parsed_line);
                    parsed_line = CodeLine {
                        label: Some(String::from("")),
                        instruction: String::from(""),
                        parameters: Vec::new(),
                    };
                    parser_state = LineStart;
                } else if current_char == '.'
                    || (current_char.is_ascii() && current_char.is_alphabetic())
                {
                    println!("starting instruction");
                    token.push(current_char);
                    parser_state = InInstruction;
                } else {
                    println!(
                        "ERROR invalid instruction character at line {} column {}",
                        source_line, source_column
                    );
                    return;
                }
            }
            InInstruction => {
                if current_char == ';' {
                    println!("instruction: {}", token);
                    parsed_line.instruction = token;
                    token = String::from("");
                    println!("starting comment");
                    parser_state = InComment;
                } else if current_char == ' ' || current_char == '\t' {
                    println!("instruction: {}", token);
                    parsed_line.instruction = token;
                    token = String::from("");
                    println!("end of instruction, waiting for parameter");
                    parser_state = BeforeParameter;
                } else if current_char == '\n' {
                    println!("instruction: {}", token);
                    parsed_line.instruction = token;
                    println!(
                        "parsed line: label: {} instruction: {}",
                        parsed_line.label.as_ref().unwrap(),
                        parsed_line.instruction
                    );
                    parsed_file.push(parsed_line);
                    parsed_line = CodeLine {
                        label: Some(String::from("")),
                        instruction: String::from(""),
                        parameters: Vec::new(),
                    };
                    token = String::from("");
                    println!("end of line");
                    parser_state = LineStart;
                } else if current_char == '.'
                    || (current_char.is_ascii() && current_char.is_alphanumeric())
                {
                    token.push(current_char);
                } else {
                    println!(
                        "ERROR invalid instruction character at line {} column {}",
                        source_line, source_column
                    );
                    return;
                }
            }
            BeforeParameter => {
                if current_char == ';' {
                    println!("starting comment");
                    parser_state = InComment;
                } else if current_char == ' ' || current_char == '\t' {
                    // do nothing, still waiting for parameter
                } else if current_char == '\n' {
                    println!("end of line");
                    println!(
                        "parsed line: label: {} instruction: {}",
                        parsed_line.label.as_ref().unwrap(),
                        parsed_line.instruction
                    );
                    parsed_file.push(parsed_line);
                    parsed_line = CodeLine {
                        label: Some(String::from("")),
                        instruction: String::from(""),
                        parameters: Vec::new(),
                    };
                    parser_state = LineStart;
                } else {
                    // TODO: define which characters are legal for parameters
                    println!("starting parameter");
                    token.push(current_char);
                    parser_state = InParameter;
                }
            }
            InParameter => {
                if current_char == ';' {
                    println!("parameter: {}", token);
                    parsed_line.parameters.push(token);
                    token = String::from("");
                    println!("starting comment");
                    parser_state = InComment;
                } else if current_char == ',' {
                    println!("parameter: {}", token);
                    parsed_line.parameters.push(token);
                    token = String::from("");
                    println!("comma, waiting for parameter");
                    parser_state = BeforeParameter;
                } else if current_char == ' ' || current_char == '\t' {
                    println!("parameter: {}", token);
                    parsed_line.parameters.push(token);
                    token = String::from("");
                    println!("end of parameter, waiting for comma");
                    parser_state = AfterParameter;
                } else if current_char == '\n' {
                    println!("parameter: {}", token);
                    parsed_line.parameters.push(token);
                    print!(
                        "parsed line: label: {} instruction: {}",
                        parsed_line.label.as_ref().unwrap(),
                        parsed_line.instruction
                    );
                    for p in &parsed_line.parameters {
                        print!(" parameter: {}", p);
                    }
                    println!("");
                    parsed_file.push(parsed_line);
                    parsed_line = CodeLine {
                        label: Some(String::from("")),
                        instruction: String::from(""),
                        parameters: Vec::new(),
                    };
                    token = String::from("");
                    println!("end of line");
                    parser_state = LineStart;
                } else {
                    token.push(current_char);
                    // TODO: define which characters are legal for parameters
                    // still in parameter
                }
            }
            AfterParameter => {
                if current_char == ';' {
                    println!("starting comment");
                    parser_state = InComment;
                } else if current_char == ',' {
                    println!("comma between parameters");
                    parser_state = BeforeParameter;
                } else if current_char == ' ' || current_char == '\t' {
                    // still after parameter
                } else if current_char == '\n' {
                    println!("end of line");
                    print!(
                        "parsed line: label: {} instruction: {}",
                        parsed_line.label.as_ref().unwrap(),
                        parsed_line.instruction
                    );
                    for p in &parsed_line.parameters {
                        print!(" parameter: {}", p);
                    }
                    parsed_file.push(parsed_line);
                    parsed_line = CodeLine {
                        label: Some(String::from("")),
                        instruction: String::from(""),
                        parameters: Vec::new(),
                    };
                    parser_state = LineStart;
                } else {
                    println!(
                        "ERROR invalid character after parameter at line {} column {}",
                        source_line, source_column
                    );
                    return;
                }
            }
            InComment => {
                if current_char == '\n' {
                    println!("new line, end comment");
                    print!(
                        "parsed line: label: {} instruction: {}",
                        parsed_line.label.as_ref().unwrap(),
                        parsed_line.instruction
                    );
                    for p in &parsed_line.parameters {
                        print!(" parameter: {}", p);
                    }
                    parsed_file.push(parsed_line);
                    parsed_line = CodeLine {
                        label: Some(String::from("")),
                        instruction: String::from(""),
                        parameters: Vec::new(),
                    };
                    parser_state = LineStart;
                } else {
                    // still in comment
                }
            }
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

    println!("");
    println!("source listing:");
    println!("");
    for line in &parsed_file {
        if line.label.as_ref().unwrap() != "" {
            print!("{}: ", line.label.as_ref().unwrap());
        } else {
            print!(" ");
        }
        if line.instruction != "" {
            print!("{}", line.instruction);
        }
        let mut first_param = true;
        for p in &line.parameters {
            if first_param {
                print!(" ");
                first_param = false;
            } else {
                print!(",");
            }
            print!("{}", p);
        }
        println!("");
    }

    let mut addr = 0_u32;

    for line in &parsed_file {
        if line.instruction == ".org" {
            addr = lex_number(&line.parameters[0]);
            println!("address moved to {}", addr);
        }
        if line.instruction == "LDX" {
            println!("issuing LDX at {}", addr);
            addr += 1;
        }
    }
}

fn lex_number(str: &String) -> u32 {
    let mut radix = 10;
    let mut ret: u32 = 0;
    for current_char in str.chars() {
        if current_char == '$' {
            radix = 16;
        } else if current_char == '%' {
            radix = 2;
        } else {
            ret = ret * radix + current_char.to_digit(radix).unwrap();
        }
    }
    return ret;
}
