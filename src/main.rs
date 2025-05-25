use std::{env, fs, process};


fn count_lines(f:  &String) -> usize {
    let mut i: usize = 0;
    for _ in f.lines() { i += 1 }
    i
}

fn count_bytes(f:  &String) -> usize {
    let mut i: usize = 0;
    for _ in f.as_bytes() { i += 1 }
    i
}

fn count_chars(f:  &String) -> usize {
    let mut i: usize = 0;
    for _ in f.chars() { i += 1 }
    i
}

fn count_words (f: &String) -> usize {
    f.split_whitespace().count()
}

fn main() {
    let mut print_lines = false;
    let mut print_bytes = false;
    let mut print_words = false;
    let mut print_chars = false;


    let mut args: Vec<String> = env::args().collect();
    let p: String;
    let file: String;
    if args[1].starts_with("-") {
        p = args[2].clone();
        file = fs::read_to_string(&p).expect("Path should be a valid file");

        let mut letter_args: Vec<char> = Vec::new();
        args[1].remove(0);
        for c in args[1].chars() {
            letter_args.push(c);
        }

        for arg in letter_args {
            match arg {
                'l' => print_lines = true,
                'w' => print_words = true,
                'c' => print_bytes = true,
                'm' => print_chars = true,

                _ => {
                    eprintln!("Uknown argument {arg}");
                    process::exit(1);
                }
            }
        }

        // Here we are printing the results of our last for loop
        
        if print_lines { print!("{} ", count_lines(&file)) }
        if print_words { print!("{} ", count_words(&file)) }
        if print_bytes { print!("{} ", count_bytes(&file)) }
        if print_chars { print!("{} ", count_chars(&file)) }

        println!("{p}");

    } else {
        p = args[1].clone();

        file = fs::read_to_string(&p).expect("Path should be a valid file");
        println!("{} {} {} {}", count_lines(&file), count_words(&file), count_bytes(&file), p);
        // println!("{} {}", count_lines(&file), p)
    }

}
