# Sequence Alignment With A* Example

This is an example of using A* path finding to accelerate a dynamic programming algorithm, in this case the [sequence alignment problem](https://en.wikipedia.org/wiki/Sequence_alignment), which [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) is a specific instance of.

Unlike the standard Levenshtein distance algorithm, this runs in something like `O(n * e^2)` time where `n` is the input length and `e` is the edit distance. It does this by using a heuristic like A* to explore only promising states along the diagonal of the grid and not the whole `O(n^2)` grid.

It is substantially faster for large files with few edits than the naive `O(n^2)` dynamic programming algorithm it is based on, but is still much slower than specialized and highly optimized global sequence alignment programs like [Edlib](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5408825/). The difference is I wrote this in two hours and it's 150 lines of code including tests, debugging routines and examples.

It is written in Rust and contains two example programs:

- `seqalign`: Reads to FASTA format genetic sequence files and prints the alignment distance.
- `seqalign_plain`: Reads two plain text files and prints the alignment distance.
