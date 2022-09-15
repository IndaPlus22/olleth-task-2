use std::io::{self, BufRead};   
fn main() {
    let input = io::stdin();
    let mut lines = input.lock().lines();

    while let Some(line) = lines.next() {
        let numbers = line.unwrap();

        if numbers.len() == 0 {
            break;
        }
        difference(&numbers)
    }
}

fn difference(num: &String) {
    let num: Vec<i128> = num.trim()
    .split(" ")
    .map(|x| x.parse().expect("Could not parse numbers!"))
    .collect();
    println!("{}", (num[0]-num[1]).abs());
}