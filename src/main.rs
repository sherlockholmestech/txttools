/*
Text tools written in Rust.
Copyright (C) 2023  Sherlock

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub mod cringe;
pub mod caps;

use crate::cringe::cringe::cringeText;
use crate::caps::caps::{allCaps, allNoCaps};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "txttools")]
#[command(author = "Sherlock Holmes")]
#[command(version = "0.0.1")]
#[command(about = "Text Tools written in Rust", long_about = "Text tools written in Rust.\nCopyright (C) 2023  Sherlock\n\nThis program is free software: you can redistribute it and/or modify\nit under the terms of the GNU General Public License as published by\nthe Free Software Foundation, either version 3 of the License, or\n(at your option) any later version.\n\nThis program is distributed in the hope that it will be useful,\nbut WITHOUT ANY WARRANTY; without even the implied warranty of\nMERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\nGNU General Public License for more details.\n\nYou should have received a copy of the GNU General Public License\nalong with this program.  If not, see <https://www.gnu.org/licenses/>.")]
#[command(propagate_version = true)]
struct Cli {
    /// Which text command to use.
    #[command(subcommand)]
    mode: Modes,

}

#[derive(Subcommand)]
enum Modes {
    /// Makes your text look cringe.
    Cringe(Cringe),
    /// Capitalise all letters.
    Caps(Caps),
    /// Un-capitalise all letters aka make them small.
    Nocaps(NoCaps),
}

#[derive(Args)]
struct Cringe {
    /// retardation factor (or level).  The higher the worse the capitalisation.  Accepted values: 1 - 9.
    #[arg(short, long, default_value_t = 8)]
    retard: i32,
    /// The input text
    text: String
}
#[derive(Args)]
struct Caps {
    /// The input text.
    text: String
}
#[derive(Args)]
struct NoCaps {
    /// The input text.
    text: String
}

fn main() {
    let cli = Cli::parse();
    match &cli.mode {
        Modes::Cringe(i) => {
            let return_text = cringeText(i.text.as_str(), i.retard);
            println!("{}", return_text);
        },
        Modes::Caps(i) => {
            let return_text = allCaps(i.text.as_str());
            println!("{}", return_text);
        },
        Modes::Nocaps(i) => {
            let return_text = allNoCaps(i.text.as_str());
            println!("{}", return_text);
        },
    }
}
