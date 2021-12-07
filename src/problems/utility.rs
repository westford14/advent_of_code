pub fn clean_array(args: Vec<String>) -> Vec<String> {
    let mut input = args.clone();
    if input[input.len() - 1] == "" {
        input.remove(input.len() - 1);
        return input;
    } else {
        return input;
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub grid: Vec<Vec<String>>,
    pub reflected_grid: Vec<Vec<usize>>,
}

impl Board {
    pub fn default() -> Board {
        Board { 
            grid: Vec::new(),
            reflected_grid: Vec::new()
        }
    }

    pub fn check_match(&mut self, val: String) -> i32 {
        for (i, row) in self.grid.iter().enumerate() {
            let matched = row.iter().position(|x| *x == val);
            if matched != None {
                self.reflected_grid[i][matched.unwrap()] = 1;
                return self.is_bingo(val) 
            }
        }
        return -1
    }

    pub fn is_bingo(&mut self, called_val: String) -> i32 {
        for (_i, row) in self.reflected_grid.iter().enumerate() {
            if row.iter().sum::<usize>() == row.len() {
                return self.calculate_score(called_val) 
            }
        }
        for i in 0..self.reflected_grid.len() {
            let col_sum: usize = self.reflected_grid.iter().map(|x| x[i]).sum();
            if col_sum == self.reflected_grid.len() {
                return self.calculate_score(called_val) 
            }
        }
        return -1 
    }

    pub fn calculate_score(&mut self, val: String) -> i32 {
        let called_val = val.parse::<i32>().unwrap();
        let mut uncalled_vec: Vec<i32> = Vec::new();
        for (i, _row) in self.reflected_grid.iter().enumerate() {
            for (j, &val) in self.reflected_grid[i].iter().enumerate() {
                if val == 0 as usize {
                    uncalled_vec.push(self.grid[i][j].parse::<i32>().unwrap());
                }
            }
        }
        return called_val * uncalled_vec.iter().sum::<i32>()
    }
}

#[derive(Clone, Debug)]
pub struct Bingo {
    pub draws: Vec<String>,
    pub boards: Vec<Board>,
}

impl Bingo {
    pub fn default() -> Bingo {
        Bingo{
            draws: Vec::new(),
            boards: Vec::new(),
        }
    }
    pub fn parse_input(&mut self, input: Vec<String>) {
        self.draws = input[0].split(",").map(|s| s.to_string()).collect();

        let mut lines = &input[2..];

        while lines.len() != 0 {
            let line = &lines[0];
            if line == "" {
                lines = &lines[1..];
                continue
            } else {
                let mut board = Board::default();
                let mut second_line = &lines[0];

                while second_line != "" {
                    board.grid.push(lines[0].trim_start_matches(' ').replace("  ", " ").split(" ").map(|s| s.to_string()).collect());
                    let mut reflect_grid: Vec<Vec<usize>> = Vec::new();
                    for _ in 0..board.grid.len() {
                        reflect_grid.push(vec![0; board.grid[0].len()])
                    }
                    board.reflected_grid = reflect_grid;
                    
                    lines = &lines[1..];

                    if lines.len() == 0 {
                        break
                    } else {
                        second_line = &lines[0];
                    }
                }
                self.boards.push(board);
            }
        }
    }
}
