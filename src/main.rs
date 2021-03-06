use nand2vmtranslator::{code_writer, parser};
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_name = &args[1];

    let f = File::open(input_file_name).unwrap();
    let f = BufReader::new(f);

    let mut parser = parser::Parser::new(f);
    let re = Regex::new(r"^(.+)\.vm$").unwrap();
    let caps = re.captures(input_file_name).unwrap();
    let f = BufWriter::new(File::create(format!("tmp/{}.asm", caps.at(1).unwrap())).unwrap());
    let mut code_writer = code_writer::CodeWriter::new(f);

    println!("Start translating!");
    loop {
        let bytes = parser.advance();
        if bytes == 0 {
            break;
        }

        if !parser.has_more_commands() {
            continue;
        }

        match parser.command_type().unwrap() {
            parser::CommandType::CArithmetic => {
                code_writer.write_arithmetic(parser.arg1().unwrap())
            }
            parser::CommandType::CPush => code_writer.write_push_pop(
                parser::CommandType::CPush,
                parser.arg1().unwrap(),
                parser.arg2().unwrap().to_string(),
            ),
            parser::CommandType::CPop => code_writer.write_push_pop(
                parser::CommandType::CPop,
                parser.arg1().unwrap(),
                parser.arg2().unwrap().to_string(),
            ),
            parser::CommandType::CComment
            | parser::CommandType::CLabel
            | parser::CommandType::CGoto
            | parser::CommandType::CIf
            | parser::CommandType::CFunction
            | parser::CommandType::CReturn
            | parser::CommandType::CCall => (),
        };
    }
    println!("Finished!")
}
