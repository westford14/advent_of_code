use std::i32::MAX;

pub struct Submarine {
    pub crabs: Vec<i32>
}

impl Submarine {
    pub fn calculate_fuel(self, fuel_boost: bool) -> i32 {
        let max_val = self.crabs.iter().max().unwrap();
        let mut best_fuel: i32 = MAX; 

        for i in 0..*max_val {
            let mut fuel = 0;
            for crab in self.crabs.iter() {
                if fuel_boost {
                    if i > *crab {
                        fuel += (i - *crab) + (1..(i - *crab)).sum::<i32>();
                    } else if i < *crab {
                        fuel += (*crab - i) + (1..(*crab - i)).sum::<i32>();
                    } else {
                        fuel += 0;
                    }
                } else {
                    if i > *crab {
                        fuel += i - *crab;
                    } else if i < *crab {
                        fuel += *crab - i;
                    } else {
                        fuel += 0;
                    }
                }
            }

            if fuel < best_fuel {
                best_fuel = fuel;
            }
        }
        return best_fuel;
    }
}

pub fn fuel(args: String, part: String) -> i32 {
    let input = args.split(",")
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();

    let sub = Submarine{crabs: input};

    match part.as_ref() {
        "one" => {
            return sub.calculate_fuel(false)
        }, 
        "two" => {
            return sub.calculate_fuel(true)
        }
        _ => panic!("could not understand part {}", part),
    }
}