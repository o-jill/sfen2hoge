// struct Tegoma {
//     koma: Vec<i8>,
// }

pub struct Sfen {
    ban: String,
    teban: String,
    tegoma: String,
    nteme: i32,
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
            res = res + &e.to_string() + "\n";
        }
        res
    }
}
