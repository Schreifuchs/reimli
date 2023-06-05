use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let wordlist = get_wordlist();

    loop {
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
    let word = word.graphemes(true).collect::<Vec<&str>>();

    let mut matcher = String::new();

    for i in word.len() - depth..word.len() {
        match word.get(i) {
            None => print!("upsi"),
            Some(char) => {
                matcher = matcher + char;
            }
        }
    }

    let re = Regex::new(&(r"\w*".to_owned() + &matcher + &r"\b")).unwrap();
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
