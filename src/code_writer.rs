use std::io::{BufReader, Write};

pub struct CodeWriter {
    stream: BufReader<std::fs::File>,
}

impl CodeWriter {
    pub fn new(f: BufReader<std::fs::File>) -> CodeWriter {
        CodeWriter { stream: f }
    }
    pub fn write_arithmetic(&self) -> String {
        self.write_pop_to_m_register();
        if self.command == "add" {
            return;
        }
    }

    fn write_pop_to_m_register(&self) {
        self.stream
            .write(
                "@SP\n\
                M=M-1\n\
                A=M\n"
                    .to_string(),
            )
            .unwrap();
    }

    fn write_push_from_d_register(&self) -> String {
        "@SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n"
            .to_string()
    }

    pub fn set_file_name() {}

    pub fn write_push_pop() {}

    pub fn close() {}
}
