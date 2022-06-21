use std::iter::Iterator;

#[derive(Debug)]
pub struct MimeyPiece {
    pub dialogue: String,
    pub comments: String,
    pub sound_note: String,
    pub charactor: String,
    pub sound_position: String,
}
#[repr(transparent)]
pub struct Mimey {
    pub pieces: Vec<MimeyPiece>,
}

pub struct MeDoRaw {
    pub header: String,
    pub body: String
}

impl Mimey {
    fn get_attribute(block: Vec<&str>, prefix: Vec<char>) -> Vec<String> {
        let attrs: Vec<String> = block
            .into_iter()
            .filter(|x| prefix.iter().any(|&p| x.chars().nth(0).unwrap() == p))
            .map(|v| -> String {
                let mut text = v.to_string().clone();
                text.remove(0);
                text
            })
            .collect();
        return attrs;
    }
    fn get_dialogue(block: Vec<&str>, ignorePrefix: Vec<char>) -> Vec<String> {
        let dialogue = block
            .into_iter()
            .filter(|x| ignorePrefix.iter().all(|&p| x.chars().nth(0).unwrap() != p))
            .map(|v| v.to_string())
            .collect();
        return dialogue;
    }
}

pub extern "win64" fn parseMimey(_text: &str) -> Mimey {
    let tmp = _text.replace("\r\n", "\n");
    let blocks: Vec<&str> = tmp.split("\n\n").collect();
    //println!("blocks{:?}", blocks);
    let block = blocks
        .iter()
        .map(|&x| -> MimeyPiece {
            let lines: Vec<&str> = x.split("\n").collect::<Vec<&str>>();
            let dialogue = Mimey::get_dialogue(
                lines.clone(),
                vec!['#', '＃', '$', '＄', '@', '＠', '!', '！'],
            )
            .join(","); //Mimey::get_comments(lines).join(",");
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
        })
        .collect::<Vec<_>>();
    println!("{:?}", block);

    let result: Mimey = Mimey { pieces: block };

    return result;
}
