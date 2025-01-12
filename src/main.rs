use std;
use std::io;
use std::io::{Write};

use rand::prelude::*;
use rand_pcg::Pcg64;

use seq_io::fasta::{Record,Reader};

use subsample_fasta::*;


fn main() {
  
  let mut rng = Pcg64::seed_from_u64(10); 
  let mut reader = Reader::new(io::stdin());
  
  let mut large_vecs : Vec<Vec<u8>> = Vec::new();

  subsample_fasta::reservoir_sample(&mut rng, &mut large_vecs, &mut reader);

  // now print out all the large vecs to stdout
  let mut stdout = io::stdout();
  write_vecs(&stdout, &large_vecs);
}
