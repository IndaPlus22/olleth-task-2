use std::io;

fn main() {
    let mut l1 = String::new();
    let mut l2 = String::new();

    io::stdin().read_line(&mut l1).expect("Failed read 1");
    io::stdin().read_line(&mut l2).expect("Failed read 2");

    let mut first_number: usize = l1.trim().parse().expect("Could not parse 1st number");
    if first_number % 2 == 1 { first_number += 1; }

    let mut numbers: Vec<u32> = l2
    .trim()
    .split(" ")
    .map(|x| x.parse().expect("Could not parse numbers!"))
    .collect();
    numbers.sort();
    numbers.reverse();

    let mut answer: u32 = 0;
    for _index in 0..(first_number / 2) { answer += numbers[_index];}

    println!("{}", answer);
}   