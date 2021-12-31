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

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use tui::{Frame, Terminal, backend::CrosstermBackend, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::{Span, Spans}, widgets::{Block, Borders, Paragraph}};

use crate::interpreter::Interpreter;

use std::{io, time::Duration};

pub struct Ui {
    run: bool,
    interpreter: Interpreter,
    memory_start: usize,
    output_line: usize,
}

impl Ui {
    pub fn new(interpreter: Interpreter) -> io::Result<Self> {


        Ok(Ui {
            run: true,
            interpreter,
            memory_start: 0,
            output_line: 0,
        })
    }

    pub fn run_app(&mut self) -> io::Result<()> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        enable_raw_mode()?;
        terminal.clear()?;

        while self.run {
            terminal.draw(|f| {
                self.draw(f);
            })?;
            self.handle_input()?;
        }
        terminal.clear()?;
        disable_raw_mode()?;

        Ok(())
    }

    fn draw(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
        let size = f.size();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Percentage(40),
                Constraint::Percentage(100),
            ])
            .split(size);
        

        f.render_widget(self.data_view(layout[1]), layout[1]);
        f.render_widget(self.src_view(), layout[2]);
        f.render_widget(self.output_view(), layout[3]);
    }

    fn output_view(&self) -> Paragraph {
        let text = self.interpreter.output.lines().skip(self.output_line).map(|line| Spans::from(line)).collect::<Vec<_>>();
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Output"))
    }

    fn data_view(&self, rect: Rect) -> Paragraph {
        let (tape, adjusted_data_pointer) = self.interpreter.get_range_from_tape(self.memory_start, self.memory_start + (rect.width / 2) as usize);
    
        let colored_style = Style::default().bg(Color::Red);
        let default_style = Style::default();

        let mut text  = vec![];

        for i in 0..tape.len() {
            text.push(Span::styled(tape[i].to_string(), if i == adjusted_data_pointer { colored_style } else {default_style}));
            text.push(Span::from(" "));
        }
        
        Paragraph::new(Spans::from(text)).block(Block::default().borders(Borders::ALL).title("Memory tape"))
    }

    fn src_view(&self) -> Paragraph {
        let (src, ip) = self.interpreter.get_source_with_inst_pointer();

        let colored_style = Style::default().bg(Color::Red);
        let default_style = Style::default();

        let mut text = vec![];
        let mut line = vec![];
        for c in 0..src.len() {

            match src.as_bytes()[c] as char {
                '\n' => {
                    text.push(Spans::from(line));
                    line = vec![];
                }
                ch => {

                    line.push(Span::styled(ch.to_string(), if c == ip { colored_style } else {default_style}));
                }
            }
            if c == src.len() -1 {
                text.push(Spans::from(line));
                line = vec![];
            }
            
        }
        
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Source"))
    }

    fn handle_input(&mut self) -> io::Result<()> {
        if poll(Duration::from_millis(150))? {
            if let Event::Key(event) = read()? {
                use KeyCode::*;

                match event.code {
                    Char(c) => {
                        match c {
                            'q' => {self.run = false;},
                            's' => {self.interpreter.step();},
                            'r' => {self.interpreter.run();}
                            _ => (),
                        }
                    }
                    Left => {
                        if self.memory_start  > 0 {self.memory_start -= 1;}
                    }
                    Right => {
                        if self.memory_start < 30_000 {self.memory_start += 1;}
                    }
                    Down => {
                        if self.output_line + 1 < self.interpreter.output.lines().count() {
                            self.output_line += 1;
                        }
                    }
                    Up => {
                        if self.output_line != 0 {
                            self.output_line -= 1;
                        }
                    }
                    _ => (),
                }
            }
        };

        Ok(())
    }
}
