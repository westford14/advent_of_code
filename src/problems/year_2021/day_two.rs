use crate::problems::year_2021::utility;

pub fn dive(args: Vec<String>, part: String) -> i32 {
    let input = utility::clean_array(args);

    match part.as_ref() {
        "one" => {
            let mut x_position = 0;
            let mut y_position = 0;
    
            for directions in input.iter() {
                let func_input: Vec<String> = directions.split(" ").map(|s| s.to_string()).collect();
                let direction = func_input[0].to_string();
                let distance = func_input[1].parse::<i32>().unwrap();
        
                match direction.as_ref() {
                    "forward" => x_position += distance,
                    "up" => y_position -= distance,
                    "down" => y_position += distance,
                    _ => panic!("could not understand direction: {}", direction)
                } 
            }
        
            return x_position * y_position
        }, 
        "two" => {
            let mut x_position = 0;
            let mut y_position = 0;
            let mut aim = 0;
            
            for directions in input.iter() {
                let func_input: Vec<String> = directions.split(" ").map(|s| s.to_string()).collect();
                let direction = func_input[0].to_string();
                let distance = func_input[1].parse::<i32>().unwrap();
        
                match direction.as_ref() {
                    "forward" => {
                        if aim != 0 {
                            y_position = y_position + (distance * aim);
                            x_position += distance
                        } else {
                            x_position += distance
                        }
                    },
                    "up" => {
                        aim -= distance;
                    },
                    "down" => { 
                        aim += distance
                    },
                    _ => panic!("could not understand direction: {}", direction)
                } 
            }
            return x_position * y_position
        },
        _ => panic!("could not understand part {}", part),
    }
}