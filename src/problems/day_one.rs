use crate::problems::utility;

pub fn depth(args: Vec<String>, part: String) -> i32 {
    let input = utility::clean_array(args);

    match part.as_ref() {
        "one" => {
            let mut ans = 0;
            for (i, distance) in input.iter().enumerate() {
                if i == 0 {
                    continue 
                } else {
                    let conv_distance: i32 = distance.parse().unwrap();
                    if conv_distance > input[i - 1].parse().unwrap() {
                        ans += 1;
                    }
                }
            }
            return ans
        },
        "two" => {
            let window: i32 = 3;
            let mut ans = 0;
            for (i, distance) in input.iter().enumerate() {
                if (i + window as usize) == input.len() {
                    break;
                } else {
                    let base: i32 = distance.parse::<i32>().unwrap() + input[i + 1].parse::<i32>().unwrap() + input[i + 2].parse::<i32>().unwrap();
                    let next: i32 = input[i + 1].parse::<i32>().unwrap() + input[i + 2].parse::<i32>().unwrap() + input[i + 3].parse::<i32>().unwrap();
                    if next > base {
                        ans += 1;
                    }
                }
            }
            return ans
        },
        _ => panic!("could not understand part {}", part),
    }
}
