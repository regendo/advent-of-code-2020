use once_cell::sync as once_cell;
use regex;

static RE_HAIR_COLOR: once_cell::Lazy<regex::Regex> =
	once_cell::Lazy::new(|| regex::Regex::new(r"^(#[\da-f]{6})$").unwrap());
static RE_HEIGHT: once_cell::Lazy<regex::Regex> = once_cell::Lazy::new(|| {
	regex::Regex::new(r"^((1[5-8]\d|19[0-3])cm|(59|6\d|7[0-6])in)$").unwrap()
});

/// ```
///# use advent_of_code_2020::day04::validate::byr;
/// assert_eq!(Ok(2002), byr(Some(&"2002")));
///	assert!(byr(Some(&"2003")).is_err());
/// ```
pub fn byr(entry: Option<&&str>) -> Result<u16, &'static str> {
	if let Some(val) = entry {
		if let Ok(year) = val.parse() {
			if year >= 1920 && year <= 2002 {
				Ok(year)
			} else {
				Err("Invalid byr")
			}
		} else {
			Err("Unparsable byr")
		}
	} else {
		Err("Missing field byr")
	}
}

/// ```
///# use advent_of_code_2020::day04::validate::iyr;
/// assert_eq!(Ok(2020), iyr(Some(&"2020")));
///	assert!(iyr(Some(&"2023")).is_err());
/// ```
pub fn iyr(entry: Option<&&str>) -> Result<u16, &'static str> {
	if let Some(val) = entry {
		if let Ok(year) = val.parse() {
			if year >= 2010 && year <= 2020 {
				Ok(year)
			} else {
				Err("Invalid iyr")
			}
		} else {
			Err("Unparsable iyr")
		}
	} else {
		Err("Missing field iyr")
	}
}

/// ```
///# use advent_of_code_2020::day04::validate::eyr;
/// assert_eq!(Ok(2030), eyr(Some(&"2030")));
///	assert!(eyr(Some(&"2031")).is_err());
/// ```
pub fn eyr(entry: Option<&&str>) -> Result<u16, &'static str> {
	if let Some(val) = entry {
		if let Ok(year) = val.parse() {
			if year >= 2020 && year <= 2030 {
				Ok(year)
			} else {
				Err("Invalid eyr")
			}
		} else {
			Err("Unparsable eyr")
		}
	} else {
		Err("Missing field eyr")
	}
}

pub fn hcl<'a, 'b>(entry: Option<&'a &'b str>) -> Result<&'a &'b str, &'static str> {
	if let Some(val) = entry {
		if RE_HAIR_COLOR.is_match(val) {
			Ok(val)
		} else {
			Err("Invalid hcl")
		}
	} else {
		Err("Missing field hcl")
	}
}

/// ```
///# use advent_of_code_2020::day04::validate::pid;
/// assert_eq!(Ok(&"000000001"), pid(Some(&"000000001")));
/// assert!(pid(Some(&"0123456789")).is_err());
/// ```
pub fn pid<'a, 'b>(entry: Option<&'a &'b str>) -> Result<&'a &'b str, &'static str> {
	if let Some(val) = entry {
		if val.len() == 9 && val.parse::<u32>().is_ok() {
			Ok(val)
		} else {
			Err("Invalid pid")
		}
	} else {
		Err("Missing field pid")
	}
}

pub fn ecl<'a, 'b>(entry: Option<&'a &'b str>) -> Result<&'a &'b str, &'static str> {
	if let Some(val) = entry {
		if vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(val) {
			Ok(val)
		} else {
			Err("Invalid ecl")
		}
	} else {
		Err("Missing field ecl")
	}
}

/// ```
///# use advent_of_code_2020::day04::validate::hgt;
/// assert_eq!(Ok(&"60in"), hgt(Some(&"60in")));
/// assert_eq!(Ok(&"190cm"), hgt(Some(&"190cm")));
/// assert!(hgt(Some(&"190in")).is_err());
/// assert!(hgt(Some(&"190")).is_err());
/// ```
pub fn hgt<'a, 'b>(entry: Option<&'a &'b str>) -> Result<&'a &'b str, &'static str> {
	if let Some(val) = entry {
		if RE_HEIGHT.is_match(val) {
			Ok(val)
		} else {
			Err("Invalid hgt")
		}
	} else {
		Err("Missing field hgt")
	}
}
