use structopt::StructOpt;
use std::path::PathBuf;
use std::fs;
use crate::problems::{
    day_one, day_two, day_three, day_four, day_five,
    day_six, day_seven,
};

mod problems;

#[derive(Debug, StructOpt)]
#[structopt(name = "advent", about = "advent of code command line parser")]
struct Cli {
    // day of the advent of code 
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

    match args.day.as_ref() {
        "one" => println!("answer: {}", day_one::depth(func_input, args.part)),
        "two" => println!("answer: {}", day_two::dive(func_input, args.part)),
        "three" => println!("answer: {}", day_three::fuel(func_input, args.part)),
        "four" => println!("answer: {}", day_four::bingo(func_input, args.part)),
        "five" => println!("answer: {}", day_five::vents(func_input, args.part)),
        "six" => println!("answer: {}", day_six::lantern_fish(contents, args.part, args.extra)),
        "seven" => println!("answer: {}", day_seven::fuel(contents, args.part)),
        _ => panic!("could not understand request for args {} {}", args.day, args.part),
    }
}
