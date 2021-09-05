use advtools::prelude::*;
use advtools::input::iter_input;

#[derive(Clone, Copy, Debug)]
pub enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Default)]
pub struct Machine {
    pc: usize,
    accu: i32,
    visited: HashSet<usize>,
}

fn run(machine: &mut Machine, prog: &[Op]) -> Result<i32, i32> {
    while machine.pc < prog.len() {
        if !machine.visited.insert(machine.pc) {
            return Err(machine.accu);
        }
        let op = prog[machine.pc];
        machine.pc += 1;
        match op {
            Op::Nop(_) => (),
            Op::Acc(arg) => machine.accu += arg,
            Op::Jmp(arg) => machine.pc = ((machine.pc as i32) + (arg - 1)) as usize,
        }
    }
    Ok(machine.accu)
}

fn main() {
    let mut machine = Machine::default();
    let mut prog = iter_input().map(|line: Vec<String>| match &*line[0] {
        "nop" => Op::Nop(line[1].parse().expect("invalid op arg")),
        "acc" => Op::Acc(line[1].parse().expect("invalid op arg")),
        "jmp" => Op::Jmp(line[1].parse().expect("invalid op arg")),
        _ => panic!("unknown op: {}", &line[0])
    }).collect_vec();

    let bad_accu = run(&mut machine, &prog).unwrap_err();
    advtools::verify("Accumulator on loop", bad_accu, 1816);

    let mut fixed_accu = None;
    for i in 0..prog.len() {
        machine.pc = 0;
        machine.accu = 0;
        machine.visited.clear();
        match prog[i] {
            Op::Acc(_) => (),
            Op::Nop(x) => {
                prog[i] = Op::Jmp(x);
                if let Ok(i) = run(&mut machine, &prog) {
                    fixed_accu = Some(i);
                    break;
                }
                prog[i] = Op::Nop(x);
            }
            Op::Jmp(x) => {
                prog[i] = Op::Nop(x);
                if let Ok(i) = run(&mut machine, &prog) {
                    fixed_accu = Some(i);
                    break;
                }
                prog[i] = Op::Jmp(x);
            }
        }
    }
    advtools::verify("Accumulator when fixed", fixed_accu.unwrap(), 1149);
}
