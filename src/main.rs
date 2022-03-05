use std::io::Write;

mod myoptions;
mod sfen;
mod svg2png;
mod svgbuilder;

fn help(msg: String) {
    if !msg.is_empty() {
        eprintln!("{}", msg);
    }
    eprintln!(
        "sfen2reader sfen [options]\n\
        sfen:\n\
        ex.\t\"lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1\"\n\
        options:\n\
        \t--txt  : text style.\n\
        \t--svg  : svg style.\n\
        \t--png  : png style.\n\
        \t--last 7776FU : emphasizing last move.\n\
        \t--sente \"John Doe\" : set sente's name.\n\
        \t--gote \"名無権兵衛\" : set gote's name.\n\
        \t--title \"title\" : set title.\n\
        \t--help : show this help."
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut txt = String::from("9/9/9/9/9/9/9/9/9 w - 0");
    // let txt = "lnsgkgsnl/1r5b1/p1ppppp1p/9/9/9/P1PPPPP1P/1B2K2R1/LNSG1GSNL w 2P2p 2";
    // let mut txt = "ln+sgkgsnl/1+r5b1/p1pp+ppp1p/9/9/9/P1PPP+PP1+P/1+B2K2R1/LNS+G1GSNL w 2P2p 2";
    // let mut txt = "ln+sgkgsnl/1+r5b1/p1pp+ppp1p/9/9/9/P1PPP+PP1+P/1+B2K2R1/LNS+G1GSNL \
    //               w 18P4L4N4S4G2B2R18p4l4n4s4g2b2r 2";
    // let txt = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";

    let mut mo = myoptions::MyOptions::new();

    match mo.read_options(args) {
        Ok(sfen) => {
            if !txt.is_empty() {
                txt = sfen
            }
        }
        Err(msg) => return help(msg),
    }

    let sfen = sfen::Sfen::new(&txt);

    match mo.mode {
        myoptions::Mode::SVG => {
            match sfen.to_svg(mo.lastmove.topos(), mo.sname, mo.gname, mo.title) {
                Ok(svg) => println!("{}", svg.to_string()),
                Err(msg) => {
                    help(msg);
                    return;
                }
            };
        }
        myoptions::Mode::PNG => {
            match sfen.to_svg(mo.lastmove.topos(), mo.sname, mo.gname, mo.title) {
                Ok(svg) => {
                    match svg2png::start_rsvg(svg.to_string()) {
                    // match svg2png::start_inkscape(svg.to_string()) {
                        Err(msg) => {
                            help(msg);
                        }
                        Ok(png) => {
                            // println!("generated png.")
                            std::io::stdout().write_all(&png).unwrap();
                        }
                    }
                }
                Err(msg) => {
                    help(msg);
                }
            }
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
