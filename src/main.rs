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
    // let mut txt = "ln+sgkgsnl/1+r5b1/p1pp+ppp1p/9/9/9/P1PPP+PP1+P/1+B2K2R1/LNS+G1GSNL w 2P2p 2";
    let mut txt = "ln+sgkgsnl/1+r5b1/p1pp+ppp1p/9/9/9/P1PPP+PP1+P/1+B2K2R1/LNS+G1GSNL w 18P4L4N4S4G2B2R18p4l4n4s4g2b2r 2";
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
        Mode::SVG => match sfen.to_svg() {
            Ok(svg) => println!("{}", svg.to_string()),
            Err(msg) => println!("Error:{}", msg),
        },
        Mode::PNG => println!("png will be here."),
        _ => {
            println!("sfen:{}", txt);
            println!("{}", sfen.dump());
        }
    }
}
