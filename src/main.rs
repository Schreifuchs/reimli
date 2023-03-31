use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let wordlist = get_wordlist();

    dbg!(get_all_rimes(&"gigu", &wordlist));
}

fn get_all_rimes(word: &str, wordlist: &Vec<String>) -> Vec<Vec<String>> {
    let mut rimes: Vec<Vec<String>> = vec![];

    for depth in (1..dbg!(word.len())).rev() {
        rimes.push(get_rime_for_depth(word, wordlist, depth));
    }
    for level in 1..rimes.len() {
        let next_rimes = rimes[level - 1].clone();
        for i in (level)..rimes.len() {
            rimes[i].retain(|word| !next_rimes.contains(word));
        }
    }

    rimes
}

fn get_rime_for_depth(word: &str, wordlist: &Vec<String>, depth: usize) -> Vec<String> {
    let mut rimes: Vec<String> = wordlist.clone();
    let re = Regex::new(&(r"\w*".to_owned() + &word[word.len() - depth..] + &r"\b")).unwrap();
    rimes.retain(|s| re.is_match(s));
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
