use std;
use std::io;

use rand::prelude::*;
use rand_pcg::Pcg64;

use seq_io::fasta::{Reader};

use subsample_fasta::*;

use clap::{Parser};
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

  #[arg(short, long, default_value_t = 2)]
  num_seq: usize,

// seed : u64, // add a seed if we want reproducibility

}

fn main() {
  
  let _seed: u64 = 25;
  let cli = Cli::parse();

  let k = cli.num_seq;

  //let mut rng = Pcg64::seed_from_u64(seed); 
  let mut rng = Pcg64::from_rng(thread_rng()).unwrap();


  let mut reader = Reader::new(io::stdin());
  
  let mut samples : Vec<Vec<u8>> = Vec::new();
  let mut indices : Vec<usize> = Vec::new();
  
  
  reservoir_sample(&mut rng, k, &mut samples, &mut indices, &mut reader);

  // now print out all the large vecs to stdout
  let stdout = io::stdout();
  let _ = write_vecs(&stdout, &samples, &indices);
}
