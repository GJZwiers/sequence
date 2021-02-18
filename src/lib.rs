use std::thread;
use std::sync::{Arc, Mutex};
use substring::Substring;

pub enum NucleicAcid {
    DNA,
    RNA
}

pub struct Strand {
    pub bases: String,
    pub index: usize,
    pub is_dna: bool,
}

pub struct Options {
    pub seq_len: usize,
    pub is_dna: bool,
}

pub fn transcribe_sequence(dna: String, opts: Options) -> String {
    let start_sites = find_start_sites(&dna);
    let substrands: Vec<Strand> = to_subs(dna, opts);
    return transcribe_strands(substrands);
}

fn find_start_sites(dna: &String) -> Vec<usize> {
    let minus35: Vec<char> = vec!['T','T','G','A','C','A'];
    let minus10: Vec<char> = vec!['T','A','T','A','T','T'];
    
    let mut i = 0;
    let farthest = dna.chars().count() - 6;
    let mut start_sites: Vec<usize> = vec![];
    while i <= farthest {
        let one = dna.substring(i, i + 6);
        let two = dna.substring(i + 23, i + 6 + 23);
        if is_promotor(one, &minus35) && is_promotor(two, &minus10) {
            let start_site = i + 25 + 10;
            start_sites.push(start_site);
        }
        i += 1;
    }
    println!("{:?}", start_sites);
    start_sites
}

fn is_promotor(substr: &str, consensus: &Vec<char>) -> bool {
    let mut y: u32 = 0;
    let mut n: u32 = 0;
    for (index, char) in substr.chars().enumerate() {
        if char == consensus[index] {
            y += 1;
        } else if n <= 3 {
            n += 1;
        } else {
            return false;
        }
    }
    if y > 3 { true } else { false }
}

fn to_subs(strand: String, opts: Options) -> Vec<Strand> {
    let strand_len: usize = strand.chars().count();

    let mut substrands: Vec<Strand> = Vec::new();
    let mut index: usize = 0;
    while index * opts.seq_len <= strand_len {
        substrands.push(Strand { 
            bases: String::from(strand.substring(
                index * opts.seq_len,
                index * opts.seq_len + opts.seq_len)),
            index,
            is_dna: opts.is_dna
        });
        index += 1;
    }
    substrands
}

fn transcribe_strands(strands: Vec<Strand>) -> String {
    let mut handles = vec![];
    let v = vec![String::from(""); strands.len()];
    let arc: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(v));
    
    for strand in strands {
        let clone = Arc::clone(&arc);
        let handle = thread::spawn(move || {
            let mut total = clone.lock().unwrap();
            let i: usize = strand.index;
            total[i] = transcribe(strand);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    return arc.lock().unwrap().join("");
}

fn transcribe(strand: Strand) -> String {
    let var_base = match strand.is_dna {
        false => "U",
        true =>  "T",
    };
    let mut transcript = String::from("");
    for base in strand.bases.chars() {
        if base == 'A' {
            transcript += var_base;
        } else if base == 'T' {
            transcript += "A";
        } else if base == 'C' {
            transcript += "G";
        } else if base == 'G' {
            transcript += "C";
        } else {
            panic!("Detected unknown nucleotide, either you are an alien or this is a mistake!");
        }
    }
    transcript
}
