use advtools::input::iter_input_regex;

fn main() {
    let pws: Vec<(_, _, char, String)> = iter_input_regex(r"(\d+)-(\d+) (.): (.*)").collect();

    let count1 = pws.iter().filter(|(min, max, ch, pw)| {
        let n = pw.chars().filter(|c| c == ch).count();
        n >= *min && n <= *max
    }).count();

    let count2 = pws.iter().filter(|(pos1, pos2, ch, pw)| {
        let correct_fst = pw.chars().nth(pos1 - 1) == Some(*ch);
        let correct_snd = pw.chars().nth(pos2 - 1) == Some(*ch);
        correct_fst ^ correct_snd
    }).count();

    advtools::verify("Correct PWs (way 1)", count1, 600);
    advtools::verify("Correct PWs (way 2)", count2, 245);
}
