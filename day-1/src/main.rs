const NUMBER_STRS: [&str; 19] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3",
    "4", "5", "6", "7", "8", "9",
];

fn main() {
    let contents = std::fs::read_to_string("input").unwrap();
    let sum: usize = contents
        .lines()
        .map(|el| {
            let digits = matches_in_string(el, &NUMBER_STRS);
            let digits: Vec<_> = digits.iter().map(|v| v.as_str()).collect();
            let first_and_last = [digits[0], digits[digits.len() - 1]];
            let first_and_last = first_and_last.map(|el| string_to_int(el.as_ref()));
            first_and_last[0].unwrap() * 10 + first_and_last[1].unwrap()
        })
        .sum();
    println!("{}", sum);
}

fn string_to_int(input: &str) -> Option<usize> {
    NUMBER_STRS
        .iter()
        .position(|el| *el == input)
        .map(|v| if v > 8 { v - 9 } else { v + 1 }) // To account for the two different kinds of
                                                    // numbers in the same array
}

// Return all strings that match one of the strings in matches in input
fn matches_in_string(input: &str, matches: &[&str]) -> Vec<String> {
    (0..input.len())
        .map(|el| input[el..].to_string())
        .filter_map(|el| {
            for m in matches {
                if el.starts_with(m) {
                    return Some(m.to_string());
                }
            }
            None
        })
        .collect()
}
