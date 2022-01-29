use std::io;

mod sfen;

fn main() {
    // let txt = "9/9/9/9/9/9/9/9/9 w - 0";
    let txt = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let sfen = sfen::Sfen::new(txt);
    println!("{}", sfen.dump());
}
