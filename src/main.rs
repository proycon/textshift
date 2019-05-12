extern crate clap;
extern crate termion;
extern crate rand;

use std::io::{self, Read, Write};
use std::collections::BTreeSet;

struct TextShiftMatrix {
    width: u16,
    height: u16,
    cells: Vec<char>,
    targetcells: Vec<char>,
    vocabulary: BTreeSet<char>,
    probability: f64,
    defaulttarget: char
}


impl TextShiftMatrix {
    fn newfullscreen(probability: f64, vocabulary: String, defaulttarget: char) -> TextShiftMatrix {
        let (width, height) = termion::terminal_size().unwrap();
        TextShiftMatrix::new(width,height,probability, vocabulary, defaulttarget)
    }

    fn new(width: u16, height: u16, probability: f64, vocabulary: String, defaulttarget: char) -> TextShiftMatrix {
        let mut m = TextShiftMatrix {
            width: width,
            height: height,
            cells: Vec::new(),
            targetcells: Vec::new(),
            vocabulary: BTreeSet::new(),
            probability: probability,
            defaulttarget: defaulttarget
        };
        m.vocabulary.insert(defaulttarget);
        for c in vocabulary.chars() {
            m.vocabulary.insert(c);
        }
        let vocabulary: Vec<char> = m.vocabulary.iter().cloned().collect();
        let size = width*height;
        for _ in 0..size {
            let choice: f64 = rand::random::<f64>() * vocabulary.len() as f64;
            let choice: usize = choice.floor() as usize;
            m.cells.push(vocabulary[choice]);
            m.targetcells.push(defaulttarget);
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
        let vocabulary: Vec<char> = self.vocabulary.iter().cloned().collect();
        for x in 0..self.width {
            for y in 0..self.height {
                let index = self.getindex(x,y).unwrap();
                if self.cells[index] != self.targetcells[index] {
                    if rand::random::<f64>() <= self.probability {
                        self.cells[index] = self.targetcells[index];
                    } else {
                        let choice: f64 = rand::random::<f64>() * vocabulary.len() as f64;
                        let choice: usize = choice.floor() as usize;
                        self.cells[index] = vocabulary[choice];
                    }
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
    let argmatches = clap::App::new("Textshift")
        .version("0.1")
        .author("Maarten van Gompel (proycon) <proycon@anaproy.nl>")
        .about("Text emerges from noise")
        .arg(clap::Arg::with_name("vocabulary")
            .help("Vocabulary as a string")
            .short("v")
            .long("vocabulary")
            .takes_value(true)
            .default_value("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*(){}[]?.,<>;:' ")
        )
        .arg(clap::Arg::with_name("defaulttarget")
            .help("Vocabulary as a string")
            .short("t")
            .long("defaulttarget")
            .takes_value(true)
            .default_value(" ")
        )
        .arg(clap::Arg::with_name("probability")
             .help("Probability")
             .long("probability")
             .short("p")
             .takes_value(true)
             .default_value("0.01")
        )
        .get_matches();
    let defaulttarget: String = argmatches.value_of("defaulttarget").unwrap().to_string();
    let defaulttarget: char = defaulttarget.chars().last().unwrap();
    let probability: f64 = argmatches.value_of("probability").unwrap().parse().unwrap();

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut matrix = TextShiftMatrix::newfullscreen(probability, argmatches.value_of("vocabulary").unwrap().to_string(), defaulttarget );
    let mut inputbuffer = String::new();
    io::stdin().read_to_string(&mut inputbuffer).unwrap();
    matrix.printcenter(inputbuffer);
    matrix.run(&mut stdout);
}

