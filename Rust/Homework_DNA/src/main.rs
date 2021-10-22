mod lib;
use lib::*;

fn main() {
    let input = ['A', 'C', 'G', 'A', 'T'];
    let result = counts(&input);
    result.print();
    let complement = dna_complement(&input);
    println!("{:?}",complement);
}
