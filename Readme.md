[![CC0](https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1)![CC0](https://mirrors.creativecommons.org/presskit/icons/zero.svg?ref=chooser-v1)](ref="http://creativecommons.org/publicdomain/zero/1.0?ref=chooser-v1)
[![issues](https://img.shields.io/github/issues/o-jill/sfen2hoge.svg)](https://github.com/o-jill/sfen2hoge/issues/)
[![Rust](https://github.com/o-jill/sfen2hoge/actions/workflows/rust.yml/badge.svg)](https://github.com/o-jill/sfen2hoge/actions/workflows/rust.yml)

sfen2hoge is a converter from sfen to text, svg and png.  

rsvg-convert (and inkscape) are supported to generate png.

## command help:  
sfen2reader sfen [options]  
sfen:  
ex.     "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1"  
options:  
        --txt  : text style.  
        --svg  : svg style.  
        --png  : png style.  
        --last 7776FU : emphasizing last move.  
        --sente "John Doe" : set sente's name.  
        --gote "名無権兵衛" : set gote's name.  
        --title "title" : set title.  
        --help : show this help.  

## examples:
* png  
./sfen2hoge "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1" --png --last 0065FU --sente "John Doe" --gote "日 本 語" --title "my kyokumen" >mypng.png
* svg  
./sfen2hoge "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1" --png --last 0099KY --sente "John Doe" --gote "日 本 語" --title "my svg kyokumen" >test.svg
* text to terminal  
./sfen2hoge "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1" --png --last 0065RYRH --sente "John Doe" --gote "日 本 語" --title "コメントだよ"

---
