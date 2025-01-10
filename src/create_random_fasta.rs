use std::iter;

use rand::prelude::*;

use rand_pcg::Pcg64;

// use seq_io::fasta::{Reader,Record};

fn generate_random_aa(rng : &mut impl Rng, len: usize) -> String {
  // Copied from https://stackoverflow.com/a/74953997
  const STANDARD_AA: &[u8] = b"ACDEFGHIKLMNPQRSTVWY";
  //let mut rng = rand::thread_rng();
  let one_char = || STANDARD_AA[rng.gen_range(0..STANDARD_AA.len())] as char;
  iter::repeat_with(one_char).take(len).collect()
}

fn main() {

  let mut rng = Pcg64::seed_from_u64(10); 

  let n: u16 = 10; // number of random sequences

  for i in 0..n {
    let seq_id = format!( "Random_{}", i);
    let seq_len : usize = rng.gen_range(20..50);
    let random_seq = generate_random_aa(&mut rng, seq_len);
    println!(">{}\n{}", seq_id, random_seq);
  }


}
