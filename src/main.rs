use std;
use std::io;
use std::io::{Write};
use seq_io::fasta::{Reader,Record};


fn main() {
  
  let mut reader = Reader::new(io::stdin());
  
  let mut large_vecs : Vec<Vec<u8>> = Vec::new();
  let mut large_vec_len = large_vecs.len(); // 0 but with the right type

  while let Some(result) = reader.next() {
      let record = result.unwrap();
      // determine sequence length
      let seqlen = record.seq_lines()
                         .fold(0, |l, seq| l + seq.len());
      if seqlen > 400 {
	  // Make a new vec<u8> and write the sequence to it without 
          // making any changes to the sequence 
          large_vecs.push(Vec::new());
          large_vec_len = large_vec_len + 1;
          let _write_result = record.write_unchanged(
				&mut large_vecs[large_vec_len-1]);
      } else {
          eprintln!("{} is only {} long", record.id().unwrap(), seqlen);
      }
  }
  // now print out all the large vecs to stdout
  let mut stdout = io::stdout();
  for vec in large_vecs.iter() {
    let _ = stdout.write_all(vec);
  }
}
