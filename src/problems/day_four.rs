use crate::problems::utility;

pub fn bingo(args: Vec<String>, part: String) -> i32 {
    let mut input = utility::Bingo::default();
    input.parse_input(args);

    match part.as_ref() {
        "one" => {
            for draw in input.draws.iter() {
                for board in &mut input.boards {
                    let check = board.check_match(draw.to_string());
                    if check != -1 {
                        return check 
                    } 
                }
            }
        },
        "two" => {
            let mut winners: Vec<i32> = Vec::new();
            for draw in input.draws.iter() {
                let mut winner: bool = false;
                let mut i = 0;
                let mut remove: Vec<usize> = Vec::new();
                for board in &mut input.boards {
                    let check = board.check_match(draw.to_string());
                    if check != -1 {
                        winners.push(check);
                        winner = true;
                        remove.push(i);
                    } 
                    i += 1;
                }
                if winner {
                    for (i, val) in remove.iter().enumerate() {
                        input.boards.remove(*val - i);
                    }
                } 
            }
            return winners[winners.len() - 1]
        }
        _ => panic!("part {} not understood", part)
    }
    0
}