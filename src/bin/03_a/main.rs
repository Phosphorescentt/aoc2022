use std::collections::HashSet;
use std::fs;

fn main() {
    let content = fs::read_to_string("input_valid").unwrap();
    let rucksacks = content.split("\n").collect::<Vec<&str>>();

    let mut total_priority: i32 = 0;
    for items in rucksacks.iter() {
        let pouch_size = items.len() / 2;
        let pouch1: HashSet<char> = items
            .chars()
            .enumerate()
            .filter(|&(i, _)| i < pouch_size)
            .map(|(_, e)| e)
            .into_iter()
            .collect();
        let pouch2: HashSet<char> = items
            .chars()
            .enumerate()
            .filter(|&(i, _)| i >= pouch_size)
            .map(|(_, e)| e)
            .into_iter()
            .collect();

        let intersection: Vec<&char> = pouch1.intersection(&pouch2).collect();
        let unicode_val = *intersection[0] as i32 - 96;
        let ord = match unicode_val {
            0..=100 => unicode_val,
            -100..=0 => unicode_val + 52 + 6,
            _ => 0,
        };
        println!("{} : {}", intersection[0], ord);
        total_priority += ord;
    }
    println!("{}", total_priority);
}
