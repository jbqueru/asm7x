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
    BeforeParameter,
    InParameter,
    AfterParameter,
    InComment,
}

struct CodeLine {
    label: String,
    instruction: String,
    parameters: Vec<String>,
}

fn main() {
    use crate::ParserState::*;

    println!("asm7x version 0.0.a20221029");

    let source = concat!("\t.org $8000\nreset:\n\tLDX\t#$FF\n\t",'\n');

    let mut parsed_file : Vec<CodeLine> = Vec::new();
    let mut parsed_line = CodeLine { label: String::from(""), instruction: String::from(""), parameters: Vec::new() };

    let mut source_line = 1;
    let mut source_column = 1;

    let mut parser_state = LineStart;

    let mut token = String::from("");

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
            BeforeParameter => {
                println!("before parameter");
            },
            InParameter => {
                println!("in parameter");
            },
            AfterParameter => {
                println!("after parameter");
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
                    // do nothing, empty line
                } else if current_char == '.' || current_char == '_' || (current_char.is_ascii() && current_char.is_alphabetic()) {
                    println!("starting label");
                    token.push(current_char);
                    parser_state = InLabel;
                } else {
                    println!("ERROR invalid label character at line {} column {}", source_line, source_column);
                    return;
                }
            },
            InLabel => {
                if current_char == ':' {
                    println!("label: {}", token);
                    parsed_line.label = token;
                    token = String::from("");
                    println!("end of label, waiting for instruction");
                    parser_state = BeforeInstruction;
                } else if current_char == '_' || (current_char.is_ascii() && current_char.is_alphanumeric()) {
                    token.push(current_char);
                } else {
                    println!("ERROR invalid label character at line {} column {}", source_line, source_column);
                    return;
                }
            },
            BeforeInstruction => {
                if current_char == ';' {
                    println!("starting comment");
                    parser_state = InComment;
                } else if current_char == ' ' || current_char == '\t' {
                    // do nothing, still waiting for instruction
                } else if current_char == '\n' {
                    println!("end of line");
                    println!("parsed line: label: {} instruction: {}", parsed_line.label, parsed_line.instruction);
                    parsed_file.push(parsed_line);
                    parsed_line = CodeLine { label: String::from(""), instruction: String::from(""), parameters: Vec::new() };
                    parser_state = LineStart;
                } else if current_char == '.' || (current_char.is_ascii() && current_char.is_alphabetic()) {
                    println!("starting instruction");
                    token.push(current_char);
                    parser_state = InInstruction;
                } else {
                    println!("ERROR invalid instruction character at line {} column {}", source_line, source_column);
                    return;
                }
            },
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
                    println!("parsed line: label: {} instruction: {}", parsed_line.label, parsed_line.instruction);
                    parsed_file.push(parsed_line);
                    parsed_line = CodeLine { label: String::from(""), instruction: String::from(""), parameters: Vec::new() };
                    token = String::from("");
                    println!("end of line");
                    parser_state = LineStart;
                } else if current_char == '.' || (current_char.is_ascii() && current_char.is_alphanumeric()) {
                    token.push(current_char);
                } else {
                    println!("ERROR invalid instruction character at line {} column {}", source_line, source_column);
                    return;
                }
            },
            BeforeParameter => {
                if current_char == ';' {
                    println!("starting comment");
                    parser_state = InComment;
                } else if current_char == ' ' || current_char == '\t' {
                    // do nothing, still waiting for parameter
                } else if current_char == '\n' {
                    println!("end of line");
                    println!("parsed line: label: {} instruction: {}", parsed_line.label, parsed_line.instruction);
                    parsed_file.push(parsed_line);
                    parsed_line = CodeLine { label: String::from(""), instruction: String::from(""), parameters: Vec::new() };
                    parser_state = LineStart;
                } else {
                    // TODO: define which characters are legal for parameters
                    println!("starting parameter");
                    token.push(current_char);
                    parser_state = InParameter;
                }
            },
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
                    print!("parsed line: label: {} instruction: {}", parsed_line.label, parsed_line.instruction);
                    for p in &parsed_line.parameters {
                        print!(" parameter: {}", p);
                    }
                    println!("");
                    parsed_file.push(parsed_line);
                    parsed_line = CodeLine { label: String::from(""), instruction: String::from(""), parameters: Vec::new() };
                    token = String::from("");
                    println!("end of line");
                    parser_state = LineStart;
                } else {
                    token.push(current_char);
                    // TODO: define which characters are legal for parameters
                    // still in parameter
                }
            },
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
                    print!("parsed line: label: {} instruction: {}", parsed_line.label, parsed_line.instruction);
                    for p in &parsed_line.parameters {
                        print!(" parameter: {}", p);
                    }
                    parsed_file.push(parsed_line);
                    parsed_line = CodeLine { label: String::from(""), instruction: String::from(""), parameters: Vec::new() };
                    parser_state = LineStart;
                } else {
                    println!("ERROR invalid character after parameter at line {} column {}", source_line, source_column);
                    return;
                }
            },
            InComment => {
                if current_char == '\n' {
                    println!("new line, end comment");
                    print!("parsed line: label: {} instruction: {}", parsed_line.label, parsed_line.instruction);
                    for p in &parsed_line.parameters {
                        print!(" parameter: {}", p);
                    }
                    parsed_file.push(parsed_line);
                    parsed_line = CodeLine { label: String::from(""), instruction: String::from(""), parameters: Vec::new() };
                    parser_state = LineStart;
                } else {
                    // still in comment
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

    println!("");
    println!("source listing:");
    println!("");
    for line in &parsed_file {
        if !line.label.eq("") {
            print!("{}: ", line.label);
        } else {
            print!(" ");
        }
        if !line.instruction.eq("") {
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
    
}
