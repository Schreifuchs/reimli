use regex::Regex;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let wordlist = get_wordlist();

    println!("{:?}", dbg!(get_rime(&"gigu", &wordlist, 2)).get(0))
}

fn get_rime(word: &str, wordlist: &Vec<String>, depth: usize) -> Vec<String> {
    let mut rimes: Vec<String> = wordlist.clone();
    let re = Regex::new(r"\w*oss\b").unwrap();
    rimes.retain(|s| dbg!(re.is_match(dbg!(s))));
    rimes
}

fn get_wordlist() -> Vec<String> {
    let mut word_list: Vec<String> = vec![];
    if let Ok(lines) = read_lines("berndeutsch.csv") {
        for line in lines {
            if let Ok(word) = line {
                word_list.push(word);
            }
        }
    }
    word_list
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
