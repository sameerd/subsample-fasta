use std;
use std::io;

use rand::prelude::*;
use rand_pcg::Pcg64;

use seq_io::fasta::{Reader};

use subsample_fasta::*;


fn main() {
  
  let _seed: u64 = 25;
  let k: usize = 2;// number of samples to return

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
