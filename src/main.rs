use std::io;

mod sfen;
mod svgbuilder;

enum Mode {
    Text,
    SVG,
    PNG,
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
    println!("\t--help : show this help.");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // let txt = "9/9/9/9/9/9/9/9/9 w - 0";
    // let txt = "lnsgkgsnl/1r5b1/p1ppppp1p/9/9/9/P1PPPPP1P/1B2K2R1/LNSG1GSNL w 2P2p 2";
    let mut txt = "ln+sgkgsnl/1+r5b1/p1pp+ppp1p/9/9/9/P1PPP+PP1+P/1+B2K2R1/LNS+G1GSNL w 2P2p 2";
    // let txt = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";

    let mut md = Mode::Text;
    for e in args[1..].iter() {
        if e == "--svg" {
            md = Mode::SVG;
        } else if e == "--png" {
            md = Mode::PNG;
        } else if e == "--txt" {
            md = Mode::Text;
        } else if e == "--help" {
            help(String::new());
            return;
        } else {
            txt = e;
        }
    }

    let sfen = sfen::Sfen::new(txt);

    match md {
        Mode::SVG => {
            let mut svg = svgbuilder::SVG::new();

            let mut gban = svgbuilder::Tag::new("g");
            gban.addattrib(svgbuilder::Attrib::new("id", String::from("ban")));
            match sfen.extractban() {
                Ok(ban) => {
                    for (i, dan) in ban.iter().enumerate() {
                        let mut gdan = svgbuilder::Tag::new("g");
                        gdan.addattrib(svgbuilder::Attrib::new("id", format!("dan{}", i + 1)));
                        gdan.addattrib(svgbuilder::Attrib::new(
                            "transform",
                            format!("translate(0,{})", i * 20 + 10),
                        ));
                        for (j, k) in dan.iter().enumerate() {
                            if k.is_blank() {
                                continue;
                            }
                            let mut t1 = svgbuilder::Tag::new("text");
                            t1.addattrib(svgbuilder::Attrib::new("x", format!("{}", j * 20)));
                            t1.addattrib(svgbuilder::Attrib::new("y", String::from("0")));
                            t1.value = k.to_kstring().unwrap();
                            gdan.addchild(t1);
                        }
                        gban.addchild(gdan);
                    }
                    svg.tag.addchild(gban);
                }
                Err(msg) => println!("Error:{}", msg),
            }
            println!("{}", svg.to_svg());
        }
        Mode::PNG => println!("png will be here."),
        _ => {
            println!("sfen:{}", txt);
            println!("{}", sfen.dump());
        }
    }
}
