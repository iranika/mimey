
use std::iter::Iterator;

pub trait MimeyInterface{
    fn add(&self, a: i32, b: i32) -> i32{
        a + b
    }
}

pub struct MimeyPiece{
    pub dialogue: String,
    pub comments: String,
    pub sound_note: String,
    pub charactor: String,
    pub sound_position: i8,
}

pub struct Mimey{
    pieces: Vec<MimeyPiece>
}

impl MimeyInterface for Mimey {}

impl Mimey {
    fn get_comments(block: Vec<&str>)->Vec<&str>{
        let comment = block.iter()
                    .copied()
                    .filter(|&x| {
                        x.chars().nth(0).unwrap() == '＃' || x.chars().nth(0).unwrap() == '#'
                    })
                    .collect::<Vec<_>>();
        return comment
    }

    fn get_attribute(block: Vec<&str>, prefix: Vec<char>)->Vec<&str>{
        let attrs = block.iter()
                    .copied()
                    .filter(|&x| {
                        prefix.iter().any(|&p|{
                            x.chars().nth(0).unwrap() == p
                        })
                    })
                    .collect::<Vec<&str>>();
        
        return attrs
    }


    pub fn parse(_text: &str)-> Vec<MimeyPiece>{
        let tmp = _text.replace("\r\n", "\n");
        let blocks: Vec<&str> = tmp.split("\n\n").collect();
        println!("blocks{:?}", blocks);
        let block = blocks.iter().map(|&x| -> MimeyPiece {
            let lines: Vec<&str> = x.split("\n").collect::<Vec<&str>>();
            let comments = Mimey::get_attribute(lines, vec!['#','＃']).join(","); //Mimey::get_comments(lines).join(",");

            return MimeyPiece{
                dialogue: comments,
                comments: "comment text".to_string(),
                sound_note: "sound note text".to_string(),
                charactor: "charactor text".to_string(),
                sound_position: 1
            }
        }).collect::<Vec<_>>();
        println!("{:?}", block[3].dialogue);
        

        let dummy = MimeyPiece{
            dialogue: "dialogue text".to_string(),
            comments: "comment text".to_string(),
            sound_note: "sound note text".to_string(),
            charactor: "charactor text".to_string(),
            sound_position: 1
        };
        let result: Vec<MimeyPiece> = vec![dummy];

        return result;
    }
}
