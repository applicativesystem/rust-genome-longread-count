mod args;
use std::env::Args;
use cmd_lib::run_cmd;
use std::fs::File;
use std::io::Read;
use std::fmt::Display;
use args::kmeroriginArgs;
use clap::Parser;
use std::io::{BufReader, BufRead};

/*
 *Author Gaurav Sablok
 *Universitat Potsdam
 *Date 2022-10-22
*/


fn fastafile(path: &str, kmer: u32) -> Result<Vec<&str>,E> {
    let file_open = File::open(&path)?;
    Ok(());
    let header: Vec<&str> = vec![];
    let sequence:Vec<&str> = vec![];
    let file_read = BufReader::new(file_open);
