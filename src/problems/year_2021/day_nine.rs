use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SmokeGrid {
    pub grid: Vec<Vec<i32>>
}

impl SmokeGrid {
    pub fn parse_grid(input: Vec<String>) -> SmokeGrid {
        let mut temp_grid: Vec<Vec<i32>> = Vec::new();
        for strs in input {
            let mut row: Vec<i32> = Vec::new();
            if strs.len() == 0 {
                continue 
            } else {
                for ch in strs.chars(){
                    row.push(ch.to_digit(10).unwrap() as i32);
                }
                temp_grid.push(row);
            }
        }
        SmokeGrid{grid: temp_grid}
    }

    pub fn calculate_low_points(& mut self) -> Vec<(usize, usize)> {
        let mut lowest: Vec<(usize, usize)> = Vec::new();

        for (i, row) in self.grid.iter().enumerate() {
            let ind = i as i8;
            for (j, col) in row.iter().enumerate() {
                let j_ind = j as i8;

                let left = j_ind - 1 >= 0; 
                let right = j_ind + 1 < row.len() as i8;
                let up = ind - 1 >= 0;
                let down = ind + 1 < self.grid.len() as i8;

                let mut direction_map: HashMap<String, (bool, bool)> = HashMap::new();
    
                match left {
                    true => {
                        direction_map.insert("left".to_string(), (true, col < &row[j - 1]));
                    },
                    false => ()
                }; 

                match right {
                    true => {
                        direction_map.insert("right".to_string(), (true, col < &row[j + 1]));
                    },
                    false => ()
                }; 

                match up {
                    true => {
                        direction_map.insert("up".to_string(), (true, col < &self.grid[i - 1][j]));
                    },
                    false => ()
                }; 

                match down {
                    true => {
                        direction_map.insert("down".to_string(), (true, col < &self.grid[i + 1][j]));
                    },
                    false => ()
                }; 
                
                let mut logic_vec: Vec<bool> = Vec::new();

                for (_dir, logic_tup) in &direction_map {
                    if logic_tup.0 {
                        logic_vec.push(logic_tup.1);
                    }
                }
                if logic_vec.iter().all(|x| *x == true) {
                    lowest.push((ind as usize, j_ind as usize));
                }
            }
        }
        lowest
    }

    pub fn calculate_danger(&mut self, low_points: Vec<(usize, usize)>) -> i32 {
        let mut total = 0;
        for pt in low_points {
            total += self.grid[pt.0 as usize][pt.1 as usize] + 1;
        }
        total 
    }

    pub fn calculate_basin(self, low_points: Vec<(usize, usize)>) -> i32 {
        let mut basins: Vec<i32> = Vec::new();

        for pt in low_points {
            let mut exploration: Vec<((usize, usize), (usize, usize))> = vec![(pt, pt)];
            let mut explored: Vec<(usize, usize)> = vec![pt];

            while exploration.len() != 0 {
                let explore = exploration[0];
                let t_pt = explore.1;

                let left = t_pt.1 != 0; 
                let right = t_pt.1 + 1 < self.grid[t_pt.0].len();
                let up = t_pt.0 != 0;
                let down = t_pt.0 + 1 < self.grid.len();

                let mut direction_map: HashMap<String, ((usize, usize), (usize, usize))> = HashMap::new();

                match left {
                    true => {
                        if self.grid[t_pt.0][t_pt.1 - 1] > self.grid[t_pt.0][t_pt.1] && self.grid[t_pt.0][t_pt.1 - 1] != 9 {
                            direction_map.insert(
                                "left".to_string(),
                                ((t_pt.0, t_pt.1), (t_pt.0, t_pt.1 - 1)),
                            );
                        }
                    },
                    false => ()
                }; 

                match right {
                    true => {
                        if self.grid[t_pt.0][t_pt.1 + 1] > self.grid[t_pt.0][t_pt.1] && self.grid[t_pt.0][t_pt.1 + 1] != 9 {
                            direction_map.insert(
                                "right".to_string(),
                                ((t_pt.0, t_pt.1), (t_pt.0, t_pt.1 + 1)),
                            );
                        }
                    },
                    false => ()
                }; 

                match up {
                    true => {
                        if self.grid[t_pt.0 - 1][t_pt.1] > self.grid[t_pt.0][t_pt.1] && self.grid[t_pt.0 - 1][t_pt.1] != 9 {
                            direction_map.insert(
                                "up".to_string(),
                                ((t_pt.0, t_pt.1), (t_pt.0 - 1, t_pt.1)),
                            );
                        }
                    },
                    false => ()
                }; 

                match down {
                    true => {
                        if self.grid[t_pt.0 + 1][t_pt.1] > self.grid[t_pt.0][t_pt.1] && self.grid[t_pt.0 + 1][t_pt.1] != 9 {
                            direction_map.insert(
                                "down".to_string(),
                                ((t_pt.0, t_pt.1), (t_pt.0 + 1, t_pt.1)),
                            );
                        }
                    },
                    false => ()
                }; 

                for (_, temp_pt) in &direction_map {
                    exploration.push(*temp_pt);
                    explored.push(temp_pt.1);
                }
                exploration = exploration[1..].iter().map(|x| *x).collect();
            }
            explored.sort();
            explored.dedup();
            basins.push(explored.len() as i32);
        }

        let mut total = 1;
        basins.sort();
        for val in basins[basins.len() - 3..].iter() {
            total *= *val;
        }
        total
    }
}

pub fn smoke(args: Vec<String>, part: String) -> i32{
    match part.as_ref() {
        "one" => {
            let mut grid = SmokeGrid::parse_grid(args);
            let low_pts = grid.calculate_low_points();
            grid.calculate_danger(low_pts)
        }
        "two" => {
            let mut grid = SmokeGrid::parse_grid(args);
            let low_pts = grid.calculate_low_points();
            grid.calculate_basin(low_pts)
        }
        _ => panic!("could not understand part {}", part),
    }
}