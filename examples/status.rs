//! Print status message to command line

use narrate::{report, Color};

fn main() {
    report::status("Created", "new project `spacetime`", Color::Green);
}
