use std::io;
use rand::prelude::*;

use seq_io::fasta::{Record,Reader,RecordsIter};
use seq_io::policy::{BufPolicy};

pub fn write_vecs<W: io::Write>(mut writer: W, vecs : &Vec<Vec<u8>> ) -> io::Result<()> {
  for vec in vecs.iter() {
    writer.write_all(vec)?;
  }
  Ok(())
}

// Implement Reservoir_sampling
// https://en.wikipedia.org/wiki/Reservoir_sampling#Optimal:_Algorithm_L
pub fn reservoir_sample<R : io::Read, P> (rng : &mut impl Rng, 
                     samples : &mut Vec<Vec<u8>>,
                     reader : &mut seq_io::fasta::Reader<R,P>) where P : BufPolicy  {

  let mut k = samples.len();
  while let Some(result) = reader.next() {
      let record = result.unwrap();
      // determine sequence length
      let seqlen = record.seq_lines()
                         .fold(0, |l, seq| l + seq.len());
      if seqlen > 400 {
          // Make a new vec<u8> and write the sequence to it without 
          // making any changes to the sequence 
          samples.push(Vec::new());
          k = k + 1;
          let _write_result = record.write_unchanged(
                                &mut samples[k-1]);
      } else {
          eprintln!("{} is only {} long", record.id().unwrap(), seqlen);
      }
  }

}
