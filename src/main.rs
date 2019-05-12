//extern crate clap;
extern crate termion;
extern crate rand;

use std::io::{self, Read, Write};
//use clap::{App, Arg};

struct TextShiftMatrix {
    width: u16,
    height: u16,
    cells: Vec<char>,
    targetcells: Vec<char>
}

impl TextShiftMatrix {
    fn newfullscreen() -> TextShiftMatrix {
        let (width, height) = termion::terminal_size().unwrap();
        TextShiftMatrix::new(width,height)
    }

    fn new(width: u16, height: u16) -> TextShiftMatrix {
        let mut m = TextShiftMatrix {
            width: width,
            height: height,
            cells: Vec::new(),
            targetcells: Vec::new()
        };
        let size = width*height;
        for _ in 0..size {
            let choice: f64 = rand::random::<f64>() * (126.0 - 32.0) + 32.0;
            let choice: u8 = choice.floor() as u8;
            m.cells.push(choice as char);
            m.targetcells.push(' ');
        }
        m
    }

    fn printcenter(&mut self, text: String) {
        let mut width = 1;
        let mut offset = 0;
        let mut height = 1;
        for c in text.chars() {
            if c == '\n' {
                height += 1;
                offset = 0;
                continue;
            }
            offset += 1;
            if offset > width {
                width = offset;
            }
        }
        if width > self.width {
            width = self.width;
        }
        if height > self.height {
            height = self.height;
        }
        let x: u16 = self.width / 2 - width / 2;
        let y: u16 = self.height / 2 - height / 2;
        return self.print(x,y, text);
    }

    fn print(&mut self, x: u16, y: u16, text: String) {
        let mut offset: u16 = 0;
        let mut y = y;
        for c in text.chars() {
            if c == '\n' {
                y += 1;
                offset = 0;
                continue;
            }
            if let Some(index) = self.getindex(x+ offset,y) {
                self.targetcells[index] = c;
            }
            offset += 1;
        }
    }

    fn tick(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let index = self.getindex(x,y).unwrap();
                if self.cells[index] != self.targetcells[index] {
                    let choice: f64 = rand::random::<f64>() * (126.0 - 32.0) + 32.0;
                    let choice: u8 = choice.floor() as u8;
                    self.cells[index] = choice as char;
                }
            }
        }
    }

    fn getindex(&self, x: u16, y: u16) -> Option<usize> {
        return if x < self.width && y < self.height {
            Some((x*self.height + y) as usize)
        } else {
            None
        }
    }

    fn render(&self, stdout: &mut std::io::StdoutLock  ) {
        for x in 0..self.width {
            for y in 0..self.height {
                let t = self.gettarget(x,y).unwrap();
                let c = self.get(x,y).unwrap();
                if c == t {
                    write!(stdout, "{}{}{}{}{}", termion::cursor::Goto(x+1, y+1), termion::style::Bold, termion::color::Fg(termion::color::White), c, termion::style::Reset).unwrap();
                } else {
                    write!(stdout, "{}{}{}", termion::cursor::Goto(x+1, y+1), termion::color::Fg(termion::color::White), c).unwrap();
                }
            }
        }
        stdout.flush().unwrap();
    }

    fn get(&self, x: u16, y: u16) -> Option<char> {
       self.getindex(x,y).map(|i| self.cells[i])
    }

    fn gettarget(&self, x: u16, y: u16) -> Option<char> {
       self.getindex(x,y).map(|i| self.targetcells[i])
    }

    fn run(&mut self, stdout: &mut std::io::StdoutLock) {
        while !self.done() {
            self.tick();
            self.render(stdout);
        }
    }

    fn done(&self) -> bool {
       self.cells.iter().zip(&self.targetcells).all( |(c,t)| *c == *t )
    }
}

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut matrix = TextShiftMatrix::newfullscreen();
    let mut inputbuffer = String::new();
    io::stdin().read_to_string(&mut inputbuffer).unwrap();
    matrix.printcenter(inputbuffer);
    matrix.run(&mut stdout);
}

