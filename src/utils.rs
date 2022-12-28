
// utility functions and macros

use std::process;

use reqwest;
use reqwest::header::COOKIE;
use std::fs;
use std::path::Path;


pub fn hello_utils() {
    println!("Hello, utils!");
}

const YEAR: isize =  2021;
const PATH_SESSION: &str = "./res/.SESSION";
const PATH_INPUTS: &str =  "./res/inputs/";
const URL_AOC: &str =  "https://adventofcode.com/";
const URL_AOC_DAY: &str =  "/day/";
const URL_AOC_INPUT: &str =  "/input";
// const URL_AOC_INPUT: &str =  "https://adventofcode.com/{}/day/{}/input";

pub fn get_session() -> String {
    return include_str!("../res/.SESSION").to_owned();
}

pub fn get_input_from_aoc(day: usize) -> String {
    println!("Fetching input for day {} from AoC.", day);
    let url = format!("{}{}{}{}{}", URL_AOC, YEAR, URL_AOC_DAY, day, URL_AOC_INPUT);
    let path = format!("{}{}/day{}", PATH_INPUTS, YEAR, day);
    let path = Path::new(path.as_str());

    let client = reqwest::blocking::Client::new();
    let text = client.get(url.as_str())
        .header(COOKIE, format!("session={}", get_session()))
        .send()
        .unwrap()
        .text()
        .unwrap();

    // Cache input in a file.
    println!("Success! Caching input for day {} in a file", day);
    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("Error when trying to create directories. Failed with error: '{}'", e);
        }
    }
    // match File::create(path) {
    //     Ok(mut file) => {
    //         
    //         if let Err(e) = file.write_all(text.as_bytes()) {
    //             eprintln!("Error when trying to cache input for day {}. Failed with error: '{}'", day, e);
    //         }
    //     },
    //     Err(e) =>  eprintln!("Error when trying to create input-cache for day {}. Failed with error: '{}'", day, e),
    // }
    if let Err(e) = fs::write(path, &text) {
        eprintln!("Error when trying to cache input for day {}. Failed with error: '{}'", day, e);
    }
    
    return text;
}

pub fn get_input(day: usize) -> String {
    println!("Retrieving input for day {}.", day);
    let path = format!("{}{}/day{}", PATH_INPUTS, YEAR, day);
    let path = Path::new(path.as_str());

    // let mut text = String::new();
    // if let Ok(file) = fs::File::open(&path) {
    //     println!("Found cached input for day {}. Reading contents.", day);
    //     // File exists! Use the cached input instead of badgering AoC.
    //     let mut buf_reader = BufReader::new(file);
    //     if let Err(_) = buf_reader.read_to_string(&mut text) { // read contents into `text`
    //         eprintln!("ERROR! Unable to read cached input for day {}.", day);
    //         text = get_input_from_aoc(day); // Use AoC as backup, anyway, if it fails.
    //     }
    // } else { // input not fetched yet. GET from AoC
    //     println!("WARNING! Cached input not found for day {}.", day);
    //     text = get_input_from_aoc(day);
    // }
    // return text;
    return match fs::read_to_string(path) {
        Ok(str) => {
            println!("Succesfully read cached input for day {}.", day);
            str
        },
        Err(e) => {
            eprintln!("Error when trying to read cached input for day {}. Failed with error: '{}'", day, e);
             // Fetch (& cache) input from AoC when unable to retrieve a cached copy.
            get_input_from_aoc(day)
        },
    };
    
}

pub trait Day: std::fmt::Debug {
    fn eval(&self, input: &str) -> Result<(), ()> {
        self.part_1(input)?;
        self.part_2(input)?;
        return Ok(());
    }
    fn part_1(&self, _input: &str) -> Result<String, ()> {Err(())}
    fn part_2(&self, _input: &str) -> Result<String, ()> {Err(())}
}
// pub trait Day_Better: std::fmt::Debug {
//     fn eval(&self, input: &str) {
//         for (i, part) in self.get_parts().into_iter().enumerate() {
//             println!("part{}: {}", i, part(input));
//         }
//     }
//     fn get_parts(&self) -> Vec<dyn Fn(&self, &str) -> &str> {}
// }

pub fn get_lines<'a>(s: &'a String) -> Vec<&'a str> {
    return s.lines().collect::<Vec<&str>>();
}

pub fn get_words<'a>(s: &'a String) -> Vec<&'a str> {
    return s.split_whitespace().collect::<Vec<&str>>();
}

pub fn parse_vec(words: &Vec<&str>) -> Vec<i32> {
    let freqs = words.iter().map(|s| { s.parse::<i32>() });
    let unwrapped = freqs.collect::<Result<Vec<i32>, _>>().unwrap_or_else(|e| {
        eprintln!("Application error: {}", e);
        process::exit(1);});
    
    return unwrapped;
}

pub fn parse_and_sum(words: &Vec<&str>) -> i32 {
    let sum = parse_vec(&words).iter().sum();
    
    return sum;
}

use std::time::Instant;
pub struct Timer<'a> {
    name: &'a str,
    start: Instant,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        Timer { name, start: Instant::now() }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        println!("{}: {}", self.name, self.start.elapsed().as_secs());
    }
}




// } // end of 'mod utils'