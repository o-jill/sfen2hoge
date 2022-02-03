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
pub enum Promote {
    None,
    Promoted,
}

impl Promote {
    pub fn is_promoted(&self) -> bool {
        *self == Promote::Promoted
    }
}

pub struct Tegoma {
    koma: char, // plnsgbrk
    num: usize,
}

impl Tegoma {
    pub fn new(p: char, n: usize) -> Tegoma {
        Tegoma { koma: p, num: n }
    }
    pub fn to_kanji(&self) -> Result<String, String> {
        let kanji = p2fu(self.koma, Promote::None);
        let kanjinum = [
            "", "", /*"一"*/
            "二", "三", "四", "五", "六", "七", "八", "九", "十", "十一", "十二", "十三", "十四",
            "十五", "十六", "十七", "十八",
        ];
        if self.num > 18 {
            return Err(kanji + &String::from("??"));
        }
        if self.num == 0 {
            return Ok(String::new());
        }
        Ok(kanji + &kanjinum[self.num])
    }
}

fn p2fu(piece: char, promote: Promote) -> String {
    let idx = "plnsgbrk".find(piece).unwrap_or(8);
    if promote.is_promoted() {
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

    fn extracttegoma(&self) -> Result<(Vec<Tegoma>, Vec<Tegoma>), String> {
        let resente = Regex::new("[PLNSGBRK]").unwrap();
        let regote = Regex::new("[plnsgbrk]").unwrap();
        let mut sentegoma = Vec::new();
        let mut gotegoma = Vec::new();
        let mut num = 0;
        for ch in self.tegoma.chars() {
            match ch {
                '1'..='9' => num = num * 10 + ch.to_digit(10).unwrap(),
                ch if resente.is_match(&ch.to_string()) => {
                    sentegoma.push(Tegoma::new(ch.to_ascii_lowercase(), num as usize));
                    num = 0;
                }
                ch if regote.is_match(&ch.to_string()) => {
                    gotegoma.push(Tegoma::new(ch, num as usize));
                    // gotegoma = gotegoma + &p2fu(ch, Promote::None) + &kanjinum(num as usize).unwrap();
                    num = 0;
                }
                '-' => break,
                _ => return Err(format!("{} is not allowed to use!!", ch)),
            }
        }
        Ok((sentegoma, gotegoma))
    }

    pub fn dump(&self) -> String {
        let border = "+---------------------------+\n";
        let dannum = "一二三四五六七八九";
        let mut res = format!("  ９ ８ ７ ６ ５ ４ ３ ２ １\n{}", border);
        let vdan: Vec<&str> = self.ban.split("/").collect();
        for (i, e) in vdan.iter().enumerate() {
            match extractdan(e) {
                Ok(ret) => res = res + &ret + &dannum.chars().nth(i).unwrap().to_string() + "\n",
                Err(msg) => return format!("error in [{}]:{}", e, msg),
            }
        }
        match self.extracttegoma() {
            Ok((sentegoma, gotegoma)) => {
                let tgmsen = sentegoma
                    .iter()
                    .map(|t| t.to_kanji().unwrap())
                    .collect::<Vec<String>>()
                    .join("");
                let tgmgo = gotegoma
                    .iter()
                    .map(|t| t.to_kanji().unwrap())
                    .collect::<Vec<String>>()
                    .join("");
                res = format!(
                    "後手の持駒：{}\n{}{}先手の持駒：{}\n",
                    tgmgo, res, border, tgmsen
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
