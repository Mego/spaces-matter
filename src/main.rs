use std::{env, io};

use spaces_matter::parse;

fn main() {
    let s = env::args().nth(1).or_else(|| {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).ok()?;
        Some(buf)
    });
    if let Some(s) = s {
        println!("{}", parse(s));
    }
}
