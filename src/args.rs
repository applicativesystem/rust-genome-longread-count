use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(version)]

pub struct KmeroriginArgs {

    #[command(subcommand)]
    cmd:Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
   Genome {
       #[clap(short,long)]
       path : Option<String>,
       kmer: usize
   },
   Longread {
       #[clap(short,long)]
       path: Option<String>,
       kmer: usize
   },
   Illumina {
       #[clap(short,long)]
       path: Option<String>,
       kmer: usize
   },

}

/*

    /// please provide the kmer to be searched for the origin
    pub kmer_arg: u32,
    /// please provide the path to be searched for the strings containing the kmer
    pub fastqfile_arg: String,
    /// please provide the path to the fasta file to be searched for the string containing the kmer
    pub fastafile_arg: String,
    /// please provide the path to the longread file to be searched for containing the kmer
    pub longread: String,

*/
