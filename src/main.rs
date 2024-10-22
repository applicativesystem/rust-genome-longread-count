mod args;
use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::io::Read;
use args::KmeroriginArgs;
use clap::Parser;
use std::io::BufReader;
use std::collections::HashSet;

/*
 *Author Gaurav Sablok
 *Universitat Potsdam
 *Date 2022-10-22

 * A kmer origin finding faster than the recent implementation of the recent implementation
 * Back to sequences: Find the origin of 𝑘-mers DOI: 10.21105/joss.07066.
 *
 * I implemented the rust async programming to index the kmer first over a window size and then
 * use that to make the set of the kmers, so that you have less search space and using that to
 * search the kmer in the file provided
 *
 * it only searchers for the unique hashes and their location. to make it even faster, i am also
 * implementing a async programming later today.
 *
 * it support genome and short and long illumina reads.
 * */

fn main() {

    let args:KmeroriginArgs = KmeroriginArgs::parse();
    genome_file(args.fastafile_arg,  args.kmers_arg);
    illumina_file(args.fastqfile_arg, args.kmer_arg);
    longread_file(args.longreadfile, args.kmer_arg)
}

fn genome_file(path: &str, kmer: usize) -> Result<(),&'static Vec<&str>> {
    let file_open = File::open(&path);
    let header: Vec<&str> = vec![];
    let sequence:Vec<&str> = vec![];
    let file_read = BufReader::new(file_open);
    for line in file_read.lines(){
    let expect_line = line
        .expect("line not present");
    if expect_line.starts_with(">") {
        header.push(&expect_line)
    }
    if ! expect_line.starts_with(">") {
        sequence.push(&expect_line)
    }
    }

    let sequence_iter:Vec<&str> = vec![];

    for i in 0..sequence.len() {
        let intermediate: &str = sequence_iter[i];
        for j in 0..intermediate.len() - &kmer {
            sequence_iter.push(&intermediate[j..j+kmer])
    }
    }

    let hash_kmer: HashSet<_> = sequence_iter.iter().collect();
    let mut filew = File::create("kmerunique.txt");
    for i in &sequence_iter {
    filew.write_all(i.bytes())
    }

    let hash_check:Vec<&str> = vec![];
    let has_start:Vec<usize> = vec![];
    let hash_end:Vec<usize> = vec![];

    let file = File::open("kmeruniqe.txt");
    let hash_read = BufReader::new(file);
    for i in hash_read.lines(){
        let line = i.expect("empty line");
        hash_check.push(i)
    }
    for i in 0..sequence.len() {
        for j in 0..hash_check.len() {
        let has_store:Vec<_> = sequence[i].match_indices(hash_check[j]).collect();
        let f = File::create("searchedhashes.txt");
        for i in 0..has_Store.len(){
            f.write_all(has_store[i].0.as_bytes(), has_store[i].1.as_bytes(), has_store[i].0+hash_check[j].len().as_bytes())
        }
      }
   }
    Ok(())
} 


fn longread_file(path: &str, kmer: usize) -> Result<(),&'static Vec<&str>> {
    let file_open = File::open(&path);
    let header: Vec<&str> = vec![];
    let sequence:Vec<&str> = vec![];
    let file_read = BufReader::new(file_open);
    for line in file_read.lines(){
    let expect_line = line
        .expect("line not present");
    if expect_line.starts_with("@") {
        let line:&str = expect_line.
            split(" ")
            .replace("@", "")
            .collect();
        header.push(&line[0])
    }
    if ! expect_line.starts_with("@") {
        sequence.push(&expect_line)
    }
    }

    let sequence_iter:Vec<&str> = vec![];

    for i in 0..sequence.len() {
        let intermediate: &str = sequence_iter[i];
        for j in 0..intermediate.len() - &kmer {
            sequence_iter.push(&intermediate[j..j+kmer])
    }
    }

    let hash_kmer: HashSet<_> = sequence_iter.iter().collect();
    let mut filew = File::create("kmerunique.txt");
    for i in &sequence_iter {
    filew.write_all(i.bytes())
    }

    let hash_check:Vec<&str> = vec![];
    let has_start:Vec<usize> = vec![];
    let hash_end:Vec<usize> = vec![];

    let file = File::open("kmeruniqe.txt");
    let hash_read = BufReader::new(file);
    for i in hash_read.lines(){
        let line = i.expect("empty line");
        hash_check.push(i)
    }

    for i in 0..sequence.len() {
        for j in 0..hash_check.len() {
        let has_store:Vec<_> = sequence[i].match_indices(hash_check[j]).collect();
        let f = File::create("searchedhashes.txt");
        for i in 0..has_Store.len(){
            f.write_all(has_store[i].0.as_bytes, has_store[i].1.as_bytes(), has_store[i].0+hash_check[j].len().as_bytes())
        }
      }
   }
    Ok(())
}


fn illumina_file(path: &str, kmer: usize) -> Result<(),&'static Vec<&str>> {
    let file_open = File::open(&path);
    let header: Vec<&str> = vec![];
    let sequence:Vec<&str> = vec![];
    let file_read = BufReader::new(file_open);
    for line in file_read.lines(){
    let expect_line = line
        .expect("line not present");
    if expect_line.starts_with("@") {
        let line:&str = expect_line.
            split(" ")
            .replace("@", "")
            .collect();
        header.push(&line[0])
    }
    if  expect_line.starts_with("A") || expect_line.starts_with("T") || expect_line.starts_with("G") || expect_line.starts_with("C") {
        sequence.push(&expect_line)
    }
    }

    let sequence_iter:Vec<&str> = vec![];

    for i in 0..sequence.len() {
        let intermediate: &str = sequence_iter[i];
        for j in 0..intermediate.len() - &kmer {
            sequence_iter.push(&intermediate[j..j+kmer])
    }
    }

    let hash_kmer: HashSet<_> = sequence_iter.iter().collect();
    let mut filew = File::create("kmerunique.txt");
    for i in &sequence_iter {
    filew.write_all(i.bytes())
    }

    let hash_check:Vec<&str> = vec![];
    let has_start:Vec<usize> = vec![];
    let hash_end:Vec<usize> = vec![];

    let file = File::open("kmeruniqe.txt");
    let hash_read = BufReader::new(file);
    for i in hash_read.lines(){
        let line = i.expect("empty line");
        hash_check.push(i)
    }

        for i in 0..sequence.len() {
        for j in 0..hash_check.len() {
        let has_store:Vec<_> = sequence[i].match_indices(hash_check[j]).collect();
        let f = File::create("searchedhashes.txt");
        for i in 0..has_store.len(){
            f.write_all(has_store[i].0.as_bytes(), has_store[i].1.as_bytes(), has_store[i].0+hash_check[j].len().as_bytes())
        }
      }
   }
    Ok(())
}