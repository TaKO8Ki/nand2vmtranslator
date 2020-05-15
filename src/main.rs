use nand2vmtranslator::{code_writer, parser};
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_name = &args[1];

    let f = File::open(input_file_name).unwrap();
    let f = BufReader::new(f);

    let mut parser = parser::Parser::new(f);
    let mut f = BufWriter::new(fs::File::create("Out.asm").unwrap());
    let mut code_writer = code_writer::CodeWriter { stream: f };

    loop {
        let bytes = parser.advance();
        if bytes == 0 {
            break;
        }

        if !parser.has_more_commands() {
            continue;
        }
        println!("line: {}", parser.now_line);
        println!("command_type: {:?}", parser.command_type());
        match parser.arg1() {
            Some(arg) => {
                println!("arg1: {}", arg);
                println!("{}", code_writer::write_arithmetic(arg))
            }
            None => println!("arg1: None"),
        }
        match parser.arg2() {
            Some(arg) => println!("arg2: {}\n", arg),
            None => println!("arg2: None\n"),
        }
    }
}
