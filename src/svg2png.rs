use std::io::Write;

// w/ rsvg-convert version 2.50
#[allow(dead_code)]
pub fn start_rsvg(svg: String) -> Result<Vec<u8>, String> {
    let mut cmd = match std::process::Command::new("rsvg-convert")
        .arg("--format=png")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Err(msg) => return Err(format!("error running png converter... [{}]", msg)),
        Ok(prcs) => prcs,
    };

    match cmd.stdin.take().unwrap().write_all(&svg.as_bytes()) {
        Ok(_) => {
            let w = cmd.wait_with_output().unwrap();
            // println!("{} bytes.", w.stdout.len());
            Ok(w.stdout)
        }
        Err(msg) => Err(format!("error running png converter... [{}]", msg)),
    }
}

// w/ inkscape version 1.1
#[allow(dead_code)]
pub fn start_inkscape(svg: String) -> Result<Vec<u8>, String> {
    let mut cmd = match std::process::Command::new("inkscape")
        .arg("--pipe")
        .arg("--export-filename=-")
        .arg("--export-type=png")
        .arg("-b")
        .arg("white")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Err(msg) => return Err(format!("error running png converter... [{}]", msg)),
        Ok(prcs) => prcs,
    };

    match cmd.stdin.take().unwrap().write_all(&svg.as_bytes()) {
        Ok(_) => {
            let w = cmd.wait_with_output().unwrap();
            // println!("{} bytes.", w.stdout.len());
            Ok(w.stdout)
        }
        Err(msg) => Err(format!("error running png converter... [{}]", msg)),
    }
}
