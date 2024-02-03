use std::collections::HashMap;

const ALPHABET: [char; 26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

struct Scores<'a> {
    scores: HashMap<&'a str, u16>,
}

impl Scores {
    fn new_score(&mut self, name: &str, score: u16) -> Result<(),()> {
        if self.scores.contains_key(name) {
            Err(())
        } else {
            let _ = self.scores.insert(name,score);
            Ok(())
        }
    }
}