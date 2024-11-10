mod args;
use args::KmeroriginArgs;
use clap::Parser;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::collections::HashSet;
#[allow(dead_code)]
/*
 *Author Gaurav Sablok
 *Universitat Potsdam
 *Date 2024-11-4

 * A kmer origin finding faster than the recent implementation of the recent implementation
 * Back to sequences: Find the origin of 𝑘-mers DOI: 10.21105/joss.07066.
 * To make it faster, it first uses the kmer iterations to make the kmer and then it uses the 
 * the uniques hashes to search across the given file. This outputs a table showing the sequence, 
 * kmer and the start and the end position. 
 * */

fn main() {

    let args = KmeroriginArgs::parse();
    kmer_fasta(args.fastafile_arg, args.kmer_arg);
}

fn kmer_fasta(path: String, kmer:usize) {
    let f = File::open(&path).expect("file not present");
    let read = BufReader::new(f);
    for i in read.lines() {
        let line = i
                   .expect("line not present");
        let mut header: Vec<&str> = vec![];
        let mut sequence:Vec<&str> = vec![];
        if line.starts_with(">") {
            header.push(&line)
        }
        if !line.starts_with(">") {
            sequence.push(&line)
        }  
        let mut sequence_iter:Vec<&str> = vec![];
        for i in 0..sequence.len() {
            let i = sequence[i]; 
            for j in 0..i.len() - &kmer {
                sequence_iter.push(&i[j..j+&kmer])
        }
       let hash_kmer: HashSet<_> = sequence_iter.iter().collect();
      let mut finalvec: Vec<&str> = Vec::new();
      for i in hash_kmer.into_iter() {
            finalvec.push(i)
        }
        let mut path = File::create("kmeruniquefasta.txt").expect("file not present");
        for i in finalvec.iter() {
            write!(path, "{}\n", i.to_string());
      }
       let mut mutunique = Vec::new(); 
       let uniquehold = File::open("kmeruniquefasta.txt").expect("file not present");
       let uniqueread = BufReader::new(uniquehold);
       for i in uniqueread.lines() {
         let appendline = i.expect("line not present");
         mutunique.push(appendline)
       }
       //println!("{:?}", mutunique);
       #[derive(Debug)]
       struct VecStore {
        id: String,
        kmer: String, 
        numberstart: usize,
        numberend : usize,
       }
       let mut indexstorestart = Vec::new();   
        for i in sequence.iter() {
            for j in mutunique.iter() {
                let indexout = i.find(j).unwrap();
                let indexoutend = i.find(j).unwrap()+j.len();
                indexstorestart.push(VecStore {id: i.to_string(), 
                      kmer: j.to_string(),
                      numberstart:indexout, 
                      numberend:indexoutend});
            }
        }
          for line in indexstorestart.iter() {
           println!("{}\t{}\t{}\t{}", line.id, line.kmer, line.numberstart, line.numberend)
        }
    }
    for j in header.iter() {
        println!("{}", j)
    }
    }
    }