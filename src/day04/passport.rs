use once_cell::sync as once_cell;
use regex;
use std::{collections::HashMap, convert::TryFrom, error::Error, str::FromStr};

static KEY_VALUE_PAIR: once_cell::Lazy<regex::Regex> =
	once_cell::Lazy::new(|| regex::Regex::new(r"(?P<key>\w+):(?P<value>[#\d\w]+)\s?").unwrap());

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub struct Passport<'a> {
	birth_year: u16,             // byr
	issue_year: u16,             // iyr
	expiration_year: u16,        // eyr
	height: &'a str,             // hgt
	hair_color: &'a str,         // hcl
	eye_color: &'a str,          // ecl
	passport_id: &'a str,        // pid
	country_id: Option<&'a str>, // cid
}

impl<'a> TryFrom<&'a str> for Passport<'a> {
	type Error = Box<dyn Error>;

	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

		let vals: HashMap<&str, &str> = KEY_VALUE_PAIR
			.captures_iter(s)
			.filter_map(|cap| match cap.name("key").map(|m| m.as_str()) {
				Some(key) if keys.contains(&key) => {
					if let Some(value) = cap.name("value").map(|m| m.as_str()) {
						Some((key, value))
					} else {
						None
					}
				}
				_ => None,
			})
			.collect();

		(move || {
			Ok(Passport {
				birth_year: u16::from_str(vals.get("byr").ok_or("Missing field 'byr'")?)?,
				issue_year: u16::from_str(vals.get("iyr").ok_or("Missing field 'iyr'")?)?,
				expiration_year: u16::from_str(vals.get("eyr").ok_or("Missing field 'eyr'")?)?,
				height: vals.get("hgt").ok_or("Missing field 'hgt'")?,
				hair_color: vals.get("hcl").ok_or("Missing field 'hcl'")?,
				eye_color: vals.get("ecl").ok_or("Missing field 'ecl'")?,
				passport_id: vals.get("pid").ok_or("Missing field 'pid'")?,
				// CID is optional
				// But honestly no idea if this ".cloned" is the correct fix for my lifetime issues.
				country_id: vals.get("cid").cloned(),
			})
		})()
	}
}

pub fn parse(input: &str) -> Vec<Result<Passport, Box<dyn Error>>> {
	// This fixes issues with newlines in files, but also breaks inline strings 🤦
	#[cfg(unix)]
	let blankline = "\n\n";
	#[cfg(windows)]
	let blankline = "\r\n\r\n";

	input
		.split(blankline)
		.map(|chunk| Passport::try_from(chunk.trim()))
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn passport_constructs() {
		let input = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
		let expected = Passport {
			eye_color: "gry",
			passport_id: "860033327",
			expiration_year: 2020,
			birth_year: 1937,
			issue_year: 2017,
			country_id: Some("147"),
			height: "183cm",
			hair_color: "#fffffd",
		};

		let actual = Passport::try_from(input).unwrap();
		assert_eq!(expected, actual)
	}

	#[test]
	fn example_1_works() {
		let input = include_str!("example_1.txt");
		let parsed = parse(input);
		let expected = vec![true, false, true, false];

		assert_eq!(
			expected,
			parsed.iter().map(|p| p.is_ok()).collect::<Vec<bool>>()
		)
	}
}
