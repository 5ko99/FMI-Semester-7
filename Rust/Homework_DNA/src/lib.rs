pub struct NucleotideCounter {
    pub a: usize,
    pub c: usize,
    pub g: usize,
    pub t: usize,
}

impl NucleotideCounter {
    pub fn print(&self) -> () {
        println!("A={} C={} G={} T={}", self.a, self.c, self.g, self.t);
    }

    pub fn new() -> NucleotideCounter {
        NucleotideCounter {
            a: 0,
            c: 0,
            g: 0,
            t: 0,
        }
    }
}

pub fn counts(dna: &[char]) -> NucleotideCounter {
    let mut counter: NucleotideCounter = NucleotideCounter::new();
    for i in dna {
        match &i {
            'A' => counter.a += 1,
            'C' => counter.c += 1,
            'G' => counter.g += 1,
            'T' => counter.t += 1,
            _ => panic!("Мутант!")
        };
    }
    counter
}

pub fn dna_complement(dna: &[char]) -> Vec<char> {
    let mut result: Vec<char> = vec![];
    for i in dna {
        match &i {
            'A' => result.push('T'),
            'C' => result.push('G'),
            'G' => result.push('C'),
            'T' => result.push('A'),
            _ => panic!("Няма съвпадение за теб, нито тук, нито в Тиндър! :(")
        };
    }
    result
}

pub fn reverse_rna_complement(dna: &[char]) -> Vec<char> {
    let mut result : Vec<char> = vec![];
    for i in dna {
        match &i {
            'A' => result.push('U'),
            'C' => result.push('G'),
            'G' => result.push('C'),
            'U' => result.push('A'),
            _ => panic!("Няма съвпадение за теб, нито тук, нито в Тиндър! :(")
        };
    }
    result.reverse();
    result
}