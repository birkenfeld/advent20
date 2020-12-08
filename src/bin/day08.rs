use advtools::input::iter_input;
use advent20::{Machine, Op};

fn main() {
    let mut machine = Machine::default();
    let mut prog = Machine::parse(iter_input());

    let bad_accu = machine.run(&prog).unwrap_err();
    advtools::verify("Accumulator on loop", bad_accu, 1816);

    let mut fixed_accu = None;
    for i in 0..prog.len() {
        machine.reset();
        match prog[i] {
            Op::Acc(_) => (),
            Op::Nop(x) => {
                prog[i] = Op::Jmp(x);
                if let Ok(i) = machine.run(&prog) {
                    fixed_accu = Some(i);
                    break;
                }
                prog[i] = Op::Nop(x);
            }
            Op::Jmp(x) => {
                prog[i] = Op::Nop(x);
                if let Ok(i) = machine.run(&prog) {
                    fixed_accu = Some(i);
                    break;
                }
                prog[i] = Op::Jmp(x);
            }
        }
    }

    advtools::verify("Accumulator when fixed", fixed_accu.unwrap(), 1149);
}
