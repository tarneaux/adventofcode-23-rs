fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let lines = input.lines();
    let games: Vec<_> = lines
        .map(|line| {
            let id: u32 = line
                .split(":")
                .next()
                .unwrap()
                .split_whitespace()
                .next_back()
                .unwrap()
                .parse()
                .unwrap();
            let rounds: Vec<_> = line
                .rsplit(":")
                .next()
                .unwrap()
                .split(";")
                .map(|round| Round::from(round))
                .collect();
            Game(id, rounds)
        })
        .collect();
    let id_sum: u32 = games
        .into_iter()
        .filter(|game| {
            for round in &game.1 {
                if round.red > 12 || round.green > 13 || round.blue > 14 {
                    return false;
                }
            }
            true
        })
        .map(|game| game.0)
        .sum();
    println!("{}", id_sum);
}

struct Game(u32, Vec<Round>);

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Round {
    fn from(item: &str) -> Self {
        let color_strings: Vec<_> = item.split(",").map(|v| v.trim()).collect();
        Round {
            red: get_color_count(&color_strings, "red"),
            green: get_color_count(&color_strings, "green"),
            blue: get_color_count(&color_strings, "blue"),
        }
    }
}

fn get_color_count(color_strings: &[&str], color: &str) -> u32 {
    color_strings
        .iter()
        .find_map(|v| {
            if v.ends_with(color) {
                Some(v.split_whitespace().next().unwrap().parse().unwrap())
            } else {
                None
            }
        })
        .unwrap_or(0)
}
