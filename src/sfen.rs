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

fn extractdan(txt: &str) -> Result<String, String> {
    let mut res = String::from("|");
    let masu = txt.chars();
    let mut promote = ' ';
    let resente = Regex::new("[PLNSGBRK]").unwrap();
    let regote = Regex::new("[plnsgbrk]").unwrap();
    for ch in masu {
        match ch {
            '1'..='9' => {
                res = res
                    + &std::iter::repeat("  ")
                        .take(ch.to_digit(10).unwrap() as usize)
                        .collect::<String>()
            }
            '+' => promote = '+',
            ch if resente.is_match(&ch.to_string()) => {
                res = res + &promote.to_string() + &ch.to_string();
                promote = ' ';
            }
            ch if regote.is_match(&ch.to_string()) => {
                res = res + &promote.to_string() + &ch.to_string();
                promote = ' ';
            }
            _ => return Err(format!("{} is not allowed to use!!", ch)),
        }
    }
    Ok(res + "|")
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
                if num == 0 {
                    num = 1
                };
                sentegoma = sentegoma + &String::from_utf8(vec![ch as u8; num as usize]).unwrap();
                num = 0;
            }
            ch if regote.is_match(&ch.to_string()) => {
                gotegoma = gotegoma + &String::from_utf8(vec![ch as u8; num as usize]).unwrap();

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
    pub fn dump(&self) -> String {
        let mut res = String::new();
        let vdan: Vec<&str> = self.ban.split("/").collect();
        for e in vdan {
            match extractdan(e) {
                Ok(ret) => res = res + &ret + "\n",
                Err(msg) => return format!("error in [{}]:{}", e, msg),
            }
        }
        match extracttegoma(&self.tegoma) {
            Ok((sentegoma, gotegoma)) => {
                res = format!("gote:{}\n{}sente:{}\n", gotegoma, res, sentegoma)
            }
            Err(msg) => return format!("error in [{}]:{}", self.tegoma, msg),
        }
        res
    }
}
