use std::io;

mod sfen;

fn main() {
    let txt = "9/9/9/9/9/9/9/9/9 w - 0";
    let sfen = sfen::Sfen::new(txt);
    println!("{}", sfen.dump());
}
