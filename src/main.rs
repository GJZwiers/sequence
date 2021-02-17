use substring::Substring;
use std::thread;
use std::sync::{Arc, Mutex};
use std::env;

enum NucleicAcid {
    DNA,
    RNA
}

struct Strand {
    bases: String,
    index: usize,
    kind: NucleicAcid,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dna: String = String::from(&args[1]);
    println!("original: {}", dna);
    let mrna: String = rna(dna);
    println!("transcript: {}", mrna);
}

fn rna(dna: String) -> String {
    let substrands: Vec<Strand> = to_subs(dna);
    return transcribe_strands(substrands);
}

fn to_subs(strand: String) -> Vec<Strand> {
    let strand_len: usize = strand.chars().count();
    let seq_len: usize = 10;

    let mut substrands: Vec<Strand> = Vec::new();
    let mut index: usize = 0;
    while index * seq_len <= strand_len {
        substrands.push(Strand { 
            bases: String::from(
                strand.substring(index * seq_len, index * seq_len + seq_len)),
            index,
            kind: NucleicAcid::DNA
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
    let mut transcript = String::from("");
    for base in strand.bases.chars() {
        if base == 'A' {
            match strand.kind {
                NucleicAcid::DNA => { transcript += "T" }
                NucleicAcid::RNA => { transcript += "U" }
            }
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
