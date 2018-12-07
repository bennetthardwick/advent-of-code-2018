use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("./input").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    part_one(&input);
    part_two(&input);
}

fn count_doubles_and_triples(id: &str) -> (bool, bool) {
    let mut twos = false;
    let mut threes = false;
    let mut character_counts: HashMap<char, u32> = HashMap::new();

    for character in id.chars() {
        let entry = character_counts.entry(character).or_insert(0);
        *entry += 1;
    }

    for (_, &value) in character_counts.iter() {
        if twos && threes { break; }
        if value == 2 {
            twos = true;
        } else if value == 3 {
            threes = true;
        }
    }

    (twos, threes)
}

fn part_one(input: &String) {
    let mut twos = 0;
    let mut threes = 0;

    for id in input.split('\n') {
        let (add_twos, add_threes) = count_doubles_and_triples(id);
        if add_twos {
            twos += 1;
        }
        if add_threes {
            threes += 1;
        }
    }

    println!("\nPart 1!");
    println!("{}", twos * threes);
}

fn part_two(input: &String) {
    let ids = input.split('\n').collect();
    let matches = scan_strings_for_matches(ids).unwrap();
    let matches_string = matches.iter().collect::<String>();

    println!("\nPart 2!");
    println!("{}", matches_string);
}

fn scan_strings_for_matches(ids: Vec<&str>) -> Option<Vec<char>> {
    let mut current_matches: Vec<char>;

    for i in 0..ids.len() - 1 {
        for j in (i + 1)..ids.len() {
            current_matches = find_matches_between_strings(ids[i], ids[j]);

            if ids[i].len() - current_matches.len() == 1 {
                return Some(current_matches);
            }

        }
    }

    None
}

fn find_matches_between_strings(a: &str, b: &str) -> Vec<char> {
    let mut matches = Vec::new();

    let mut a_chars = a.chars();
    let mut b_chars = b.chars();

    let mut a_current;
    let mut b_current;

    loop {
        a_current = a_chars.next();
        b_current = b_chars.next();

        if a_current.is_none() || b_current.is_none() {
            break;
        }

        let _a = a_current.unwrap();
        let _b = b_current.unwrap();

        if _a == _b {
            matches.push(_a);
        } else {
        }
    }

    matches
}
