use std::{env, fs::File, io::{BufRead, BufReader}};

fn count_lines(f: BufReader<File>) -> usize {
    let mut i: usize = 0;
    for _ in f.lines() {
        i +=1
    }
    i

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let p = &args[1];
    let file = File::open(p).expect("argument should be a valid file");
    


    let read = BufReader::new(file);
    let lc = count_lines(read);
    println!("{lc} {p}");
}
