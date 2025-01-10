

fn main() {
  use seq_io::fasta::{Reader,Record};
  use std::io;
  
  let mut reader = Reader::new(io::stdin());
  let mut stdout = io::stdout();
  
  while let Some(result) = reader.next() {
      let record = result.unwrap();
      // determine sequence length
      let seqlen = record.seq_lines()
                         .fold(0, |l, seq| l + seq.len());
      if seqlen > 400 {
          record.write_wrap(&mut stdout, 80).unwrap();
      } else {
          eprintln!("{} is only {} long", record.id().unwrap(), seqlen);
      }
  }
}
