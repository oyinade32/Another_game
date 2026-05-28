use std::io::{self, Read};
use another_game::winner;

fn main() {

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    let mut output = Vec::with_capacity(t);

    for _ in 0..t {

        let n: usize = iter.next().unwrap().parse().unwrap();

        let mut heaps = Vec::with_capacity(n);

        for _ in 0..n {
            heaps.push(iter.next().unwrap().parse().unwrap());
        }

        output.push(winner(&heaps));
    }
    println!("{}", output.join("\n"));
}