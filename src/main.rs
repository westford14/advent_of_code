use structopt::StructOpt;
use std::path::PathBuf;
use std::fs;
use crate::problems::year_2020;
use crate::problems::year_2021;

mod problems;

#[derive(Debug, StructOpt)]
#[structopt(name = "advent", about = "advent of code command line parser")]
struct Cli {
    // day of the advent of code 
    #[structopt(short = "y", long = "year")]
    year: String, 

    #[structopt(short = "d", long = "day")]
    day: String,

    #[structopt(short = "p", long = "part")]
    part: String,

    #[structopt(short = "f", long = "file")]
    input: PathBuf,

    #[structopt(short = "e", long = "extra", default_value = "18")]
    extra: u16,
}

fn main() {
    let args = Cli::from_args();
    let contents = fs::read_to_string(args.input).unwrap();
    let func_input: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    match args.year.as_ref() {
        "2020" => {
            match args.day.as_ref() {
                "one" => println!("answer: {}", year_2020::day_one::sum(func_input, args.part)),
                "two" => println!("answer: {}", year_2020::day_two::flight(func_input, args.part)),
                _ => panic!(
                    "could not understand request for args year: {}, day: {}, part: {}", args.year, args.day, args.part
                )
            }
        }
        "2021" => {
            match args.day.as_ref() {
                "one" => println!("answer: {}", year_2021::day_one::depth(func_input, args.part)),
                "two" => println!("answer: {}", year_2021::day_two::dive(func_input, args.part)),
                "three" => println!("answer: {}", year_2021::day_three::fuel(func_input, args.part)),
                "four" => println!("answer: {}", year_2021::day_four::bingo(func_input, args.part)),
                "five" => println!("answer: {}", year_2021::day_five::vents(func_input, args.part)),
                "six" => println!("answer: {}", year_2021::day_six::lantern_fish(contents, args.part, args.extra)),
                "seven" => println!("answer: {}", year_2021::day_seven::fuel(contents, args.part)),
                "eight" => println!("answer: {}", year_2021::day_eight::digits(contents, args.part)),
                "nine" => println!("answer: {}", year_2021::day_nine::smoke(func_input, args.part)),
                _ => panic!(
                    "could not understand request for args year: {}, day: {}, part: {}", args.year, args.day, args.part
                ),
            }
        }
        _ => panic!("could not understand year: {}", args.year)
    }
}
