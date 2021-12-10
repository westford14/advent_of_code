use std::cmp;

#[derive(Clone, Debug)]
pub struct Coordinate {
    pub x_1: i32,
    pub y_1: i32,
    pub x_2: i32,
    pub y_2: i32,
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub coordinates: Vec<Coordinate>,
}

impl Grid {
    pub fn range(&self) -> (i32, i32) {
        let mut max_x = 0;
        let mut max_y = 0;
        for coord in self.coordinates.iter() {
            if coord.x_2 > max_x {
                max_x = coord.x_2;
            } else if coord.x_1 > max_x {
                max_x = coord.x_1;
            }
            if coord.y_2 > max_y {
                max_y = coord.y_2;
            } else if coord.y_1 > max_y {
                max_y = coord.y_1;
            }
        }
        return (max_x, max_y)
    }

    pub fn overlap(&self, diagonal: bool) -> i32 {
        let mut grid = Vec::new();
        let (max_x, max_y) = self.range();

        for _ in 0..(max_y + 1) {
            grid.push(vec![0; (max_x + 1) as usize]);
        }

        for coord in self.coordinates.iter() {
            if coord.x_1 == coord.x_2 {
                let temp_max = cmp::max(coord.y_2, coord.y_1);
                let temp_min = cmp::min(coord.y_2, coord.y_1); 
                for i in temp_min..(temp_max + 1) {
                    grid[i as usize][coord.x_1 as usize] += 1
                }
                
            } else if coord.y_1 == coord.y_2 {
                let temp_max = cmp::max(coord.x_2, coord.x_1);
                let temp_min = cmp::min(coord.x_2, coord.x_1); 
                for i in temp_min..(temp_max + 1) {
                    grid[coord.y_1 as usize][i as usize] += 1
                }
            } else {
                if diagonal {
                    if coord.x_1 < coord.x_2 && coord.y_1 > coord.y_2 {
                        let temp: Vec<(i32, i32)> = (coord.x_1..(coord.x_2 + 1)).zip((coord.y_2..(coord.y_1 + 1)).rev()).collect();
                        for (i, j) in temp.iter() {
                            grid[*j as usize][*i as usize] += 1;
                        }
                    } else if coord.x_1 < coord.x_2 && coord.y_1 < coord.y_2  {
                        let temp: Vec<(i32, i32)> = (coord.x_1..(coord.x_2 + 1)).zip(coord.y_1..(coord.y_2 + 1)).collect();
                        for (i, j) in temp.iter() {
                            grid[*j as usize][*i as usize] += 1;
                        }
                    } else if coord.x_1 > coord.x_2 && coord.y_1 < coord.y_2 {
                        let temp: Vec<(i32, i32)> = (coord.x_2..(coord.x_1 + 1)).rev().zip(coord.y_1..(coord.y_2 + 1)).collect();
                        for (i, j) in temp.iter() {
                            grid[*j as usize][*i as usize] += 1;
                        }
                    } else if coord.x_1 > coord.x_2 && coord.y_1 > coord.y_2 {
                        let temp: Vec<(i32, i32)> = (coord.x_2..(coord.x_1 + 1)).rev().zip((coord.y_2..(coord.y_1 + 1)).rev()).collect();
                        for (i, j) in temp.iter() {
                            grid[*j as usize][*i as usize] += 1;
                        }
                    }
                }
            }
        }

        let mut total = 0;
        for row in grid.iter() {
            for item in row.iter() {
                if item >= &2 {
                    total += 1;
                }
            }
        }
        
        return total
    }
}

pub fn vents(args: Vec<String>, part: String) -> i32 {
    let mut coordinates: Vec<Coordinate> = Vec::new();
    for arg in args.iter() {
        let arg_clean = arg.replace(" ", "").replace("->", ",");
        let arr = arg_clean.split(",").map(|x| x.to_string()).collect::<Vec<String>>();
        let coord = Coordinate{
            x_1: arr[0].parse::<i32>().unwrap(),
            y_1: arr[1].parse::<i32>().unwrap(),
            x_2: arr[2].parse::<i32>().unwrap(),
            y_2: arr[3].parse::<i32>().unwrap(),
        };
        coordinates.push(coord);
    }

    let grid = Grid{coordinates};

    match part.as_ref() {
        "one" => {
            return grid.overlap(false)
        },
        "two" => {
            return grid.overlap(true)
        },
        _ => panic!("could not understand part: {}", part)
    }
}