#!/bin/bash


(for i in {1..10000}; 
  do  
    cat tests/data/small_10.fasta | ./target/debug/subsample-fasta -n 3 | grep "^>"; 
  done 
) | sort | uniq -c
