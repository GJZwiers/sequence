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

    let params: StopSiteParams = StopSiteParams {
        len: 8,
        t_region_len: 4 * 8,
        stop_seq: "TTTT",
        stop_seq_len: 4
    };

    let stop_sites: Vec<usize> = find_stop_sites(&dna, params);

    let gene: String = String::from(dna.substring(start_sites[0],  stop_sites[0]));

    let target: String = transcribe(Strand {
        bases: String::from(&gene),
        index: 0,
        is_dna: true,
    });

    println!("{:?}", start_sites);
    println!("{:?}", stop_sites);
    println!("5`-> 3`: {}", gene);
    println!("3`-> 5`: {}", target);

    let substrands: Vec<Strand> = to_subs(target, opts);

    return transcribe_strands(substrands);
}

fn find_start_sites(dna: &String) -> Vec<usize> {
    let minus35: Vec<char> = vec!['T','T','G','A','C','A'];
    let minus10: Vec<char> = vec!['T','A','T','A','T','T'];
    
    let mut i = 0;
    let last_start = dna.chars().count() - 6;
    let mut start_sites: Vec<usize> = vec![];
    while i <= last_start {
        let one = dna.substring(i, i + 6);
        let two = dna.substring(i + 23, i + 6 + 23);
        if is_promotor(one, &minus35) && is_promotor(two, &minus10) {
            let start_site = i + 25 + 10;
            start_sites.push(start_site);
        }
        i += 1;
    }
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

struct StopSiteParams<'a> {
    len: usize,
    t_region_len: usize,
    stop_seq: &'a str,
    stop_seq_len: usize
}

fn find_stop_sites(dna: &String, params: StopSiteParams) -> Vec<usize> {
    let mut i: usize = 0;
    let last_start: usize = dna.chars().count() - params.len;
    let mut stop_sites: Vec<usize> = vec![];
    while i <= last_start {
        let n = i + params.len;
        let first_region: &str = dna.substring(i, n);

        if is_terminator(&first_region) {
            let second_region: &str = dna.substring(n, i + params.t_region_len);
            let palindrome: String = make_palindrome(&first_region);

            if let Some(t) = second_region.find(&palindrome) {
                let third_region: &str = second_region.substring(t, i + params.t_region_len);

                if let Some(j) = third_region.find(params.stop_seq) {
                    let stop_site: usize = n + t + j + params.stop_seq_len;
                    stop_sites.push(stop_site);
                }
            }
        }

        i += 1;
    }
    stop_sites
}

fn make_palindrome(str: &str) -> String {
    let transcript: String = transcribe(Strand {
        bases: String::from(str),
        index: 0,
        is_dna: true
    });
    transcript.chars().rev().collect()
}

fn is_terminator(sequence: &str) -> bool {
    let mut cs: u32 = 0;
    let mut gs: u32 = 0;
    for char in sequence.chars() {
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
