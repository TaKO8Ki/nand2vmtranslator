use crate::parser::CommandType;
use std::io::{BufWriter, Write};

pub struct CodeWriter {
    pub stream: BufWriter<std::fs::File>,
    pub label_num: usize,
    pub current_translated_file_name: String,
}

impl CodeWriter {
    pub fn new(f: BufWriter<std::fs::File>) -> CodeWriter {
        CodeWriter {
            stream: f,
            label_num: 0,
            current_translated_file_name: "".to_string(),
        }
    }

    pub fn write_arithmetic(&mut self, command: String) {
        if ["add", "sub", "and", "or"].contains(&command.as_str()) {
            self.write_binary_operation(command);
        } else if ["neg", "not"].contains(&command.as_str()) {
            self.write_unary_opition(command);
        } else if ["eq", "gt", "lt"].contains(&command.as_str()) {
            self.write_comp_operaton(command)
        }
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: String, index: String) {
        let index = index.parse().unwrap();

        match command {
            CommandType::CPush => {
                if segment.clone() == "constant" {
                    self.write_codes(format!(
                        "@{}\n\
                        D=A\n",
                        index
                    ));
                    self.write_push_from_d_register();
                } else if ["local", "argument", "this", "that"].contains(&segment.clone().as_str())
                {
                    self.write_push_from_virtual_segment(segment.clone(), index);
                } else if ["temp", "pointer"].contains(&segment.clone().as_str()) {
                    self.write_push_from_static_segment(segment.clone(), index);
                }
                if segment.clone() == "static" {
                    self.write_codes(format!(
                        "@{}.{}\n",
                        self.current_translated_file_name, index
                    ));
                    self.write_codes("D=M\n".to_string());
                    self.write_push_from_d_register();
                }
            }
            CommandType::CPop => {
                if ["local", "argument", "this", "that"].contains(&segment.as_str()) {
                    self.write_pop_from_virtual_segment(segment.clone(), index)
                } else if ["temp", "pointer"].contains(&segment.clone().as_str()) {
                    self.write_pop_from_static_segment(segment.clone(), index)
                }
                if segment.clone() == "static" {
                    self.write_pop_to_m_register();
                    self.write_codes(format!(
                        "'D=M\n\
                        @{}.{}\n",
                        self.current_translated_file_name, index
                    ));
                    self.write_codes("M=D".to_string());
                }
            }
            CommandType::CComment => (),
            CommandType::CArithmetic => (),
            CommandType::CGoto => (),
            CommandType::CLabel => (),
            CommandType::CIf => (),
            CommandType::CFunction => (),
            CommandType::CReturn => (),
            CommandType::CCall => (),
        }
    }

    fn write_push_from_static_segment(&mut self, segment: String, index: usize) {
        let base_address = match segment.as_str() {
            "temp" => 5,
            "pointer" => 3,
            _ => 0,
        };
        self.write_codes(format!("@{}\n", base_address,));
        for _ in 0..index {
            self.write_codes("A=A+1\n".to_string());
        }
        self.write_codes("D=M\n".to_string());
        self.write_push_from_d_register();
    }

    fn write_push_from_virtual_segment(&mut self, segment: String, index: usize) {
        let register_name = match segment.as_str() {
            "local" => "LCL",
            "argument" => "ARG",
            "this" => "THIS",
            "that" => "THAT",
            _ => "",
        };
        self.write_codes(format!(
            "@{}\n\
            A=M\n",
            register_name
        ));

        for _ in 0..index {
            self.write_codes("A=A+1\n".to_string());
        }
        self.write_codes("D=M\n".to_string());
        self.write_push_from_d_register();
    }

    fn write_pop_from_virtual_segment(&mut self, segment: String, index: usize) {
        let register_name = match segment.as_str() {
            "local" => "LCL",
            "argument" => "ARG",
            "this" => "THIS",
            "that" => "THAT",
            _ => "",
        };
        self.write_pop_to_m_register();
        self.write_codes(format!(
            "D=M\n\
            @{}\n\
            A=M\n",
            register_name
        ));
        for _ in 0..index {
            self.write_codes("A=A+1\n".to_string())
        }
        self.write_codes("M=D\n".to_string())
    }

    fn write_pop_from_static_segment(&mut self, segment: String, index: usize) {
        let base_address = match segment.as_str() {
            "temp" => 5,
            "pointer" => 3,
            _ => 0,
        };
        self.write_pop_to_m_register();
        self.write_codes(format!(
            "D=M\n\
            @{}\n",
            base_address
        ));
        for _ in 0..index {
            self.write_codes("A=A+1\n".to_string());
        }

        self.write_codes("M=D\n".to_string())
    }

    fn write_pop_to_m_register(&mut self) {
        self.stream
            .write(
                "@SP\n\
                M=M-1\n\
                A=M\n"
                    .as_bytes(),
            )
            .unwrap();
    }

    fn write_push_from_d_register(&mut self) {
        self.stream
            .write(
                "@SP\n\
                A=M\n\
                M=D\n\
                @SP\n\
                M=M+1\n"
                    .as_bytes(),
            )
            .unwrap();
    }

    fn write_binary_operation(&mut self, command: String) {
        self.write_pop_to_m_register();
        self.write_codes("D=M\n".to_string());
        self.write_pop_to_m_register();
        match command.as_str() {
            "add" => self.write_codes("D=D+M\n".to_string()),
            "sub" => self.write_codes("D=M-D\n".to_string()),
            "and" => self.write_codes("D=D&M\n".to_string()),
            "or" => self.write_codes("D=D|M\n".to_string()),
            _ => (),
        }
        self.write_push_from_d_register();
    }

    fn write_unary_opition(&mut self, command: String) {
        self.write_codes(
            "@SP\n\
            A=M-1"
                .to_string(),
        );
        if command == "neg" {
            self.write_codes("M=-M\n".to_string())
        } else if command == "not" {
            self.write_codes("M=!M\n".to_string())
        }
    }

    fn write_comp_operaton(&mut self, command: String) {
        self.write_pop_to_m_register();
        self.write_codes("D=M".to_string());
        self.write_pop_to_m_register();
        let l1 = self.new_label();
        let l2 = self.new_label();
        let comp_type = match command.as_str() {
            "eq" => "JEQ",
            "gt" => "JGT",
            "lt" => "JLT",
            _ => "",
        };
        self.write_codes(format!(
            "D=M-D\n\
            @{}\n\
            D;{}\n\
            D=0\n\
            @{}\n\
            0;JMP\n\
            ({})\n\
            D=-1\n\
            ({})\n",
            l1, comp_type, l2, l1, l2
        ));
        self.write_push_from_d_register();
    }

    fn write_codes(&mut self, command: String) {
        self.stream.write(command.as_bytes()).unwrap();
    }

    fn new_label(&mut self) -> String {
        self.label_num += 1;
        format!("LABEL{}\n", self.label_num)
    }

    pub fn set_file_name() {}

    pub fn close() {}
}
