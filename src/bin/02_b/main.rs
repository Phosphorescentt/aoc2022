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

    let winning_fixings = vec![("A", "C"), ("B", "A"), ("C", "B")];
    let losing_fixings = vec![("A", "B"), ("B", "C"), ("C", "A")];

    let mut total = 0;
    for i in 0..p1_moves.len() {
        match p2_moves[i] {
            "X" => {
                total += 0;
                for fixing in winning_fixings.iter() {
                    if fixing.0 == p1_moves[i] {
                        p2_moves[i] = fixing.1;
                    }
                }
            }
            "Y" => {
                total += 3;
                p2_moves[i] = p1_moves[i]
            }
            "Z" => {
                total += 6;
                for fixing in losing_fixings.iter() {
                    if fixing.0 == p1_moves[i] {
                        p2_moves[i] = fixing.1;
                    }
                }
            }
            _ => {}
        }

        total += match p2_moves[i] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => 0,
        };
    }

    println!("{}", total);
}
