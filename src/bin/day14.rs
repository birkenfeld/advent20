use advtools::prelude::HashMap;
use advtools::input;

const FORMAT: &str = r"mem\[(\d+)\] = (\d+)|mask = (.+)";

fn process<M, X, FM, FA>(mut process_mask: FM, mut process_assign: FA)
where FM: FnMut(u64, u64, u64) -> M,
      FA: FnMut(u64, u64, &M) -> X
{
    let mut mask_state = None;
    for (mem, mask) in input::rx_lines::<(Option<_>, &str)>(FORMAT) {
        if let Some((addr, val)) = mem {
            process_assign(addr, val, mask_state.as_ref().unwrap());
        } else {
            let (mut zero, mut one) = (0, 0);
            for (i, ch) in mask.chars().enumerate() {
                let bit = 35-i;
                match ch {
                    '0' => zero |= 1 << bit,
                    '1' => one  |= 1 << bit,
                    _ => ()
                }
            }
            mask_state = Some(process_mask(zero, one, !zero & !one));
        }
    }
}

fn all_masks(addr: u64, bits: u64, bit: usize, masks: &mut Vec<u64>) {
    if bit == 36 {
        masks.push(addr);
    } else {
        if bits & (1 << bit) != 0 {
            all_masks(addr | (1 << bit), bits, bit+1, masks);
        }
        all_masks(addr, bits, bit+1, masks);
    }
}

fn main() {
    let mut mem = HashMap::new();
    process(
        |zero, one, _| (!zero, one),
        |addr, val, &(and, or)| mem.insert(addr, (val & and) | or)
    );

    advtools::verify("Sum of memory", mem.values().sum::<u64>(), 4886706177792u64);

    mem.clear();
    process(
        |_, one, x| { let mut ors = vec![]; all_masks(0, x, 0, &mut ors); (!x, one, ors) },
        |addr, val, (and, or, ors)| for or1 in ors.iter() {
            mem.insert((addr & and) | or | or1, val);
        }
    );

    advtools::verify("Sum of memory", mem.values().sum::<u64>(), 3348493585827u64);
}
