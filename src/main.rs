use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

fn main() {
    let args = &args().collect::<Vec<String>>();
    if args.len() <= 1 {
        println!("No file inputted.");
        exit(1);
    }

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => {
            println!("couldn't open {}: {}", display, why);
            exit(1);
        }
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            println!("couldn't read {}: {}", display, why);
            exit(1);   
        },
        Ok(_) => interpret(s),
    }
}

fn interpret(s: String) {
    let mut memory: HashMap<i64, u8> = HashMap::new();
    memory.insert(0, 0);

    interpret_some(s, &mut memory, 0, false);
}

fn interpret_some(
    mut s: String,
    memory: &mut HashMap<i64, u8>,
    mut ptr_loc: i64,
    is_in_brackets: bool,
) -> i64 {
    let mut instrc_loc: usize = 0;

    while instrc_loc < s.len() {
        let c: char = s.char_indices().nth(instrc_loc).unwrap().1;
        match c {
            '>' => {
                ptr_loc += 1;
                if !memory.contains_key(&ptr_loc) {
                    memory.insert(ptr_loc, 0);
                }
                instrc_loc += 1;
            }
            '<' => {
                ptr_loc -= 1;
                if !memory.contains_key(&ptr_loc) {
                    memory.insert(ptr_loc, 0);
                }
                instrc_loc += 1;
            }
            '+' => {
                let mut cur = *memory.get(&ptr_loc).unwrap();

                if cur == 255 {
                    cur = 0;
                } else {
                    cur += 1;
                }

                *memory.get_mut(&ptr_loc).unwrap() = cur;

                instrc_loc += 1;
            }
            '-' => {
                let mut cur = *memory.get(&ptr_loc).unwrap();

                if cur == 0 {
                    cur = 255;
                } else {
                    cur -= 1;
                }

                *memory.get_mut(&ptr_loc).unwrap() = cur;

                instrc_loc += 1;
            }
            '.' => {
                // println!("{}", memory[&ptr_loc]);
                print!("{}", memory[&ptr_loc] as char);
                instrc_loc += 1;
            }
            ',' => {
                match std::io::stdin().bytes().next().unwrap() {
                    Ok(input) => memory.insert(ptr_loc, input),
                    Err(why) => {
                        println!("{}", why);
                        Some(0)
                    }
                };
                instrc_loc += 1;
            }
            '[' => {
                let mut bracket_level = 0;
                let start_index = instrc_loc + 1;
                loop {
                    let cur_char = s.char_indices().nth(instrc_loc).unwrap().1;

                    match cur_char {
                        '[' => bracket_level += 1,
                        ']' => bracket_level -= 1,
                        _ => {}
                    }

                    if bracket_level == 0 {
                        if memory[&ptr_loc] == 0 {
                            instrc_loc += 1;
                            break;
                        }

                        let next_eval = unsafe { s.get_unchecked_mut(start_index..instrc_loc) };
                        ptr_loc = interpret_some(String::from(next_eval), memory, ptr_loc, true);
                        instrc_loc += 1;
                        break;
                    }

                    instrc_loc += 1;
                }
            }
            _ => instrc_loc += 1,
        }
    }

    if is_in_brackets {
        if memory[&ptr_loc] != 0 {
            ptr_loc = interpret_some(s, memory, ptr_loc, true)
        }
    }
    return ptr_loc;
}
