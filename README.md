# rust-genome-longread-count

- A kmer origin finding faster than the recent implementation of the recent implementation. Back to sequences: Find the origin of 𝑘-mers DOI: 10.21105/joss.07066.
- output a table for the direct ingestion into any graphs. 
- outputs a sam type file with the distinct count of the kmers and can be used for the jellyfish count. 
- support both the genome and the longread fasta file. 

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
AGTCAGTC	TCAG	2	6
AGTCAGTC	AGTC	0	4
AGTCAGTC	GTCA	1	5
AGTCAGTC	CAGT	3	7
AGTCAGTT	AGTC	0	4
AGTCAGTT	GTCA	1	5
AGTCAGTT	TCAG	2	6
AGTCAGTT	CAGT	3	7
>seq2
AGGCAGTC	AGGC	0	4
AGGCAGTC	GGCA	1	5
AGGCAGTC	GCAG	2	6
AGGCAGTC	CAGT	3	7
ATATATGA	TATG	3	7
ATATATGA	ATAT	0	4
ATATATGA	TATA	1	5

```

Gaurav Sablok
