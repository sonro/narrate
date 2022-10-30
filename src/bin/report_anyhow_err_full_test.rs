use anyhow::anyhow;
use narrate::report;

fn main() {
    let mut args = std::env::args();
    let bin_name = args.next().expect("will always has executable name");

    // must have at least 1 argument
    let mut error = match args.next() {
        Some(msg) => anyhow!(msg),
        None => {
            eprintln!("usage: {} <error list>", bin_name);
            std::process::exit(1);
        }
    };

    for arg in args {
        error = error.context(arg);
    }

    report::anyhow_err_full(&error);
}
