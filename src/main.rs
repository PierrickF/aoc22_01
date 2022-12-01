use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn main() -> Result<(), Error> {
    let elves = parse_file(File::open("input")?);
    compare_values(elves);
    Ok(())
}

fn parse_file<R: Read>(input: R) -> Vec<Vec<u32>> {
    let br = BufReader::new(input);
    let mut elves: Vec<Vec<u32>> = Vec::new();
    let mut elf: Vec<u32> = Vec::new();
    for line in br.lines() {
        if line.as_ref().unwrap() != "" {
            let number: u32 = line.expect("Error reading line").trim().parse().unwrap();
            elf.push(number);
        } else {
            elves.push(elf.clone());
            elf.clear();
            continue
        }
    }
    elves
}

fn compare_values(vec: Vec<Vec<u32>>) {
    let mut totals: Vec<u32> = Vec::new();
    for elf in vec {
        totals.push(calculate_sum(elf));
    }
    find_largest(&totals);
    find_three_largest(&totals);
}

fn calculate_sum(vec: Vec<u32>) -> u32 {
    vec.iter().sum()
}

fn find_largest(vec: &Vec<u32>) {
    let num = *vec.iter().max().unwrap();
    println!("The largest sum is {}", num.to_string());
}

fn find_three_largest(vec: &Vec<u32>) {
    let mut sorted = vec.clone();
    sorted.sort();
    sorted.reverse();
    let total = sorted[0] + sorted[1] + sorted[2];
    println!("The total of the three largest sum is {total}");
}
