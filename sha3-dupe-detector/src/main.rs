use sha3_dupe_detector::{analyze_duplicates, generate_random_phrases};


fn main() {
    let phrases = generate_random_phrases();
    analyze_duplicates(&phrases);
}
