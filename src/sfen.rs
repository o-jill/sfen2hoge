use regex::Regex;

// struct Tegoma {
//     koma: Vec<i8>,
// }

pub struct Sfen {
    ban: String,
    teban: String,
    tegoma: String,
    nteme: i32,
}

#[derive(PartialEq, Debug)]
enum Promote {
    None,
    Promoted,
}

fn p2fu(piece: char, promote: Promote) -> String {
    let idx = "plnsgbrk".find(piece).unwrap_or(8);
    if promote == Promote::Promoted {
        return "と杏圭全金馬龍玉？".chars().nth(idx).unwrap().to_string();
    }
    "歩香桂銀金角飛玉？".chars().nth(idx).unwrap().to_string()
}

fn extractdan(txt: &str) -> Result<String, String> {
    let mut res = String::from("|");
    let masu = txt.chars();
    let mut promote = Promote::None;
    let resente = Regex::new("[PLNSGBRK]").unwrap();
    let regote = Regex::new("[plnsgbrk]").unwrap();
    for ch in masu {
        match ch {
            '1'..='9' => {
                res = res
                    + &std::iter::repeat(" ・")
                        .take(ch.to_digit(10).unwrap() as usize)
                        .collect::<String>()
            }
            '+' => promote = Promote::Promoted,
            ch if resente.is_match(&ch.to_string()) => {
                res = res + " " + &p2fu(ch.to_ascii_lowercase(), promote);
                promote = Promote::None;
            }
            ch if regote.is_match(&ch.to_string()) => {
                res = res + "v" + &p2fu(ch, promote);
                promote = Promote::None;
            }
            _ => return Err(format!("{} is not allowed to use!!", ch)),
        }
    }
    Ok(res + "|")
}

fn kanjinum(num: usize) -> Result<String, String> {
    let kanji = [
        "", "一", "二", "三", "四", "五", "六", "七", "八", "九", "十", "十一", "十二", "十三",
        "十四", "十五", "十六", "十七", "十八",
    ];
    if num > 18 {
        return Err(String::from("??"));
    }
    Ok(String::from(kanji[num]))
}

fn extracttegoma(txt: &str) -> Result<(String, String), String> {
    let resente = Regex::new("[PLNSGBRK]").unwrap();
    let regote = Regex::new("[plnsgbrk]").unwrap();
    let mut sentegoma = String::new();
    let mut gotegoma = String::new();
    let mut num = 0;
    for ch in txt.chars() {
        match ch {
            '1'..='9' => num = num * 10 + ch.to_digit(10).unwrap(),
            ch if resente.is_match(&ch.to_string()) => {
                sentegoma = sentegoma
                    + &p2fu(ch.to_ascii_lowercase(), Promote::None)
                    + &kanjinum(num as usize).unwrap();
                num = 0;
            }
            ch if regote.is_match(&ch.to_string()) => {
                gotegoma = gotegoma + &p2fu(ch, Promote::None) + &kanjinum(num as usize).unwrap();
                num = 0;
            }
            '-' => return Ok((String::new(), String::new())),
            _ => return Err(format!("{} is not allowed to use!!", ch)),
        }
    }
    Ok((sentegoma, gotegoma))
}

impl Sfen {
    pub fn new(text: &str) -> Sfen {
        let e: Vec<&str> = text.split(" ").collect();
        Sfen {
            ban: e[0].to_string(),
            teban: e[1].to_string(),
            tegoma: e[2].to_string(),
            nteme: e[3].parse().unwrap_or(-1),
        }
    }
    fn tebanexp(&self) -> Result<String, String> {
        if self.teban == "b" {
            return Ok(String::from("先手の番です。"));
        }
        if self.teban == "w" {
            return Ok(String::from("後手の番です。"));
        }
        if self.teban == "fb" {
            return Ok(String::from("先手の勝ちです。"));
        }
        if self.teban == "fw" {
            return Ok(String::from("後手の勝ちです。"));
        }
        Err(format!("{} is invalid teban expression.", self.teban))
    }
    pub fn dump(&self) -> String {
        let border = "+---------------------------+\n";
        let mut res = String::from(border);
        let vdan: Vec<&str> = self.ban.split("/").collect();
        for e in vdan {
            match extractdan(e) {
                Ok(ret) => res = res + &ret + "\n",
                Err(msg) => return format!("error in [{}]:{}", e, msg),
            }
        }
        match extracttegoma(&self.tegoma) {
            Ok((sentegoma, gotegoma)) => {
                res = format!(
                    "後手の持駒：{}\n{}{}先手の持駒：{}\n",
                    gotegoma, res, border, sentegoma
                )
            }
            Err(msg) => return format!("error in [{}]:{}", self.tegoma, msg),
        }
        match self.tebanexp() {
            Ok(msg) => {
                return res + &format!("手数＝{}　{}", self.nteme, msg);
            }
            Err(msg) => msg,
        }
    }
}
