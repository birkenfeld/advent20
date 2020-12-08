use advtools::prelude::*;

pub type Arg = i32;

#[derive(Clone, Copy, Debug)]
pub enum Op {
    Nop(Arg),
    Acc(Arg),
    Jmp(Arg),
}

fn imm(s: &str) -> Arg {
    s.parse().expect("invalid op arg")
}

#[derive(Default)]
pub struct Machine {
    accu: i32,
    pc:   usize,
    visited: HashSet<usize>,
}

impl Machine {
    pub fn parse(prog: impl Iterator<Item=Vec<String>>) -> Vec<Op> {
        prog.map(|line| match &*line[0] {
            "nop" => Op::Nop(imm(&line[1])),
            "acc" => Op::Acc(imm(&line[1])),
            "jmp" => Op::Jmp(imm(&line[1])),
            _ => panic!("unknown op: {}", &line[0])
        }).collect()
    }

    pub fn run(&mut self, prog: &[Op]) -> Result<i32, i32> {
        while self.pc < prog.len() {
            if !self.visited.insert(self.pc) {
                return Err(self.accu);
            }
            let op = prog[self.pc];
            self.pc += 1;
            match op {
                Op::Nop(_) => (),
                Op::Acc(arg) => self.accu += arg,
                Op::Jmp(arg) => self.pc = ((self.pc as i32) + (arg - 1)) as usize,
            }
        }
        Ok(self.accu)
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.accu = 0;
        self.visited.clear();
    }
}
