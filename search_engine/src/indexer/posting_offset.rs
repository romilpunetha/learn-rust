pub struct PostingOffset<'a> {
    posting_word: &'a str,
    posting_offset: &'a str,
}

impl<'a> PostingOffset<'a> {
    pub fn new(posting_word: &'a str, posting_offset: &'a str) -> Self {
        PostingOffset { posting_word, posting_offset }
    }
}