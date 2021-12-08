use counter::Counter;

#[derive(Clone, Debug)]
pub struct Lantern {
    pub fish: Vec<u8>
}

impl Lantern {
    pub fn calculate_total(&mut self, mut days: u16) -> i64 {
        let mut fish_count = self.fish.iter()
                                             .map(|x| *x)
                                             .collect::<Counter<u8>>();

        while days != 0 {
            let mut new_count= Counter::<u8>::new();

            for (fish, count) in &mut fish_count.iter() {
                if *fish == 0 {
                    new_count[&6] += count;
                    new_count[&8] += count;
                } else {
                    new_count[&(fish - 1)] += count;
                }
            } 
            
            fish_count = new_count.clone();
            days -= 1;
        }
        
        let mut total_fish: i64 = 0;
        for (_key, val) in fish_count.into_iter() {
            total_fish += *val as i64;
        }

        return total_fish
    }
}

pub fn lantern_fish(args: String, part: String, days: u16) -> i64 {
    let input = args.split(",")
                                    .map(|x| x.parse::<u8>().unwrap())
                                    .collect::<Vec<u8>>();
    match part.as_ref() {
        "one" => {
            let mut lantern = Lantern{fish: input};
            return lantern.calculate_total(days);
        },
        _ => panic!("could not understand part: {}", part)
    }
}