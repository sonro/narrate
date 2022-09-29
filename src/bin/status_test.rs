use std::str::FromStr;

use narrate::{report, Color};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: {} <title> <msg> <color>", args[0]);
        std::process::exit(1);
    }

    let title = &args[1];
    let msg = &args[2];
    let color = match Color::from_str(&args[3]) {
        Ok(color) => color,
        Err(err) => {
            eprintln!("error: not a valid color: {}\n{:?}", args[2], err);
            std::process::exit(1);
        }
    };

    report::status(title, msg, color);
}
