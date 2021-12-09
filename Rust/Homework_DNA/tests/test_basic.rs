use homework_dna::*;

#[test]
#[cfg(test)]
fn test_basic() {
    let input: Vec<char> = "GC".chars().collect();
    let counter = counts(&input);

    assert_eq!(counter.g, 1);
    assert_eq!(counter.c, 1);
    assert_eq!(counter.a, 0);
    assert_eq!(counter.t, 0);

    assert_eq!(dna_complement(&input),         vec!['C', 'G']);
    assert_eq!(reverse_rna_complement(&input), vec!['G', 'C']);
}

#[test]
#[cfg(test)]
fn count_tests() {
    let input : Vec<char> = "GTACAGT".chars().collect();
    let count = counts(&input);

    assert_eq!(count.a, 2);
    assert_eq!(count.c, 1);
    assert_eq!(count.g, 2);
    assert_eq!(count.t, 2);
}

#[test]
#[cfg(test)]
fn dna_complement_tests() {
    let input : Vec<char> = "GGATTACCGA".chars().collect();
    assert_eq!(dna_complement(&input),vec!['C','C','T','A','A','T','G','G','C','T']);
}

#[test]
#[cfg(test)]
fn rnk_tests() {
    let input1 : Vec<char> = "GGATTACCGA".chars().collect();
    let input2 : Vec<char> = "GTACAGT".chars().collect();
    assert_eq!(vec!['U','C','G','G','U','A','A','U','C','C'],reverse_rna_complement(&input1));
    assert_eq!(vec!['A','C','U','G','U','A','C'],reverse_rna_complement(&input2));
}

#[test]
#[cfg(test)]
#[should_panic]
fn wrong_input() {
    let input1 : Vec<char> = "ACGTEED".chars().collect();
    let input2 : Vec<char> = "GTACAGTП".chars().collect();
    let input3 : Vec<char> = "GGATTACCGA".chars().collect();
    let input4: Vec<char> = "ACGTe".chars().collect();
    counts(&input1);
    dna_complement(&input1);
    reverse_rna_complement(&input1);
    reverse_rna_complement(&input2);
    reverse_rna_complement(&input3);
    counts(&input2);
}