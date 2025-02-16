use std::iter;
use std::io;
use rand::prelude::*;

use seq_io::fasta::{Reader};
use seq_io::policy::{BufPolicy};

use seq_io::fastq::Reader as Reader_fastq;

//const DEBUG : bool = true;
const DEBUG : bool = false;

const STANDARD_AA: &[u8] = b"ACDEFGHIKLMNPQRSTVWY";
const STANDARD_NT: &[u8] = b"ACGT";

pub fn generate_random(pool : &[u8], rng : &mut impl Rng, len: usize) -> Vec<u8> {
  // Copied from https://stackoverflow.com/a/74953997
  // Changed to return Vec<u8>
  let one_u8 = || pool[rng.gen_range(0..pool.len())] ;
  iter::repeat_with(one_u8).take(len).collect()
}

pub fn generate_random_aa(rng : &mut impl Rng, len: usize) -> Vec<u8> {
  generate_random(STANDARD_AA, rng, len)
}

pub fn generate_random_nt(rng : &mut impl Rng, len: usize) -> Vec<u8> {
  generate_random(STANDARD_NT, rng, len)
}

pub fn generate_random_qual(rng : &mut impl Rng, len: usize) -> Vec<u8> {
  const QUAL_START : u8 = 0x21; // ! char (lowest quality)
  const QUAL_END : u8 = 0x7e; // ~ char (highest quality)
  let one_u8 = || rng.gen_range(QUAL_START..=QUAL_END) ;
  iter::repeat_with(one_u8).take(len).collect()
}





pub fn write_vecs<W: io::Write>(mut writer: W, vecs : &Vec<Vec<u8>>,
                        indices: &Vec<usize> ) 
                        -> io::Result<()> {
  let mut indices_argsort = (0..indices.len()).collect::<Vec<_>>();
  indices_argsort.sort_by_key(|&i| &indices[i]);
  for idx in indices_argsort.iter() {
    let _ = writer.write_all(&vecs[*idx]);
  }
  Ok(())
}

// Implement Reservoir_sampling
// https://en.wikipedia.org/wiki/Reservoir_sampling#Optimal:_Algorithm_L
pub fn reservoir_sample<R : io::Read, P> (rng : &mut impl Rng, 
              k : usize, // number of samples to uniformly sample
              samples : &mut Vec<Vec<u8>>, // where the samples are stored
              indices : &mut Vec<usize>, // index of each sample in Reader (0-based)
              reader : &mut Reader<R,P> // seq_io reader RefRecord iterator
              ) where P : BufPolicy  {

  samples.clear();
  samples.resize(k, Vec::new());
  
  indices.clear();
  for i in 0..k {
    indices.push(i);
  }

  let mut ctr : usize = 0;
  let mut w : f64;
  w = ((rng.gen::<f64>().ln()) / (k as f64)).exp();
  let mut skip_until : usize;
  skip_until = k + ((rng.gen::<f64>().ln() / (1.0 - w).ln()).floor() 
                                 as usize);
  while let Some(result) = reader.next() {
    ctr = ctr + 1;
    let i = ctr - 1; // actual index and starts with zero
    let record = result.unwrap();
    if i < k {
      if DEBUG {println!("i={}, k={} initial fill", i, k);}
      let _ = record.write_unchanged(&mut samples[i]);
    } else { // i >= k
      if i < skip_until {
        if DEBUG {println!("i={}, skip_until={} waiting", i, skip_until);}
      } else {
        let idx_to_replace = rng.gen_range(0..k);
        if DEBUG {println!("i={}, skip_until={} replacing {}", i, skip_until, 
                                idx_to_replace);}
        samples[idx_to_replace].clear();
        let _ = record.write_unchanged(&mut samples[idx_to_replace]);
        indices[idx_to_replace] = i;
        w = w * ((rng.gen::<f64>().ln()) / (k as f64)).exp();
        skip_until = i + ((rng.gen::<f64>().ln() / (1.0 - w).ln()).floor() 
                                 as usize);
        if DEBUG {println!("i={}, skip_until={} w={}", i, skip_until, 
                                w);}
      } 
    }
  }
}


