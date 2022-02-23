use std::io;

mod sfen;
mod svgbuilder;

enum Mode {
    Text,
    SVG,
    PNG,
}

struct LastMove {
    suji: usize,
    dan: usize,
    koma: sfen::Koma,
}

impl LastMove {
    fn is_ok(&self) -> bool {
        self.suji > 0 && self.dan > 0
    }
}
struct MyOptions {
    mode: Mode,
    lastmove: LastMove,
}

fn help(msg: String) {
    if !msg.is_empty() {
        println!("{}", msg);
    }
    println!("sfen2reader sfen [options]");
    println!("sfen:");
    println!("ex.\t\"lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1\"");
    println!("options:");
    println!("\t--txt  : text style.");
    println!("\t--svg  : svg style.");
    println!("\t--png  : png style.");
    println!("\t--last[suji][dan] : emphasizing last move.");
    println!("\t--help : show this help.");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // let txt = "9/9/9/9/9/9/9/9/9 w - 0";
    // let txt = "lnsgkgsnl/1r5b1/p1ppppp1p/9/9/9/P1PPPPP1P/1B2K2R1/LNSG1GSNL w 2P2p 2";
    // let mut txt = "ln+sgkgsnl/1+r5b1/p1pp+ppp1p/9/9/9/P1PPP+PP1+P/1+B2K2R1/LNS+G1GSNL w 2P2p 2";
    let mut txt = "ln+sgkgsnl/1+r5b1/p1pp+ppp1p/9/9/9/P1PPP+PP1+P/1+B2K2R1/LNS+G1GSNL w 18P4L4N4S4G2B2R18p4l4n4s4g2b2r 2";
    // let txt = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";

    let mut mo = MyOptions {
        mode: Mode::Text,
        lastmove: LastMove {
            suji: 0,
            dan: 0,
            koma: sfen::Koma::from(' ', sfen::Promotion::None),
        },
    };

    let reg_last = regex::Regex::new("--last([1-9])([1-9])").unwrap();
    for e in args[1..].iter() {
        if e == "--svg" {
            mo.mode = Mode::SVG;
        } else if e == "--png" {
            mo.mode = Mode::PNG;
        } else if e == "--txt" {
            mo.mode = Mode::Text;
        } else if e == "--help" {
            help(String::new());
            return;
        } else if reg_last.is_match(e) {
            let cap = reg_last.captures(e).unwrap();
            if cap.len() != 3 {
                help(format!("invalid option {}.", e));
                return;
            }
            mo.lastmove.suji = cap.get(1).map_or("", |s| s.as_str()).parse().unwrap();
            mo.lastmove.dan = cap.get(2).map_or("", |s| s.as_str()).parse().unwrap();
        } else if e.starts_with("--") {
            help(format!("invalid option {}.", e));
            return;
        } else {
            txt = e;
        }
    }

    let sfen = sfen::Sfen::new(txt);

    match mo.mode {
        Mode::SVG => {
            if mo.lastmove.is_ok(){
                // sfen.set_lastmove();
            }
            match sfen.to_svg() {
                Ok(svg) => println!("{}", svg.to_string()),
                Err(msg) => println!("Error:{}", msg),
            };
        }
        Mode::PNG => println!("png will be here."),
        _ => {
            println!("sfen:{}", txt);
            println!("{}", sfen.dump());
        }
    }
}
