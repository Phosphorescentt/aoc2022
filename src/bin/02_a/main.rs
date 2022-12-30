use std::fs;

fn main() {
    let content = fs::read_to_string("input_valid").unwrap();
    let games_vec = content.split("\n").collect::<Vec<&str>>();

    let mut p1_moves: Vec<&str> = Vec::new();
    let mut p2_moves: Vec<&str> = Vec::new();

    for game in games_vec.iter() {
        let moves = game.split(" ").collect::<Vec<&str>>();
        p1_moves.push(moves[0]);
        p2_moves.push(moves[1]);
    }

    let mut total = 0;
    for i in 0..p1_moves.len() {
        total += match p2_moves[i] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };

        total += match (p2_moves[i], p1_moves[i]) {
            ("X", "C") => 6,
            ("Y", "A") => 6,
            ("Z", "B") => 6,
            ("X", "A") => 3,
            ("Y", "B") => 3,
            ("Z", "C") => 3,
            _ => 0,
        }
    }

    println!("{}", total);
}
