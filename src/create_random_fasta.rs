//! # create_random_fasta.rs
//!
//! Create random fasta file of various sizes.
//! Output should be fixed using a given seed.
//! Used for testing other code via shell piping stdout

use std::iter;
use std::io;

use rand::prelude::*;
use rand_pcg::Pcg64;

use seq_io::fasta::{Record, OwnedRecord};

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

  #[arg(short, long, default_value_t = 80)]
  wrap_size: usize,

}

fn generate_random_aa(rng : &mut impl Rng, len: usize) -> Vec<u8> {
  // Copied from https://stackoverflow.com/a/74953997
  // Changed to return Vec<u8>
  const STANDARD_AA: &[u8] = b"ACDEFGHIKLMNPQRSTVWY";
  let one_u8 = || STANDARD_AA[rng.gen_range(0..STANDARD_AA.len())] ;
  iter::repeat_with(one_u8).take(len).collect()
}


fn main() {

  let cli = Cli::parse();
  let stdout = io::stdout();
  let mut rng = Pcg64::seed_from_u64(cli.seed); 
  let n = cli.num_seq; // number of random sequences
  let min_size = cli.min_size;
  let max_size = cli.max_size;
  let wrap_size = cli.wrap_size;

  for i in 0..n {
    let rec_head = [b"Random_", {i+1}.to_string().as_bytes()].concat();
    let seq_len : usize = rng.gen_range(min_size..max_size);
    let rec_seq = generate_random_aa(&mut rng, seq_len);
    let rec = OwnedRecord{head: rec_head, seq: rec_seq};
    _ = rec.write_wrap(&stdout, wrap_size);
  }


}
