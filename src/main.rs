use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let wordlist = get_wordlist();

    while true {
        let mut word = String::new();

        io::stdin()
            .read_line(&mut word)
            .expect("Fehler beim lesen der Zeile");

        print_rimes(&get_all_rimes(dbg!(&word.trim()), &wordlist));
    }
}
fn print_rimes(rimes: &Vec<Vec<String>>) {
    for line in rimes {
        for word in line {
            print!("{},", word);
        }
        print!("\n-----------------------------------------\n");
    }
}

fn get_all_rimes(word: &str, wordlist: &Vec<String>) -> Vec<Vec<String>> {
    let mut rimes: Vec<Vec<String>> = vec![];

    for depth in (1..dbg!(dbg!(word).len())).rev() {
        rimes.push(get_rime_for_depth(word, wordlist, depth));
    }

    //Delete duplicates
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
    //let re = Regex::new(&(r"\w*".to_owned() + &word[word.len() - depth..] + &r"\b")).unwrap();
    rimes.retain(|s| check_for_depth(s, word, depth));
    rimes
}

fn check_for_depth(word: &str, word_two: &str, depth: usize) -> bool {
    let chars: Vec<char> = word.chars().collect();
    let chars_two: Vec<char> = word_two.chars().collect();

    if chars.len() < depth || chars_two.len() < depth {
        return false;
    }
    for i in (chars.len() - depth)..chars.len() {
        for j in (chars_two.len() - depth)..chars_two.len() {
            if chars.get(i) != chars_two.get(j) {
                return false;
            }
        }
    }
    true
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
