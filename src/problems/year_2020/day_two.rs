#[derive(Debug, Clone)]
pub struct Password {
    low: usize,
    high: usize,
    letter: String,
    password: String
}

impl Password {
    pub fn parse_input(input: String) -> Password {
        let strs: Vec<String> = input.split(' ').map(|x| x.to_string()).collect();
        let password = &strs[strs.len() - 1];
        let pass_password = password.clone();
        let char = (strs[1].as_bytes()[0] as char).to_string();
        let range: Vec<usize> = strs[0].split('-')
                                       .map(|x| x.parse().unwrap())
                                       .collect();
        let low = range[0];
        let high = range[range.len() - 1];
        
        Password { 
            low: low, 
            high: high, 
            letter: char, 
            password: pass_password
        }
    }

    pub fn check(&mut self) -> bool {
        if !self.password.contains(&self.letter) {
            return false 
        } else {
            let count = self.password.matches(&self.letter).count(); 
            return (count >= self.low) && (count <= self.high)
        }
    }

    pub fn check_position(&mut self) -> bool {
        if self.low > self.password.len() {
            return false 
        } else if self.high > self.password.len() {
            return false 
        }
        let low_position = self.password.chars().nth(self.low - 1).unwrap().to_string();
        let high_position = self.password.chars().nth(self.high - 1).unwrap().to_string();

        if low_position == self.letter && high_position == self.letter {
            return false 
        } else if low_position == self.letter && high_position != self.letter {
            return true 
        } else if low_position != self.letter && high_position == self.letter {
            return true 
        } else {
            return false
        }
    }
}

pub fn flight(args: Vec<String>, part: String) -> i32 {
    match part.as_ref() {
        "one" => {
            let mut pword_vec: Vec<Password> = Vec::new();
            for st in args.iter() {
                let pword = Password::parse_input(st.to_string());
                pword_vec.push(pword);
            }

            let mut correct_pwords = 0;
            for pword in pword_vec.iter() {
                let mut clone_pword = pword.clone();
                if clone_pword.check() {
                    correct_pwords += 1;
                }
            }
            correct_pwords
        },
        "two" => {
            let mut pword_vec: Vec<Password> = Vec::new();
            for st in args.iter() {
                let pword = Password::parse_input(st.to_string());
                pword_vec.push(pword);
            }

            let mut correct_pwords = 0;
            for pword in pword_vec.iter() {
                let mut clone_pword = pword.clone();
                if clone_pword.check_position() {
                    correct_pwords += 1;
                }
            }
            correct_pwords
        }
        _ => panic!("could not understand part {}", part)
    }
}