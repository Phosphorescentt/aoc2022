use std::fs;

fn main() {
    let calories_string = fs::read_to_string("input_valid").expect("Unable to read the file");
    let calories_vec = calories_string.split("\n").collect::<Vec<&str>>();
    let mut elves_vec: Vec<i32> = Vec::new();
    let mut current_total: i32 = 0;
    for e in calories_vec.iter() {
        if *e == "" {
            elves_vec.push(current_total.clone());
            current_total = 0;
        } else {
            match e.parse::<i32>() {
                Ok(val) => {
                    current_total = current_total + val;
                }
                Err(_) => {}
            }
        }
    }

    // let max_value = match elves_vec.iter().max() {
    //     Some(val) => val,
    //     _ => &0,
    // };

    // println!("{}", max_value);

    let mut i = 0;
    let mut total = 0;
    while i < 3 {
        let max_value = elves_vec.iter().max().unwrap();
        let index = elves_vec.iter().position(|x| *x == *max_value).unwrap();
        total += elves_vec.remove(index);
        i += 1;
    }

    println!("{}", total);
}
