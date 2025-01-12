/*
 * holding all the structs in the separate files so that they
 * can be easily called as a reference call in the result.
 *
 *
 * */
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MergeBed {
    pub bed1: String,
    pub start1: usize,
    pub end1: usize,
    pub bedseq: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Fasta {
    pub header: String,
    pub sequence: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MergeFasta {
    pub bed: String,
    pub start1: usize,
    pub end1: usize,
    pub mergedseq: String,
}
