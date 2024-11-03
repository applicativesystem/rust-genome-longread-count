# rust-nuc-iter

- A kmer origin finding faster than the recent implementation of the recent implementation. Back to sequences: Find the origin of 𝑘-mers DOI: 10.21105/joss.07066.
- I implemented the rust async programming to index the kmer first over a window size and then use that to make the set of the kmers, so that you have less search space and using that to search the kmer in the file provided
- It only searchers for the unique hashes and their location. to make it even faster.It support genome and short and long illumina reads.


```
Usage: kmerorigin <COMMAND>

Commands:
  genome
  longread
  illumina
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
❯ ./target/debug/kmerorigin genome -h
Usage: kmerorigin genome [OPTIONS] <KMER>

Arguments:
  <KMER>

Options:
  -p, --path <PATH>
  -h, --help         Print help
❯ ./target/debug/kmerorigin illumina -h
Usage: kmerorigin illumina [OPTIONS] <KMER>

Arguments:
  <KMER>

Options:
  -p, --path <PATH>
  -h, --help         Print help
❯ ./target/debug/kmerorigin longread -h
Usage: kmerorigin longread [OPTIONS] <KMER>

Arguments:
  <KMER>

Options:
  -p, --path <PATH>
  -h, --help         Print help
```

Gaurav Sablok
