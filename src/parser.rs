use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub enum CommandType {
    CArithmetic,
    CPush,
    CPop,
    CLabel,
    CGoto,
    CIf,
    CFunction,
    CReturn,
    CCall,
    CComment,
}

pub struct Parser {
    pub stream: BufReader<std::fs::File>,
    pub now_line: String,
}

impl Parser {
    pub fn new(f: BufReader<std::fs::File>) -> Parser {
        Parser {
            stream: f,
            now_line: "".to_string(),
        }
    }

    pub fn advance(&mut self) -> usize {
        let mut buf = String::new();
        let bytes = self.stream.read_line(&mut buf).unwrap();
        self.now_line = buf.to_string().trim().replace("\n", "");
        bytes
    }

    pub fn has_more_commands(&mut self) -> bool {
        self.command_type().is_some()
    }

    pub fn command_type(&mut self) -> Option<CommandType> {
        if self.c_arithmetic() {
            return Some(CommandType::CArithmetic);
        } else if self.c_push() {
            return Some(CommandType::CPush);
        } else if self.c_pop() {
            return Some(CommandType::CPop);
        } else if self.c_label() {
            return Some(CommandType::CLabel);
        } else if self.c_goto() {
            return Some(CommandType::CGoto);
        } else if self.c_if() {
            return Some(CommandType::CIf);
        } else if self.c_function() {
            return Some(CommandType::CFunction);
        } else if self.c_return() {
            return Some(CommandType::CReturn);
        } else if self.c_call() {
            return Some(CommandType::CCall);
        } else if self.c_comment() {
            return Some(CommandType::CComment);
        }
        None
    }

    pub fn arg1(&self) -> Option<String> {
        if self.c_arithmetic() {
            let re = Regex::new(r"^(add|sub)").unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(1).unwrap().to_string());
        } else if self.c_push() {
            let re = Regex::new(
                r"^push\s+(local|argument|this|that|pointer|temp|constant|static)\s+(\d+)$",
            )
            .unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(1).unwrap().to_string());
        } else if self.c_pop() {
            let re = Regex::new(
                r"^pop\s+(local|argument|this|that|pointer|temp|constant|static)\s+(\d+)$",
            )
            .unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(1).unwrap().to_string());
        } else if self.c_label() {
            let re = Regex::new(r"^label\s+(.+)\s+(\d+)$").unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(1).unwrap().to_string());
        } else if self.c_goto() {
            let re = Regex::new(r"^goto\s+(.+)\s+(\d+)$").unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(1).unwrap().to_string());
        } else if self.c_if() {
            let re = Regex::new(r"^if\s+(.+)\s+(\d+)$").unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(1).unwrap().to_string());
        } else if self.c_function() {
            let re = Regex::new(r"^function\s+(.+)\s+(\d+)$").unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(1).unwrap().to_string());
        } else if self.c_return() {
            let re = Regex::new(r"^return\s+(.+)\s+(\d+)$").unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(1).unwrap().to_string());
        } else if self.c_call() {
            let re = Regex::new(r"^call\s+(.+)\s+(\d+)$").unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(1).unwrap().to_string());
        } else if self.c_comment() {
            return None;
        }
        None
    }

    pub fn arg2(&self) -> Option<u32> {
        if self.c_push() {
            let re = Regex::new(
                r"^push\s+(local|argument|this|that|pointer|temp|constant|static)\s+(\d+)$",
            )
            .unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(2).unwrap().to_string().parse::<u32>().unwrap());
        } else if self.c_pop() {
            let re = Regex::new(
                r"^pop\s+(local|argument|this|that|pointer|temp|constant|static)\s+(\d+)$",
            )
            .unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(2).unwrap().to_string().parse::<u32>().unwrap());
        } else if self.c_function() {
            let re = Regex::new(r"^function\s+(.+)\s+(\d+)$").unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(2).unwrap().to_string().parse::<u32>().unwrap());
        } else if self.c_call() {
            let re = Regex::new(r"^call\s+(.+)\s+(\d+)$").unwrap();
            let caps = re.captures(&self.now_line).unwrap();
            return Some(caps.at(2).unwrap().to_string().parse::<u32>().unwrap());
        }
        None
    }

    fn c_arithmetic(&self) -> bool {
        let re = Regex::new(r"^(add|sub)$").unwrap();
        re.is_match(&self.now_line)
    }

    fn c_push(&self) -> bool {
        let re = Regex::new(r"^push\s+(.+)\s+(\d+)$").unwrap();
        re.is_match(&self.now_line)
    }

    fn c_pop(&self) -> bool {
        let re = Regex::new(r"^pop\s+(.+)\s+(\d+)$").unwrap();
        re.is_match(&self.now_line)
    }

    fn c_label(&self) -> bool {
        let re = Regex::new(r"^label\s+(.+)\s+(\d+)$").unwrap();
        re.is_match(&self.now_line)
    }

    fn c_goto(&self) -> bool {
        let re = Regex::new(r"^goto\s+(.+)\s+(\d+)$").unwrap();
        re.is_match(&self.now_line)
    }

    fn c_if(&self) -> bool {
        let re = Regex::new(r"^if\s+(.+)\s+(\d+)$").unwrap();
        re.is_match(&self.now_line)
    }

    fn c_function(&self) -> bool {
        let re = Regex::new(r"^function\s+(.+)\s+(\d+)$").unwrap();
        re.is_match(&self.now_line)
    }

    fn c_return(&self) -> bool {
        let re = Regex::new(r"^return\s+(.+)\s+(\d+)$").unwrap();
        re.is_match(&self.now_line)
    }

    fn c_call(&self) -> bool {
        let re = Regex::new(r"^call\s+(.+)\s+(\d+)$").unwrap();
        re.is_match(&self.now_line)
    }

    fn c_comment(&self) -> bool {
        let re = Regex::new(r"^//.+$").unwrap();
        re.is_match(&self.now_line)
    }
}
