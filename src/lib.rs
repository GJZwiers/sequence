use std::str::FromStr;
use std::thread;
use std::sync::{Arc, Mutex};
use substring::Substring;

pub enum NucleicAcid {
    DNA,
    RNA
}

impl FromStr for NucleicAcid {
    type Err = ();

    fn from_str(s: &str) -> Result<NucleicAcid, ()> {
        match s {
            "dna" => Ok(NucleicAcid::DNA),
            "rna" => Ok(NucleicAcid::RNA),
            _ => Err(())
        }
    }
}

pub struct Strand {
    pub bases: String,
    pub index: usize,
    pub is_rna: bool,
}

pub struct Options {
    pub seq_len: usize,
    pub is_rna: bool,
}

pub fn transcribe_sequence(dna: String, opts: Options) -> String {
    let substrands: Vec<Strand> = to_subs(dna, opts);
    return transcribe_strands(substrands);
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
            is_rna: opts.is_rna
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
            match strand.is_rna {
                true =>  { transcript += "U" }
                false => { transcript += "T" }
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
