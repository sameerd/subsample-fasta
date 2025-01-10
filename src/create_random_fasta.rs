use std::iter;
use std::io;

use rand::prelude::*;
use rand_pcg::Pcg64;

use seq_io::fasta::{Record, OwnedRecord};


fn generate_random_aa(rng : &mut impl Rng, len: usize) -> Vec<u8> {
  // Copied from https://stackoverflow.com/a/74953997
  // Changed to return Vec<u8>
  const STANDARD_AA: &[u8] = b"ACDEFGHIKLMNPQRSTVWY";
  let one_u8 = || STANDARD_AA[rng.gen_range(0..STANDARD_AA.len())] ;
  iter::repeat_with(one_u8).take(len).collect()
}

fn main() {

  let stdout = io::stdout();
  let mut rng = Pcg64::seed_from_u64(10); 

  let n: u16 = 10; // number of random sequences

  for i in 0..n {
    let rec_head = [b"Random_", i.to_string().as_bytes()].concat();
    let seq_len : usize = rng.gen_range(20..50);
    let rec_seq = generate_random_aa(&mut rng, seq_len);
    let rec = OwnedRecord{head: rec_head, seq: rec_seq};
    _ = rec.write(&stdout);
  }


}
