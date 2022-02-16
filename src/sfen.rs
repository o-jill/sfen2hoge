use super::*;
use regex::Regex;
use svgbuilder::*;

pub struct Sfen {
    ban: String,
    teban: String,
    tegoma: String,
    nteme: i32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Teban {
    Sente,
    Gote,
    None,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum KomaType {
    Aki,
    Fu,
    Kyosha,
    Keima,
    Gin,
    Kin,
    Kaku,
    Hisha,
    Gyoku,
}

impl KomaType {
    pub fn to_string(&self, promote: Promotion) -> String {
        let idx = [
            KomaType::Fu,
            KomaType::Kyosha,
            KomaType::Keima,
            KomaType::Gin,
            KomaType::Kin,
            KomaType::Kaku,
            KomaType::Hisha,
            KomaType::Gyoku,
            KomaType::Aki,
        ]
        .iter()
        .position(|&k| k == *self)
        .unwrap();
        if promote.is_promoted() {
            "と杏圭全金馬龍玉"
        } else {
            "歩香桂銀金角飛玉"
        }
        .chars()
        .nth(idx)
        .unwrap()
        .to_string()
    }

    pub fn from(ch: char) -> KomaType {
        let idx = "PLNSGBRK"
            .chars()
            .position(|k| k == ch.to_ascii_uppercase())
            .unwrap_or(8);
        [
            KomaType::Fu,
            KomaType::Kyosha,
            KomaType::Keima,
            KomaType::Gin,
            KomaType::Kin,
            KomaType::Kaku,
            KomaType::Hisha,
            KomaType::Gyoku,
            KomaType::Aki,
        ][idx]
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Promotion {
    None,
    Promoted,
}

impl Promotion {
    pub fn is_promoted(&self) -> bool {
        *self == Promotion::Promoted
    }
}

#[derive(Clone)]
pub struct Koma {
    koma: KomaType,
    promotion: Promotion,
    teban: Teban,
}

impl Koma {
    pub fn from(ch: char, promote: Promotion) -> Koma {
        Koma {
            koma: KomaType::from(ch),
            promotion: promote,
            teban: if ch.is_uppercase() {
                Teban::Sente
            } else {
                Teban::Gote
            },
        }
    }

    pub fn to_string(&self) -> String {
        if self.teban == Teban::None || self.koma == KomaType::Aki {
            return String::from(" ・");
        }

        String::from(if self.teban == Teban::Sente { " " } else { "v" })
            + &self.koma.to_string(self.promotion)
    }

    pub fn to_kstring(&self) -> Option<String> {
        if self.teban == Teban::None || self.koma == KomaType::Aki {
            return None;
        }
        Some(self.koma.to_string(self.promotion))
    }

    pub fn is_blank(&self) -> bool {
        self.koma == KomaType::Aki
    }
}

pub struct Tegoma {
    koma: KomaType,
    num: usize,
}

impl Tegoma {
    pub fn new(p: char, n: usize) -> Tegoma {
        Tegoma {
            koma: KomaType::from(p),
            num: n,
        }
    }
    pub fn to_kanji(&self) -> Result<String, String> {
        let kanji = self.koma.to_string(Promotion::None);
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

fn extractdan(txt: &str) -> Result<Vec<Koma>, String> {
    let mut res = Vec::<Koma>::new();
    let masu = txt.chars();
    let mut promote = Promotion::None;
    let rekoma = Regex::new("[PLNSGBRK]").unwrap();
    for ch in masu {
        match ch {
            '1'..='9' => {
                res.append(&mut vec![
                    Koma::from('?', Promotion::None);
                    ch.to_digit(10).unwrap() as usize
                ]);
            }
            ch if rekoma.is_match(&ch.to_ascii_uppercase().to_string()) => {
                res.push(Koma::from(ch, promote));
                promote = Promotion::None;
            }
            '+' => promote = Promotion::Promoted,
            _ => return Err(format!("{} is not allowed to use!!", ch)),
        }
    }
    Ok(res)
}

impl Sfen {
    pub fn new(text: &str) -> Sfen {
        let e: Vec<&str> = text.split(" ").collect();
        if e.len() < 4 {
            return Sfen {
                ban: String::new(),
                teban: String::new(),
                tegoma: String::new(),
                nteme: -2,
            };
        }
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
    pub fn extractban(&self) -> Result<Vec<Vec<Koma>>, String> {
        let mut masus: Vec<Vec<Koma>> = Vec::new();
        let vdan: Vec<&str> = self.ban.split("/").collect();
        for e in vdan.iter() {
            match extractdan(e) {
                Ok(ret) => masus.push(ret),
                Err(msg) => return Err(msg),
            }
        }
        return Ok(masus);
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
                Ok(ret) => {
                    res = format!(
                        "{}|{}|{}\n",
                        res,
                        ret.iter()
                            .map(|koma| koma.to_string())
                            .collect::<Vec<String>>()
                            .join(""),
                        dannum.chars().nth(i).unwrap()
                    );
                }
                Err(msg) => return format!("error in [{}]:{}", e, msg),
            }
            // match dumpextractdan(e) {
            //     Ok(ret) => res = res + &ret + &dannum.chars().nth(i).unwrap().to_string() + "\n",
            // }
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

    pub fn to_svg(&self) -> Result<SVG, String> {
        let mut svg = SVG::new();
        let mut gban = Tag::new("g");
        gban.addattrib(Attrib::new("id", String::from("board")));
        match self.extractban() {
            Ok(ban) => {
                for (i, dan) in ban.iter().enumerate() {
                    let mut gdan = Tag::new("g");
                    gdan.addattrib(Attrib::new("id", format!("dan{}", i + 1)));
                    gdan.addattrib(Attrib::new(
                        "transform",
                        format!("translate(0,{})", i * 20 + 10),
                    ));
                    for (j, k) in dan.iter().enumerate() {
                        if k.is_blank() {
                            continue;
                        }
                        let mut t1 = Tag::new("text");
                        t1.addattrib(Attrib::new("x", format!("{}", j * 20)));
                        t1.addattrib(Attrib::new("y", String::from("0")));
                        t1.value = k.to_kstring().unwrap();
                        gdan.addchild(t1);
                    }
                    gban.addchild(gdan);
                }
                svg.tag.addchild(gban);
            }
            Err(msg) => return Err(msg),
        }
        Ok(svg)
    }
}
