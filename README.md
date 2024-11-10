# rust-genome-longread-count

- A kmer origin finding faster than the recent implementation of the recent implementation. Back to sequences: Find the origin of 𝑘-mers DOI: 10.21105/joss.07066.
- output a table for the direct ingestion into any graphs. 
- outputs a sam type file with the distinct count of the kmers and can be used for the jellyfish count. 
- support both the genome and the longread fasta file. 
- **genome fasta file should be a linear fasta and not a multi line fasta just like long-read**
- crate is available at [rust-genome-longread-count](https://crates.io/crates/kmerorigin)

```
Usage: kmerorigin <KMER_ARG> <FASTAFILE_ARG>

Arguments:
  <KMER_ARG>       please provide the kmer to be searched for the origin
  <FASTAFILE_ARG>  please provide the path to be searched for the strings containing the kmer

Options:
  -h, --help     Print help
  -V, --version  Print version
```
- a better table for direct ingestion into the graphs also to make a jellyfish count. 

```
./target/debug/kmerorigin 4 ./sample-files/fastafile.fasta
>seq1
AGTCAGTC        AGTC    0       4
AGTCAGTC        GTCA    1       5
AGTCAGTC        CAGT    3       7
AGTCAGTC        TCAG    2       6
>seq2
AGGCAGTC        CAGT    3       7
AGGCAGTC        GGCA    1       5
AGGCAGTC        AGGC    0       4
AGGCAGTC        GCAG    2       6
```

Gaurav Sablok
