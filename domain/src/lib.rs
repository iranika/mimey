use std::iter::Iterator;
use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple};

#[derive(Debug)]
pub struct MimeyPiece {
    pub dialogue: String,
    pub comments: String,
    pub sound_note: String,
    pub charactor: String,
    pub sound_position: String,
}


#[derive(Debug)]
pub struct Mimey {
    pub pieces: Vec<MimeyPiece>,
}

#[derive(Debug)]
pub struct MedoHeader {
    pub raw: String
}

pub struct Medo {
    pub header: MedoHeader,
    pub body: Mimey
}

#[derive(Debug)]
pub struct MedoRaw {
    pub header: String,
    pub body: String
}

impl MedoRaw {
    pub fn getHeader(&self) -> Option<MedoHeader>{
        return Some(MedoHeader{
            raw : "".to_string()
        })
    }
    pub fn getBody(&self) -> Option<Mimey>{
        let result = parseMimey(self.body.as_str());
        //println!("{:?}", result);
        return Some(result)
    }
}

pub fn parseMeS(text: &str) -> Medo {
    let dummy = MimeyPiece {
        dialogue: "あ、キタキタ。女の子二人を待たせるなんて、失礼だぞ。".to_string(),
        comments: "comment text".to_string(),
        sound_note: "駅前の音".to_string(),
        charactor: "ニカ".to_string(),
        sound_position: "1".to_string()
    };
    //HeaderとBodyに分離
    let raw = parseMedoRaw(text);

    //Headerのパース
    //Bodyのパース
    //HeaderとBodyをMedoに結合

    return Medo {
        header: raw.getHeader().unwrap(),
        body: raw.getBody().unwrap()
    }
}

pub fn parseMedoRaw(text: &str) -> MedoRaw {
    let tmp = text.replace("\r\n", "\n");
    let blocks: Vec<&str> = tmp.split("----\n").collect();
    return MedoRaw {
        header: blocks[0].to_string(),
        body: blocks[1].to_string()
    }
}

impl Mimey {
    fn get_attribute(block: Vec<&str>, prefix: Vec<char>) -> Vec<String> {
        let attrs: Vec<String> = block
            .into_iter()
            .filter(|x| prefix.iter().any(|&p| {
                match x.chars().nth(0){
                    Some(v) => v == p,
                    None => false
                }
            }))
            .map(|v| -> String {
                let mut text = v.to_string().clone();
                text.remove(0);
                text
            })
            .collect();
        //println!("{:?}", attrs);
        return attrs;
    }
    fn get_dialogue(block: Vec<&str>, ignorePrefix: Vec<char>) -> Vec<String> {
        let dialogue = block
            .into_iter()
            .filter(|x| ignorePrefix.iter().all(|&p| {
                match x.chars().nth(0){
                    Some(v) => v != p,
                    None => false
                }
            }))
            .map(|v| v.to_string())
            .collect();
        //println!("{:?}", dialogue);
        return dialogue;
    }
}



pub fn parseMimey(_text: &str) -> Mimey {
    let tmp = _text.replace("\r\n", "\n");
    let blocks: Vec<&str> = tmp.split("\n\n").collect();
    //println!("blocks{:?}", blocks);
    let block = blocks
        .into_iter()
        .map(|x| -> MimeyPiece {
            let lines: Vec<&str> = x.split("\n").collect::<Vec<&str>>();
            let dialogue = Mimey::get_dialogue(
                lines.clone(),
                vec!['#', '＃', '$', '＄', '@', '＠', '!', '！']
            ).join("\n"); //Mimey::get_comments(lines).join(",");
            let comments = Mimey::get_attribute(lines.clone(), vec!['#', '＃']).join(","); //Mimey::get_comments(lines).join(",");
            let sound_note = Mimey::get_attribute(lines.clone(), vec!['$', '＄']).join(",");
            let charactor = Mimey::get_attribute(lines.clone(), vec!['@', '＠']).join(",");
            let sound_position = Mimey::get_attribute(lines.clone(), vec!['!', '！']).join(",");
            
            return MimeyPiece {
                dialogue: dialogue,
                comments: comments,
                sound_note: sound_note,
                charactor: charactor,
                sound_position: sound_position,
            };
            //println!("{:?}",&result);
        })
        .collect();
    //println!("{:?}", block);

    let result: Mimey = Mimey { pieces: block };

    return result;
}
