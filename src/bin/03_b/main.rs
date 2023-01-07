use std::collections::HashSet;
use std::fs;

fn main() {
    let content = fs::read_to_string("input_valid").unwrap();
    let rucksacks = content.split("\n").collect::<Vec<&str>>();
    let mut groups: Vec<Vec<&str>> = Vec::new();

    let mut count = 0;
    let mut current_group: Vec<&str> = Vec::new();
    for rucksack in rucksacks.iter() {
        if count != 2 {
            current_group.push(rucksack);
            count += 1;
        } else {
            count = 0;
            current_group.push(rucksack);
            groups.push(current_group.clone());
            current_group = Vec::new();
        }
    }

    let mut chars: Vec<char> = Vec::new();
    for group in groups.iter() {
        // let bag1: HashSet<char> = HashSet::from_iter(group[0].chars());
        // let bag2: HashSet<char> = HashSet::from_iter(group[1].chars());
        // let bag3: HashSet<char> = HashSet::from_iter(group[2].chars());
        // println!("{:?}", bag1);
        // println!("{:?}", bag2);
        // println!("{:?}", bag3);

        let mut g: Vec<&str> = group.clone();
        // println!("{:?}", g);
        let (intersection, others) = g.split_at_mut(1);
        // println!("{:?} : {:?}", intersection, others);
        let mut intersection = intersection[0].chars().collect::<Vec<char>>();
        for other in others.iter() {
            intersection.retain(|e| other.chars().collect::<Vec<char>>().contains(e));
        }

        chars.push(intersection[0]);
    }

    let mut total_priority: i32 = 0;
    for char in chars.iter() {
        let unicode_val = *char as i32 - 96;
        let ord = match unicode_val {
            0..=100 => unicode_val,
            -100..=0 => unicode_val + 52 + 6,
            _ => 0,
        };
        println!("{} : {}", char, ord);
        total_priority += ord;
    }
    println!("{}", total_priority);
    // for items in rucksacks.iter() {
    //     let pouch_size = items.len() / 2;
    //     let pouch1: HashSet<char> = items
    //         .chars()
    //         .enumerate()
    //         .filter(|&(i, _)| i < pouch_size)
    //         .map(|(_, e)| e)
    //         .into_iter()
    //         .collect();
    //     let pouch2: HashSet<char> = items
    //         .chars()
    //         .enumerate()
    //         .filter(|&(i, _)| i >= pouch_size)
    //         .map(|(_, e)| e)
    //         .into_iter()
    //         .collect();

    //     let intersection: Vec<&char> = pouch1.intersection(&pouch2).collect();
    //     let unicode_val = *intersection[0] as i32 - 96;
    //     let ord = match unicode_val {
    //         0..=100 => unicode_val,
    //         -100..=0 => unicode_val + 52 + 6,
    //         _ => 0,
    //     };
    //     println!("{} : {}", intersection[0], ord);
    //     total_priority += ord;
    // }
    // println!("{}", total_priority);
}
