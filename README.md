# rust-bedtools-pangenome-single-merge
 
 - rust bedtools single merge pangenome, merge across a single bed file. 
 - it uses a two tuple based approach which avoid quadratic cost. 
 - Incase of Golang and RUST, please see the last commit message and if it says compiled binary then it is completed or else still in development version.
 
 ```
 cargo build
 
 ```
 ```
 Usage: rust-bedtools-pangenome-single-merge <BED1> <FASTA>

 Arguments:
  <BED1>   please provide the path to the first alignment file
  <FASTA>  please provide the path to the reference fasta file

 Options:
  -h, --help     Print help
  -V, --version  Print version

 ```
 - to run the binary
 
 ```
   ./rust-bedtools-pangenome-single-merge ./sample-files/exon-sample.bed ./sample-files/sample.fasta 
 
 ```
 - bedtools, it doesnt report the merged sequences, rust-bedtools report the merged sequences. 

 ```
  ➜  rust-bedtools-pangenome-single-merge git:(main) ✗ cat merged_cordinates.txt 
 chr1	10	30	CTGACCATGACTGACTGACT
 chr1	50	60	CTGACATGAC
 chr1	70	80	GACTGACTGA
 chr1	70	90	GACTGACTGACTGACTGACT
 chr1	90	98	GACATGAC
 chr1	92	100	CATGACTG

 ```

 Gaurav Sablok
