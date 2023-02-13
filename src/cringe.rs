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

pub mod cringe{
	use rand::Rng;
	pub fn cringeText(text: &str, retard: i32) -> String {
		let mut rng = rand::thread_rng();
		let mut cringeText = String::new();
		let mut isCapital = rng.gen_bool(0.5);
		let mut capsInARow = 0;
		for char in text.chars() {
			if isCapital {
				cringeText.push(char.to_ascii_uppercase());
				capsInARow += 1;
			} else {
				cringeText.push(char.to_ascii_lowercase());
				capsInARow = 0;
			}
			if capsInARow <= 10 - retard {
				isCapital = rng.gen_bool(0.5);
			} else {
				isCapital = false;
				capsInARow = 0;
			}
		}
		return cringeText;
	}
}