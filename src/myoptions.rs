use super::*;

pub enum Mode {
    Text,
    SVG,
    PNG,
}

#[derive(PartialEq)]
enum OptionMode {
    Sfen,
    SenteName,
    GoteName,
    Title,
    LastMove,
}

pub struct MyOptions {
    pub mode: Mode,
    pub lastmove: sfen::LastMove,
    pub sname: String,
    pub gname: String,
    pub title: String,
}

impl MyOptions {
    pub fn new() -> MyOptions {
        MyOptions {
            mode: Mode::Text,
            lastmove: sfen::LastMove {
                from: (0, 0),
                to: (0, 0),
                koma: sfen::Koma::from(' ', sfen::Promotion::None),
                promote: sfen::Promotion::None,
                dir: String::new(),
            },
            sname: String::new(),
            gname: String::new(),
            title: String::new(),
        }
    }

    pub fn read_options(&mut self, args: Vec<String>) -> Result<String, String> {
        let mut txt = String::new();
        let mut lastop = OptionMode::Sfen;
        for e in args.iter() {
            if e == "--svg" {
                self.mode = Mode::SVG;
            } else if e == "--png" {
                self.mode = Mode::PNG;
            } else if e == "--txt" {
                self.mode = Mode::Text;
            } else if e == "--help" {
                return Err(String::new());
            } else if e == "--sente" {
                lastop = OptionMode::SenteName;
            } else if e == "--gote" {
                lastop = OptionMode::GoteName;
            } else if e == "--title" {
                lastop = OptionMode::Title;
            } else if e == "--last" {
                lastop = OptionMode::LastMove;
            } else if e.starts_with("--") {
                return Err(format!("invalid option {}.", e));
            } else {
                if lastop == OptionMode::SenteName {
                    self.sname = e.to_string();
                    lastop = OptionMode::Sfen;
                } else if lastop == OptionMode::GoteName {
                    self.gname = e.to_string();
                    lastop = OptionMode::Sfen;
                } else if lastop == OptionMode::Title {
                    self.title = e.to_string();
                    lastop = OptionMode::Sfen;
                } else if lastop == OptionMode::LastMove {
                    match sfen::LastMove::read(e) {
                        Ok(lm) => {
                            // println!("{:?}", lm);
                            self.lastmove = lm;
                            lastop = OptionMode::Sfen;
                        }
                        Err(msg) => {
                            return Err(msg);
                        }
                    }
                } else {
                    txt = e.clone();
                }
            }
        }
        Ok(txt)
    }
}
