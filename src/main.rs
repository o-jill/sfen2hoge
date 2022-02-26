use std::io::{BufRead, Write};

mod sfen;
mod svgbuilder;

enum Mode {
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

struct MyOptions {
    pub mode: Mode,
    pub lastmove: sfen::LastMove,
    pub sname: String,
    pub gname: String,
    pub title: String,
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
    println!("\t--last 7776FU : emphasizing last move.");
    println!("\t--sente \"John Doe\" : set sente's name.");
    println!("\t--gote \"名無権兵衛\" : set gote's name.");
    println!("\t--title \"title\" : set title.");
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
    };

    let mut lastop = OptionMode::Sfen;
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
        } else if e == "--sente" {
            lastop = OptionMode::SenteName;
        } else if e == "--gote" {
            lastop = OptionMode::GoteName;
        } else if e == "--title" {
            lastop = OptionMode::Title;
        } else if e == "--last" {
            lastop = OptionMode::LastMove;
        } else if e.starts_with("--") {
            help(format!("invalid option {}.", e));
            return;
        } else {
            if lastop == OptionMode::SenteName {
                mo.sname = e.to_string();
                lastop = OptionMode::Sfen;
            } else if lastop == OptionMode::GoteName {
                mo.gname = e.to_string();
                lastop = OptionMode::Sfen;
            } else if lastop == OptionMode::Title {
                mo.title = e.to_string();
                lastop = OptionMode::Sfen;
            } else if lastop == OptionMode::LastMove {
                match sfen::LastMove::read(e) {
                    Ok(lm) => {
                        // println!("{:?}", lm);
                        mo.lastmove = lm;
                        lastop = OptionMode::Sfen;
                    }
                    Err(msg) => {
                        help(msg);
                        return;
                    }
                }
            } else {
                txt = e;
            }
        }
    }

    let sfen = sfen::Sfen::new(txt);

    match mo.mode {
        Mode::SVG => {
            match sfen.to_svg(mo.lastmove.topos(), mo.sname, mo.gname, mo.title) {
                Ok(svg) => println!("{}", svg.to_string()),
                Err(msg) => println!("Error:{}", msg),
            };
        }
        Mode::PNG => {
            /*let svg2png = match std::process::Command::new("inkscape")
            .arg("--pipe")
            .arg("--export-filename=test.png")
            .arg("--export-type=png")
            .arg("-b")
            .arg("white")*/
            let mut svg2png = match std::process::Command::new("rsvg-convert")
                .arg("--format=png")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
            {
                Err(msg) => {
                    println!("error running png converter... [{}]", msg);
                    return;
                }
                Ok(prcs) => prcs,
            };
            let txt;
            match svg2png.stdin.take().unwrap().write_all(
                match sfen.to_svg(mo.lastmove.topos(), mo.sname, mo.gname, mo.title) {
                    Ok(svg) => {
                        txt = svg.to_string();
                        txt.as_bytes()
                    }
                    Err(msg) => {
                        println!("{}", msg);
                        return;
                    }
                },
            ) {
                Err(msg) => {
                    println!("error running png converter... [{}]", msg);
                    return;
                }
                Ok(_) => {
                    // println!("generated png.")
                }
            }
            let w = svg2png.wait_with_output().unwrap();
            // println!("{} bytes.", w.stdout.len());
            std::io::stdout().write_all(&w.stdout).unwrap();
        }
        _ => {
            println!("sfen:{}", txt);
            println!(
                "{}",
                sfen.dump(&mo.sname, &mo.gname, &mo.title, mo.lastmove)
            );
        }
    }
}
