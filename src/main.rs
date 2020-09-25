mod ffi;
mod reference;
mod trie;
use ffi::{get_lineno, get_val, open_file, safe_yylex, EOL};
use reference::{CodeLine, Reference};
use std::collections::{BTreeSet, HashMap, HashSet};
use trie::Trie;

fn train(in_idx: Option<Trie<i32, String>>) -> Trie<i32, String> {
    // make a new trie which we will be returning
    let mut ret = match in_idx {
        Some(s) => s,
        None => Trie::new(),
    };

    // make an iterator for stepping through the trie, and initially set it to next of current trie
    // we are wasting space of val of root here though
    let mut iter: &mut HashMap<i32, Trie<i32, String>> = &mut ret.next; // name iter better

    while let Some(token) = safe_yylex() {
        if token == EOL {
            iter = &mut ret.next;
        } else {
            // if it has already encountered this type of token before
            if iter.contains_key(&token) {
                // first get corresponding set
                let t = iter.get_mut(&token).unwrap();
                let val = get_val(); // get current value of token
                match t.val.get_mut(&val) {
                    // if already seen this val here
                    Some(set) => {
                        set.insert(Reference::current());
                    }
                    // add the val
                    None => {
                        let mut set = HashSet::new();
                        set.insert(Reference::current());
                        t.val.insert(val, set);
                    }
                }
                iter = &mut (t.next);
            } else {
                // seeing the token first time
                // get the value
                let val = get_val();
                let mut set = HashSet::new();
                set.insert(Reference::current());
                // make a new trie
                let mut t = Trie::new();
                t.val.insert(val, set);
                iter.insert(token, t);
                iter = &mut (iter.get_mut(&token).unwrap().next);
            }
        }
    }

    ret
}

fn eval(
    index: &Trie<i32, String>,
    token_score: i32,
    match_score: i32,
    compare: &dyn Fn(&str, &str) -> bool,
) -> Vec<(CodeLine, BTreeSet<Reference>)> {
    let mut iter: &HashMap<i32, Trie<i32, String>> = &index.next;

    let mut score: i32 = 0;
    let mut max_score: i32 = 0;

    let mut skip: bool = false;

    let mut ret: Vec<(CodeLine, BTreeSet<Reference>)> = Vec::with_capacity(50);
    let mut temp = BTreeSet::new();
    while let Some(token) = safe_yylex() {
        if token == EOL {
            let s = (score as f32 / max_score as f32) * 100.0;
            if s.is_nan() {
                continue;
            }
            let c = CodeLine {
                line: get_lineno(),
                score: s,
            };
            ret.push((c, temp));
            temp = BTreeSet::new();
            score = 0;
            max_score = 0;
            skip = false;
            iter = &index.next;
        } else {
            max_score += token_score + match_score;

            if skip {
                continue;
            }

            match iter.get(&token) {
                // same token was found
                Some(trie) => {
                    score += token_score;
                    let val = get_val();
                    let mut matched = false;
                    for (key, set) in trie.val.iter() {
                        if compare(key, &val) {
                            if !matched {
                                score += match_score;
                                matched = true;
                            }
                            for x in set.iter() {
                                temp.insert(x.clone());
                            }
                        }
                    }
                    iter = &trie.next;
                }
                None => {
                    // different token was found
                    skip = true;
                }
            }
        }
    }
    ret
}

fn print_use() {
    println!(
        r#"Plagiarism Detector
Usage : (binary name) command options
1. train input_file_path : trains on the file at input_file_path, and saves the index as index.json in executing folder.
2. update index_file_path input_file_path : updates the index in file index_file_path by training on file at input_file_path and re-writes the index file.
3. test index_file_path input_file_path : loads index from index_file_path and tests for matches in file input_file_path"#
    );
}

fn main() {
    // Setup
    let match_metric = |x: &str, y: &str| {
        let _x = x.to_lowercase();
        let _y = y.to_lowercase();
        _x.contains(&_y) || _y.contains(&_x)
    };
    // How much score to give if we match token type
    let token_score = 1 as i32;
    // how much score to give if we match token value
    let max_score = 1 as i32;
    //filter condition
    let cutoff = 50.0;

    // Actual split
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        print_use();
        std::process::exit(1);
    }
    match args[1].as_str() {
        "test" => {
            let idx: Trie<i32, String> = match std::fs::read_to_string(&args[2]) {
                Ok(s) => match serde_json::from_str(&s) {
                    Ok(i) => i,
                    Err(e) => {
                        println!("Error : {}", e);
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    println!("Error : {}", e);
                    std::process::exit(1);
                }
            };
            open_file(&args[3]);
            let mut temp = eval(&idx, token_score, max_score, &match_metric);
            // primary filtering
            temp = temp
                .into_iter()
                .filter(|(c, _): &(CodeLine, _)| c.score > cutoff)
                .collect();
            for (line, refer) in temp.iter() {
                println!(
                    "Line {} of code matched with {:.2} % with ",
                    line.line, line.score
                );
                for reference in refer.iter() {
                    println!("\t{}", reference);
                }
                println!();
            }
        }
        "train" => {
            open_file(&args[2]);
            let idx = train(None);
            println!("saving index...");
            let idx_str = serde_json::to_string(&idx).unwrap();
            std::fs::write("./index.json", idx_str).unwrap();
        }
        "update" => {
            let idx: Option<Trie<i32, String>> = match std::fs::read_to_string(&args[2]) {
                Ok(s) => match serde_json::from_str(&s) {
                    Ok(i) => Some(i),
                    Err(e) => {
                        println!("Error : {}", e);
                        None
                    }
                },
                Err(e) => {
                    println!("Error : {}", e);
                    None
                }
            };
            open_file(&args[3]);
            let idx = train(idx);
            println!("saving index...");
            let idx_str = serde_json::to_string(&idx).unwrap();
            std::fs::write(&args[2], idx_str).unwrap();
        }
        _ => {
            print_use();
            std::process::exit(1);
        }
    }
}
