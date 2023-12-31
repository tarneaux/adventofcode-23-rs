#![feature(let_chains)]

use std::char;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let get_matching_coordinates = |is_accepted: fn(&char) -> bool| {
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                (
                    y,
                    line.chars()
                        .enumerate()
                        .filter(|(_, c)| is_accepted(c))
                        .map(|(i, c)| (i, c.to_string()))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>()
    };
    let digit_chars = get_matching_coordinates(|c| c.to_digit(10).is_some());
    let numbers: Vec<_> = digit_chars
        .iter()
        .map(|(y, chars)| (y, combine_consecutive(&chars)))
        .map(|(y, numbers)| {
            numbers
                .into_iter()
                .map(|(x, s)| (y, x, s.len(), s.parse::<u32>().unwrap()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    let symbol_chars = get_matching_coordinates(|c| c.to_digit(10).is_none() && *c != '.');
    let mut symbol_coordinates: Vec<_> = symbol_chars
        .iter()
        .map(|(y, chars)| (y, chars.iter().map(|(x, _)| x).collect::<Vec<_>>()))
        .map(|(y, x_s)| x_s.into_iter().map(|x| (y, x)).collect::<Vec<_>>())
        .flatten()
        .collect();
    symbol_coordinates.sort();
    let filtered_numbers: Vec<_> = numbers
        .iter()
        .filter(|(y, x, l, _)| {
            let allowed_positions: Vec<_> = (0..3)
                .map(|w| {
                    (0..l + 2)
                        .map(|v| ((*y + w).checked_sub(1), (x + v).checked_sub(1)))
                        .filter(|(y, x)| y.is_some() && x.is_some())
                        .map(|(y, x)| (y.unwrap(), x.unwrap()))
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect();
            for p in allowed_positions {
                let p: &(&usize, &usize) = &(&p.0, &p.1);
                if symbol_coordinates.binary_search(&p).is_ok() {
                    return true;
                }
            }
            false
        })
        .map(|(_, _, _, v)| *v)
        .collect();
    println!("sum: {}", filtered_numbers.iter().sum::<u32>());

    let gear_chars = get_matching_coordinates(|c| *c == '*');
    let gear_positions: Vec<_> = gear_chars
        .iter()
        .map(|(y, numbers)| numbers.into_iter().map(|(x, _)| (y, x)).collect::<Vec<_>>())
        .flatten()
        .collect();
    let filtered_gears: Vec<_> = gear_positions
        .iter()
        .filter_map(|(y, x)| {
            let allowed_positions: Vec<_> = (0..3)
                .map(|w| {
                    (0..3)
                        .map(|v| ((*y + w).checked_sub(1), (*x + v).checked_sub(1)))
                        .filter(|(y, x)| y.is_some() && x.is_some())
                        .map(|(y, x)| (y.unwrap(), x.unwrap()))
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect();
            let mut my_numbers = numbers.clone();
            let numbers: Vec<_> = allowed_positions
                .iter()
                .filter_map(|p| {
                    let my_numbers_cloned = my_numbers.clone();
                    let mut rv: Option<u32> = None;
                    for (i, (ny, nx, nl, nv)) in my_numbers_cloned.iter().enumerate() {
                        if **ny == p.0 && *nx <= p.1 && nx + nl > p.1 {
                            my_numbers.remove(i);
                            rv = Some(*nv);
                            break;
                        }
                    }
                    rv
                })
                .collect();
            if numbers.len() == 2 {
                return Some(numbers);
            } else {
                return None;
            }
        })
        .collect();
    let gear_ratios: Vec<_> = filtered_gears
        .iter()
        .map(|ab| ab.first().unwrap() * ab.last().unwrap())
        .collect();
    println!("gear ratio sum: {:?}", gear_ratios.iter().sum::<u32>())
}

fn combine_consecutive(items: &Vec<(usize, String)>) -> Vec<(usize, String)> {
    let mut out: Vec<(usize, String)> = Vec::new();
    for (i, s) in items {
        if let Some(v) = out.last() && v.0 + v.1.len() == *i {
            out.last_mut().unwrap().1 += s;
        } else {
            out.push((*i, s.to_string()));
        }
    }
    out
}
