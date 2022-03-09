
use std::fs;
use std::env;
use std::collections::{HashMap, HashSet};
use std::path::Path;

fn main() {

    let args = env::args().collect::<Vec<_>>();

    if args.len() != 3 {
        println!("usage: wsort <src> <dest>");
        return;
    } 

    let src = Path::new(&args[1]);
    let dest = Path::new(&args[2]);

    if !src.exists() {
        println!("src = {}; does not exist", src.display());
    }

    if dest.exists() {
        println!("dest = {}; exists, but should not", dest.display());
    }

    let input = match fs::read_to_string(src) {
        Ok(s) => s,
        Err(e) => panic!("failure while reading src file to string: {}", e),
    };

    let mut dict : HashMap<usize, HashSet<&str>> = HashMap::new();

    for line in input.split("\n") {

        let word = line.trim_end();
        let len = word.len();

        match dict.get_mut(&len) {
            Some(v) => {v.insert(word);},
            None => { 
                let mut words_of_len = HashSet::new();
                words_of_len.insert(word);
                dict.insert(len, words_of_len);
            },
        }

    }

    let mut values = dict.into_iter().collect::<Vec<_>>();
    values.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let output = values.into_iter()
                       .map(|x| x.1)
                       .map(|x| {
                            let mut v = x.into_iter().collect::<Vec<&str>>();
                            v.sort();
                            v.join("\n")
                        })
                        .collect::<Vec<String>>()
                        .join("\n");

    fs::write(dest, output).expect("failure while writing output to dest file");
}
