//! Print status message to command line

use colored::Color;
use narrate::report;

fn main() {
    report::status("Created", "new project `spacetime`", Color::Green);
}
