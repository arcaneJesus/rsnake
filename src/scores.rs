use std::collections::HashMap;

const ALPHABET: [char; 26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

struct Scores {
    scores: HashMap<&'static str, u16>
}