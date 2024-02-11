use std::{fs::File, io::Write};
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut idx = 0;

    let mut lines = Vec::new();
    loop {
        if idx > 1000000 {
            break;
        };

        // if odd we insert that if not we insert the other
        if idx % 2 == 0 {
            lines.push(format!("Hello, Rust! {}", idx));
        } else {
            lines.push(format!("We are trying the new Rust IDE: {}", idx));
        }

        idx += 1;
    }

    let mut file = File::create("text.txt")?;

    file.write_all(lines.join("\n").as_bytes())?;

    // After the file is written, we can print a message to the console



 // Then we need to get what we write

    let mut file = File::open("text.txt")?;

    let mut contents = String::new();


    file.read_to_string(&mut contents)?;

    // Print first lines

    let first_lines = contents.lines().take(5);

    for line in first_lines {
        println!("{}", line);
    }

    Ok(())
}
