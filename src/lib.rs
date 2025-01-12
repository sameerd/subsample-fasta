use std::io;
use rand::prelude::*;

pub fn write_vecs<W: io::Write>(mut writer: W, vecs : &Vec<Vec<u8>> ) -> io::Result<()> {

  for vec in vecs.iter() {
    writer.write_all(vec)?;
  }
  Ok(())

}

// Implement Reservoir_sampling
// https://en.wikipedia.org/wiki/Reservoir_sampling#Optimal:_Algorithm_L

fn reservoir_sample (rng : &mut impl Rng) {


}
