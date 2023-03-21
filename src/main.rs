use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
fn main() -> std::io::Result<()> {
    let file = File::open("poem.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);

    println!("Uf weles Wörtli bruchsch ä reim?");
    let mut search = String::new();
    io::stdin()
        .read_line(&mut search)
        .expect("Fehler beim Lesen der Zeile");

    println!("{}", search);
    let search_len = search.trim().len();
    let depth;
    if search_len < 2 {
        depth = search_len;
    } else {
        depth = 2
    }
    let search_suffix = &search.trim()[search_len - depth..search_len];

    for line in buf_reader.lines() {
        let linestr = line?;
        let linestr_len = linestr.trim().len();
        if linestr_len >= depth {
            let linestr_suffix = &linestr.trim()[linestr_len - depth..linestr_len];
            if linestr_suffix == search_suffix {
                println!("{}", linestr);
            }
        }
    }

    Ok(())
}
