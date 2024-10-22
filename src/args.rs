use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]

pub struct kmeroriginArgs {
    /// please provide the kmer to be searched for the origin
    pub kmer_arg: u32,
    /// please provide the path to be searched for the strings containing the kmer
    pub fastqfile_arg: String,
    /// please provide the path to the fasta file to be searched for the string containing the kmer
    pub fastafile_arg: String,
    /// please provide the path to the longread file to be searched for containing the kmer
    pub longread: String
}
