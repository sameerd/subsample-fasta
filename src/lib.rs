use std::io;
use rand::prelude::*;

use seq_io::fasta::{Reader};
use seq_io::policy::{BufPolicy};

const DEBUG : bool = true;

pub fn write_vecs<W: io::Write>(mut writer: W, vecs : &Vec<Vec<u8>> ) 
                        -> io::Result<()> {
  for vec in vecs.iter() {
    writer.write_all(vec)?;
  }
  Ok(())
}

// Implement Reservoir_sampling
// https://en.wikipedia.org/wiki/Reservoir_sampling#Optimal:_Algorithm_L
pub fn reservoir_sample<R : io::Read, P> (rng : &mut impl Rng, 
              k : usize, // number of samples to uniformly sample
              samples : &mut Vec<Vec<u8>>, // where the samples are stored
              reader : &mut Reader<R,P> // seq_io reader RefRecord iterator
              ) where P : BufPolicy  {

  samples.clear();
  samples.resize(k, Vec::new());

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
        w = w * ((rng.gen::<f64>().ln()) / (k as f64)).exp();
        skip_until = i + ((rng.gen::<f64>().ln() / (1.0 - w).ln()).floor() 
                                 as usize);
        if DEBUG {println!("i={}, skip_until={} w={}", i, skip_until, 
                                w);}
      } 
    }
  }
}
