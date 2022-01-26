use std::collections::HashSet;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    global: Option<String>,
    #[clap(short, long)]
    require: Option<String>,
    #[clap(short, long)]
    no_repeats: bool,
    filters: Vec<String>,
}

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

fn parse_filter(filter: &str, global: &HashSet<char>) -> HashSet<char> {
    let spec: Vec<_> = filter.split(|c| c == '^').collect();
    let mut search: HashSet<char> = HashSet::new();
    if !spec.is_empty() {
        let include = spec[0];
        if include == "." || include == "" {
            for c in global.iter() {
                search.insert(*c);
            }
        } else {
            for c in include.chars() {
                search.insert(c);
            }
        }
    }
    if spec.len() >= 2 {
        let exclude = spec[1];
        for letter in exclude.chars() {
            search.remove(&letter);
        }
    }
    search
}

fn main() {
    let arguments = Args::parse();

    let base_set: HashSet<char> = arguments.global.map_or_else(
        || ALPHA.chars().collect(),
        |value| parse_filter(&value, &ALPHA.chars().collect()),
    );

    let mut filters: Vec<HashSet<char>> = Vec::new();
    for filter in arguments.filters {
        filters.push(parse_filter(&filter, &base_set))
    }

    let required: HashSet<char> = arguments
        .require
        .map_or(HashSet::new(), |value| value.chars().collect());

    let raw_wordlist = include_str!("wordlist");
    for line in raw_wordlist.lines() {
        let word = line.trim().chars().collect::<Vec<_>>();

        if word.len() != filters.len() || !word.iter().all(|c| c.is_alphabetic()) {
            continue;
        }

        if !word
            .iter()
            .zip(&filters)
            .map(|(letter, rule)| rule.contains(letter))
            .all(|x| x)
        {
            continue;
        }

        if !required.is_empty() && !word.iter().any(|c| required.contains(c)) {
            continue;
        }

        if arguments.no_repeats {
            let unique: HashSet<_> = word.iter().collect();
            if unique.len() < word.len() {
                continue;
            }
        }

        println!("{}", word.iter().collect::<String>());
    }
}
