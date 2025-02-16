//! # create_random_fastq.rs
//!
//! Create random fastq file of various sizes.
//! Output should be fixed using a given seed.
//! Used for testing other code via shell piping stdout

use std::io;

use rand::prelude::*;
use rand_pcg::Pcg64;

use seq_io::fastq::{Record, OwnedRecord};

use subsample_fasta::*;


use clap::{Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

  #[arg(short, long, default_value_t = 10)]
  seed: u64,

  #[arg(short, long, default_value_t = 10)]
  num_seq: u64,

  #[arg(long, default_value_t = 50)]
  min_size: usize,

  #[arg(long, default_value_t = 500)]
  max_size: usize,

}



fn main() {

  let cli = Cli::parse();
  let stdout = io::stdout();
  let mut rng = Pcg64::seed_from_u64(cli.seed);
  let n = cli.num_seq; // number of random sequences
  let min_size = cli.min_size;
  let max_size = cli.max_size;

  for i in 0..n {
    let rec_head = [b"Random_", {i+1}.to_string().as_bytes()].concat();
    let seq_len : usize = rng.gen_range(min_size..max_size);
    let rec_seq = generate_random_nt(&mut rng, seq_len);
    let rec_qual = generate_random_qual(&mut rng, seq_len);
    let rec = OwnedRecord{head: rec_head, seq: rec_seq, qual:rec_qual};
    _ = rec.write(&stdout);
  }


}
