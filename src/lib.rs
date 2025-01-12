use std::io;
use rand::prelude::*;

use seq_io::fasta::{Record,Reader};
use seq_io::policy::{BufPolicy};

pub fn write_vecs<W: io::Write>(mut writer: W, vecs : &Vec<Vec<u8>> ) -> io::Result<()> {
  for vec in vecs.iter() {
    writer.write_all(vec)?;
  }
  Ok(())
}

// Implement Reservoir_sampling
// https://en.wikipedia.org/wiki/Reservoir_sampling#Optimal:_Algorithm_L
pub fn reservoir_sample<R : io::Read, P> (_rng : &mut impl Rng, 
                     samples : &mut Vec<Vec<u8>>,
                     reader : &mut Reader<R,P>) where P : BufPolicy  {

  let k = samples.len();
  let mut sample_idx = 0;
  while let Some(result) = reader.next() {
      let record = result.unwrap();
      // determine sequence length
      let seqlen = record.seq_lines()
                         .fold(0, |l, seq| l + seq.len());
      if seqlen > 400 {
          if sample_idx < k {
            let _write_result = record.write_unchanged(
                                  &mut samples[sample_idx]);
            sample_idx = sample_idx + 1;
          }
      } else {
          eprintln!("{} is only {} long", record.id().unwrap(), seqlen);
      }
  }

}
