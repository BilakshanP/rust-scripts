use std::io::{self, BufRead, Write};

pub fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);

    io::stdout().flush()?;
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|x: String| x.trim_end().to_owned())
}

pub fn input2(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    let mut buffer = String::new();

    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}
