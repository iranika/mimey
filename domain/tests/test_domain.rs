extern crate domain;

use domain::*;
use std::fs;

#[cfg(test)]
mod domain_tests {
    #[test]
    fn test_parse() {
        let dummy = domain::MimeyPiece {
            dialogue: "あ、キタキタ。女の子二人を待たせるなんて、失礼だぞ。".to_string(),
            comments: "comment text".to_string(),
            sound_note: "駅前の音".to_string(),
            charactor: "ニカ".to_string(),
            sound_position: "1".to_string()
        };
        let text = std::fs::read_to_string("tests/SampleMimeyScript.txt").unwrap();
        //println!("{}", text);
        let result = domain::parseMimey(&text);
        println!("{:?}", result.pieces);
        
        assert_eq!(result.pieces[0].charactor, dummy.charactor);
    }
}