use std::{env, fs};

use merge_sat::clause::Clause;
use merge_sat::instance::Instance;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");
    let lines = contents.split("\n");
    let mut clauses: Vec<Clause> = vec![];
    let mut m: u32 = 0;
    let mut n: u32 = 0;

    for line in lines {
        let chars: Vec<&str> = line.split(" ").collect();
        if chars[0] == "c" {
            continue;
        } else if chars[0] == "p" {
            m = chars[2]
                .trim()
                .parse()
                .expect("Got non-integer number of variables.");
            n = chars[3]
                .trim()
                .parse()
                .expect("Got non-integer number of clauses.");
        } else if chars.len() > 1 {
            let literals: Vec<i64> = chars[1..(chars.len() - 1)]
                .into_iter()
                .map(|x| x.trim().parse().expect("Got invalid literal."))
                .collect();
            clauses.push(Clause::new(literals));
        }
    }
    let instance = Instance::new(clauses);
    println!("Read CNF instance with {} variables in {} clauses.", m, n);
    println!("{}", instance);
}
