use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::collections::HashMap;

pub struct Config {
    pub input: String,
}

const LETTER_HEIGHT: usize = 17;
const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz.!";

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        // let input = args.clone()[1..].join(" ");
        let input = args[1..].join(" ");

        Ok(Config { input })
    }
}

fn parse_letters(font: &str) -> HashMap<char, Vec<&str>> {

    let mut map = HashMap::new();

    for character in LETTERS.chars().enumerate() {
        let (i, c) = character;
        let first = i * LETTER_HEIGHT;
        let last = first + LETTER_HEIGHT;
        let mut result = Vec::new();
        for n in first..last {
            let line = font.lines().nth(n).expect("Failed to retrieve line");
            result.push(line)
        }
        map.insert(c, result);
    }
    return map;
}

fn read_font() -> Result<String, Box<Error>> {

    let mut file = File::open("resources/chars.latin1")?;
    let mut font = String::new();
    file.read_to_string(&mut font)?;

    Ok(font)
}

// 1. read files in one succinct operation
// 2. iterate over input string LETTER_HEIGHT and concact each line into a single string
// 3. print strings
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let font = read_font()?;
    let parsed_font = parse_letters(&font[..]);

    let input = &config.input;

    // println!("{:?}", chars.get(&']').ok_or("Couldn't retrieve character from parsed font")?);
    for n in 0..LETTER_HEIGHT {
        for input_char in input.chars() {
            let output_char = parsed_font.get(&input_char).ok_or("Couldn't retrieve character from parsed font")?;
            print!("{}", output_char[n]);
        }
        println!();
    }

    // }
    // let mut a_result: Vec<&str> = Vec::new();
    // let mut b_result: Vec<&str> = Vec::new();

    // for line in aa.lines() {
    //     a_result.push(line);
    // }

    // for line in bb.lines() {
    //     b_result.push(line);
    // }

    // for asd in a_result.iter().zip(b_result.iter()) {
    //     let (a, b) = asd;
    //     print!("{}", a);
    //     print!("{}", b);
    //     println!();
    // }


    // println!("{}", result);


    // let mut iter = bb.chars().enumerate().map(|(i, c)| if i % 18 == 0 {

    // } else {
    //     c
    // });
//     fn titlecase_word(word: &str) -> String {
//         word.chars().enumerate()
//             .map(|(i, c)| if i == 0 { c.to_uppercase() } else { c.to_lowercase() })
//             .collect()
// }

    // let mut result = String::new();
    // for (i, c) in bb.chars().enumerate() {
    //     if i % 18 == 0 {
    //         result.push(c);
    //         result.push('\n');
    //     } else {
    //         result.push(c);
    //     }
    // }

    // println!("{}", result);

    // println!("{}{}\n", aa, bb);

    // for line in contents. {
    //     println!("{}", line);
    // }

    // println!("{}", &config.input);
    // let results = if config.case_sensitive {
    //     search(&config.query, &contents)
    // } else {
    //     search_case_insensitive(&config.query, &contents)
    // };


    // println!("{}", &config.input);
    // let results = if config.case_sensitive {
    //     search(&config.query, &contents)
    // } else {
    //     search_case_insensitive(&config.query, &contents)
    // };

    // for line in results {
    //     println!("{}", line);
    // }

    Ok(())
}

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }
//     results
// }

// fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let query = query.to_lowercase();
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             results.push(line);
//         }
//     }

//     results
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn case_sensitive() {
//         let query = "duct";
//         let contents = "\
//         Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";

//         assert_eq!(
//             vec!["safe, fast, productive."],
//             search(query, contents)
//         );
//     }

//     #[test]
//     fn case_insensitive() {
//         let query = "rUsT";
//         let contents = "\
//         Rust:
// safe, fast, productive.
// Pick three.
// Trust me.";

//         assert_eq!(
//             vec!["Rust:", "Trust me."],
//             search_case_insensitive(query, contents)
//         );
//     }
// }