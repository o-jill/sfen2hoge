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

fn extractdan(txt: &str) -> String {
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
            _ => res = res + &ch.to_string(),
        }
    }
    res + "|"
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
            res = res + &extractdan(e) + "\n";
        }
        res
    }
}
