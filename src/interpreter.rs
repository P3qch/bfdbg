/*
MIT License

Copyright (c) 2021 P3qch

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
const LEGAL_INSTRUCTIONS: [char; 9] = ['+', '-', '>', '<', '[', ']', '.', ',', '#'];

pub struct Interpreter {
    pub tape: [u8; 30_000],
    data_pointer: usize,
    src: String,
    pub output: String,
    current: usize,
    pub parens: Vec<usize>,
    previous_cell: usize,
    inner_loops: usize,
    looping: bool
}

impl Interpreter {
    pub fn new(src: String) -> Self {
        Interpreter {
            tape: [0u8; 30_000],
            data_pointer: 0,
            src,
            current: 0,
            parens: vec![],
            output: String::new(),
            previous_cell: 0,
            inner_loops: 0,
            looping: false,
        }
    }

    pub fn run(&mut self) {
        while !self.finished() && self.current_instruction() != '#' {
            self.step();
        }
    }

    pub fn step(&mut self) {

        if self.looping {
            
            if self.current_instruction() == '[' {
                self.inner_loops += 1;
            }
            if self.current_instruction() == ']' {
                if self.inner_loops == 0 {
                    self.looping = false;
                }
                else {
                    self.inner_loops -= 1;
                }
            }
            self.advance();
            return;
        }

        match self.advance() {
            '>' => {
                if self.data_pointer < 30_000 {
                    self.data_pointer += 1;
                } else {
                    self.data_pointer = 0;
                }
            }
            '<' => {
                if self.data_pointer != 0 {
                    self.data_pointer -= 1;
                } else {
                    self.data_pointer = 30_000;
                }
            }
            '+' => {
                let cell = self.current_cell();
                *cell = cell.wrapping_add(1u8);
            }
            '-' => {
                let cell = self.current_cell();
                *cell = cell.wrapping_sub(1u8);
            }
            '[' => {
                if self.peek_cell() == 0u8 {
                    self.looping = true;
                } else {
                    self.parens.push(self.last_legal_inst());
                }
            }
            ']' => {
                if self.peek_cell() != 0u8 {
                    self.current = self.parens.pop().expect("Expected '[' before ']'");
                } else {
                    self.parens.pop().unwrap();
                }
            }
            '.' => {
                self.output.push(self.peek_char() as char);
            }
            ',' => {
                // TODO: MAKE THIS SHIT WORKY
            }
            _ => (),
        };
    }

    pub fn get_source_with_inst_pointer(&self) -> (&str, usize) {
        (self.src.as_ref(), self.current)
    }


    pub fn get_range_from_tape(&self, from: usize, to: usize) -> (Vec<u8>, usize) {
        let mut result = vec![];

        for i in from..to {
            result.push(self.tape[i]);
        }

        (result, self.data_pointer.wrapping_sub(from))
    }

    pub fn finished(&self) -> bool {
        self.src.len() -1 == self.current 
    }

    fn last_legal_inst(&self) -> usize {
        let mut result = self.current - 1;


        while !LEGAL_INSTRUCTIONS.contains(&self.peek_inst(result)) {
            result -= 1;
        }


        result
    }

    fn advance(&mut self) -> char {
        if self.finished() {
            
            return '\0';
        }
        let result = self.current_instruction();
        self.previous_cell = self.current;
        self.current += 1;

        while !LEGAL_INSTRUCTIONS.contains(&self.current_instruction()) && !self.finished() {
            self.current += 1;
        }
        return result;
    }

    fn current_instruction(&self) -> char {
        self.peek_inst(self.current)
    }

    fn current_cell(&mut self) -> &mut u8 {
        &mut self.tape[self.data_pointer]
    }

    fn peek_inst(&self, index: usize) -> char {
        self.src.as_bytes()[index] as char
    }

    fn peek_char(&self) -> char {
        self.tape[self.data_pointer] as char
    }

    fn peek_cell(&self) -> u8 {
        self.tape[self.data_pointer]
    }
}
