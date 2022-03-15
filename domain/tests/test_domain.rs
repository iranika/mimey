extern crate domain;

use domain::*;
use std::fs;

#[cfg(test)]
mod domain_tests {
    #[test]
    fn test_parse() {
        let dummy = domain::MimeyPiece {
            dialogue: "dialogue text".to_string(),
            comments: "comment text".to_string(),
            sound_note: "sound note text".to_string(),
            charactor: "charactor text".to_string(),
            sound_position: 1
        };
        let text = std::fs::read_to_string("tests/SampleMimeyScript.txt").unwrap();
        //println!("{}", text);
        let result = domain::Mimey::parse(&text);
        assert_eq!(result[0].dialogue, dummy.dialogue);
    }
}