use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    let mut idx = 0;

    let mut lines = Vec::new();
    loop {
        if idx > 1000000 {
            break;
        };

        lines.push(format!("LINE NUMBER: {}", idx));
        idx += 1;
    }

    let mut file = File::create("text.txt")?;

    file.write_all(lines.join("\n").as_bytes())?;

    println!("DONE!");

    Ok(())
}
