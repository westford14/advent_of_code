use crate::problems::utility;

pub fn fuel(args: Vec<String>, part: String) -> i32 {
    let input = utility::clean_array(args);

    match part.as_ref() {
        "one" => {
            let mut gamma: Vec<char> = Vec::new();
            let mut epsilon: Vec<char> = Vec::new();

            for i in 0..input[0].len() {
                let clone_input = input.clone();
                let mut bin_0_score = 0;
                let mut bin_1_score = 0;
                let char_array = &clone_input.into_iter().map(|x| x.chars().nth(i).unwrap()).collect::<Vec<char>>();

                for char in char_array {
                    if *char == '0' {
                        bin_0_score += 1;
                    } else {
                        bin_1_score += 1;
                    }
                }

                if bin_0_score > bin_1_score {
                    gamma.push('0');
                    epsilon.push('1');
                } else {
                    gamma.push('1');
                    epsilon.push('0');
                }

            }
            
            return (isize::from_str_radix(&gamma.iter().collect::<String>(), 2).unwrap() * isize::from_str_radix(&epsilon.iter().collect::<String>(), 2).unwrap()).try_into().unwrap()
        },
        "two" => {
            let mut oxygen_inputs = input.clone();
            let mut dioxide_inputs =  input.clone();

            for i in 0..input[0].len() {
                let mut bin_0_score_oxygen = 0;
                let mut bin_1_score_oxygen = 0;
                let mut bin_0_score_dioxide = 0;
                let mut bin_1_score_dioxide = 0;
                let char_array_oxygen = &oxygen_inputs.clone().into_iter().map(|x| x.chars().nth(i).unwrap()).collect::<Vec<char>>();
                let char_array_dioxide = &dioxide_inputs.clone().into_iter().map(|x| x.chars().nth(i).unwrap()).collect::<Vec<char>>();

                for char in char_array_oxygen {
                    if *char == '0' {
                        bin_0_score_oxygen += 1;
                    } else {
                        bin_1_score_oxygen += 1;
                    }
                }

                for char in char_array_dioxide {
                    if *char == '0' {
                        bin_0_score_dioxide += 1;
                    } else {
                        bin_1_score_dioxide += 1;
                    }
                }

                if oxygen_inputs.len() != 1 {
                    if bin_0_score_oxygen > bin_1_score_oxygen {
                        oxygen_inputs = oxygen_inputs.into_iter()
                            .filter(|x| x.chars().nth(i) == Some('0')).collect();
                    } else {
                        oxygen_inputs = oxygen_inputs.into_iter()
                            .filter(|x| x.chars().nth(i) == Some('1')).collect();
                    }
                }

                if dioxide_inputs.len() != 1 {
                    if bin_0_score_dioxide > bin_1_score_dioxide {
                        dioxide_inputs = dioxide_inputs.into_iter()
                            .filter(|x| x.chars().nth(i) == Some('1')).collect();
                    } else {
                        dioxide_inputs = dioxide_inputs.into_iter()
                            .filter(|x| x.chars().nth(i) == Some('0')).collect();
                    }
                }
            }

            let oxygen_generator: i32 = isize::from_str_radix(&oxygen_inputs.into_iter().collect::<String>(), 2).unwrap().try_into().unwrap();
            let dioxide_generator: i32 = isize::from_str_radix(&dioxide_inputs.into_iter().collect::<String>(), 2).unwrap().try_into().unwrap();

            return oxygen_generator * dioxide_generator
        },
        _ => panic!("could not understsand part {}", part)
    }
}