use itertools::Itertools;

pub fn sum(args: Vec<String>, part: String) -> i32 {
    let mut args_int: Vec<i32> = args.iter().map(|x| x.parse().unwrap()).collect();

    match part.as_ref() {
        "one" => {
            loop {
                let number_one = args_int[0];
        
                for n in args_int.iter() {
                    if number_one + n == 2020 {
                        return number_one * n
                    }
                }
                args_int = args_int[1..].to_vec();
            }
        }
        "two" => {
            for pair in args_int.iter().combinations(3) {
                if pair[0] + pair[1] + pair[2] == 2020 {
                    return pair[0] * pair[1] * pair[2];
                }
            }
            return 0
        }
        _ => panic!("could not understand part: {}", part)
    }
}