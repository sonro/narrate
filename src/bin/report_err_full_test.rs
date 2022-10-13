use narrate::{error_from, report};

fn main() {
    let mut args = std::env::args();
    let bin_name = args.next().expect("will always has executable name");

    // must have at least 1 argument
    let mut error = match args.next() {
        Some(msg) => error_from!(msg),
        None => {
            eprintln!("usage: {} <error list [-h help msg]>", bin_name);
            std::process::exit(1);
        }
    };

    let mut help_flag = false;

    for arg in args {
        if arg == "-h" {
            help_flag = true;
            continue;
        }
        if help_flag {
            error.add_help_with(|| arg);
            help_flag = false;
            continue;
        }
        error = error.wrap(arg);
    }

    if help_flag {
        eprintln!("Expected help message after -h");
        std::process::exit(1);
    }

    report::err_full(&error);
}
