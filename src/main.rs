use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use sequence::{Options, transcribe_sequence};

fn main() {
    let strand = Seq::from_args();
    match strand {
        Seq::Transcribe {file, is_dna, seq_len} => {
            let contents = fs::read_to_string(&file).unwrap();
            println!("count: {:?}", contents.chars().count());
            let sequence = contents.replace("\n", "");
            let result = transcribe_sequence(sequence, Options { seq_len, is_dna });
            println!("transcript: {}", result);
        },
        Seq::Translate {} => {},
        Seq::Identify {file } => {},
    }
}

#[derive(StructOpt)]
enum Seq {
    Transcribe {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
        #[structopt(long)]
        is_dna: bool,
        #[structopt(default_value = "100", long)]
        seq_len: usize
    },
    Translate {

    },
    Identify {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    }
}
