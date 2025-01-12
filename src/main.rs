use std;
use std::io;

use rand::prelude::*;
use rand_pcg::Pcg64;

use seq_io::fasta::{Reader};

use subsample_fasta::*;


fn main() {
  
  let seed: u64 = 10;
  let k: i32 = 4; // number of samples to return
  let mut rng = Pcg64::seed_from_u64(seed); 

  let mut reader = Reader::new(io::stdin());
  
  let mut large_vecs : Vec<Vec<u8>> = Vec::new();
  
  
  for _ in 0..k {
    large_vecs.push(Vec::new());
  }

  reservoir_sample(&mut rng, &mut large_vecs, &mut reader);

  // now print out all the large vecs to stdout
  let stdout = io::stdout();
  let _ = write_vecs(&stdout, &large_vecs);
}
