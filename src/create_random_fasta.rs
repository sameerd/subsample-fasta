
use rand::prelude::*;

// use rand::distributions::Alphanumeric;
use rand_pcg::Pcg64;

// use seq_io::fasta::{Reader,Record};

fn main() {

  let mut rng = Pcg64::seed_from_u64(10); 

  let n2: u16 = rng.gen();

  println!("Random u16: {}", n2);


}
