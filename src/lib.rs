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
    let start_sites: Vec<usize> = find_start_sites(&dna);

    let stop_sites = find_stop_sites(&dna);

    let gene = dna.substring(start_sites[0],  stop_sites[0]);
    println!("gene: {}", gene);
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

struct StopSite {
    seq: String,
    len: u32,
    close_seq: String,
    close_len: u32
}

fn find_stop_sites(dna: &String) -> Vec<usize> {
    let mut i = 0;
    let farthest = dna.chars().count() - 8;
    let mut stop_sites: Vec<usize> = vec![];
    while i <= farthest {
        let one: &str = dna.substring(i, i + 8);

        if is_terminator(&one) {
            let transcript: String = transcribe(Strand {
                bases: String::from(one),
                index: 0,
                is_dna: true
            });
            let rev: String = transcript.chars().rev().collect();   // println!("reversed: {}", rev);
            let two = dna.substring(i + 8, i + 36);

            if let Some(t) = two.find(&rev) {
                println!("1st: {}", one);
                println!("2nd: {}", two);
                let three = dna.substring(i + 8 + t, i + 36);
                println!("3rd: {}", three);
                let close = "TTTT";

                if let Some(j) = three.find(close) {
                    let stop_site = i + 8 + t + j + close.chars().count();
                    stop_sites.push(stop_site);
                }
            }
        }

        i += 1;
    }
    println!("{:?}", stop_sites);
    stop_sites
}

fn is_terminator(substr: &str) -> bool {
    let mut cs: u32 = 0;
    let mut gs: u32 = 0;
    for char in substr.chars() {
        if char == 'C' {
            cs += 1;
        } else if char == 'G' {
            gs += 1;
        }
    }

    if cs + gs > 6 && cs > 2 && gs > 2 {
        true
    } else  {
        false
    }
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
