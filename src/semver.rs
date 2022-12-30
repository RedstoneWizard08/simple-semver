use std::num::ParseIntError;

use regex::Regex;

use crate::{
    change::ChangeType,
    errors::{CheckError, ParseError},
    SemVerChangeID,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SemVer {
    pub major: i32,
    pub minor: i32,
    pub patch: Option<i32>,
    pub change: Option<SemVerChangeID>,
}

unsafe impl Send for SemVer {}
unsafe impl Sync for SemVer {}

impl SemVer {
    pub fn from_many(versions: Vec<&str>) -> Result<Vec<SemVer>, ParseError> {
        let mut semvers: Vec<SemVer> = Vec::new();

        for version in versions {
            let semver = SemVer::from(version);

            if semver.is_err() {
                return Err(semver.unwrap_err());
            }

            semvers.push(semver.unwrap());
        }

        return Ok(semvers);
    }

    pub fn from(version: &str) -> Result<SemVer, ParseError> {
        let rx = Regex::new(r#"(?i)([0-9]+)\.([0-9]+)\.?([0-9]+)?-?((?:alpha|beta)?\.?[0-9]+)?"#)
            .unwrap();

        let matches = rx.captures(version);

        match matches {
            Some(result) => {
                let major_res = result.get(1);
                let minor_res = result.get(2);
                let patch_res = result.get(3);
                let change_res = result.get(4);

                if major_res.is_none() || minor_res.is_none() {
                    return Err(ParseError::InvalidSemver);
                }

                let major_str = major_res.unwrap();
                let minor_str = minor_res.unwrap();

                let major_parse: Result<i32, ParseIntError> = major_str.as_str().parse::<i32>();
                let minor_parse: Result<i32, ParseIntError> = minor_str.as_str().parse::<i32>();

                if major_parse.is_err() || minor_parse.is_err() {
                    return Err(ParseError::InvalidNumber);
                }

                let major = major_parse.unwrap();
                let minor = minor_parse.unwrap();
                let mut patch: Option<i32> = None;
                let mut change: Option<SemVerChangeID> = None;

                if patch_res.is_some() {
                    let patch_str = patch_res.unwrap();
                    let patch_parse: Result<i32, ParseIntError> = patch_str.as_str().parse::<i32>();

                    if patch_parse.is_err() {
                        return Err(ParseError::InvalidNumber);
                    }

                    patch = Some(patch_parse.unwrap());
                }

                if change_res.is_some() {
                    let change_str = change_res.unwrap().as_str().to_lowercase();
                    let change_str_id = change_str
                        .replace("alpha", "")
                        .replace("beta", "")
                        .replace(".", "");
                    let change_parse: Result<i32, ParseIntError> = change_str_id.parse::<i32>();

                    if change_parse.is_err() {
                        return Err(ParseError::InvalidNumber);
                    }

                    let change_type: ChangeType;

                    match change_str.replace(change_str_id.as_str(), "").as_str() {
                        "alpha" | "alpha." => change_type = ChangeType::Alpha,
                        "beta" | "beta." => change_type = ChangeType::Beta,
                        _ => change_type = ChangeType::None,
                    };

                    let change_struct = SemVerChangeID {
                        r#type: change_type,
                        id: change_parse.unwrap(),
                    };

                    change = Some(change_struct);
                }

                return Ok(SemVer {
                    major,
                    minor,
                    patch,
                    change,
                });
            }

            None => return Err(ParseError::InvalidSemver),
        };
    }

    pub fn gt(&self, other: &SemVer) -> Result<bool, CheckError> {
        if other.major == self.major {
            if other.minor == self.minor {
                if other.patch == self.patch {
                    if other.change.is_none() || self.change.is_none() {
                        return Err(CheckError::SameValue);
                    }

                    let other_change = other.change.unwrap();
                    let my_change = self.change.unwrap();

                    if other_change.eq(my_change) {
                        return Err(CheckError::SameValue);
                    } else if other_change.gt(my_change) {
                        return Ok(false);
                    } else if other_change.lt(my_change) {
                        return Ok(true);
                    }
                } else if other.patch > self.patch {
                    return Ok(false);
                } else if other.patch < self.patch {
                    return Ok(true);
                }
            } else if other.minor > self.minor {
                return Ok(false);
            } else if other.minor < self.minor {
                return Ok(true);
            }
        } else if other.major > self.major {
            return Ok(false);
        } else if other.major < self.major {
            return Ok(true);
        }

        return Err(CheckError::UnknownValues);
    }

    pub fn lt(&self, other: &SemVer) -> Result<bool, CheckError> {
        if other.major == self.major {
            if other.minor == self.minor {
                if other.patch == self.patch {
                    if other.change.is_none() || self.change.is_none() {
                        return Err(CheckError::SameValue);
                    }

                    let other_change = other.change.unwrap();
                    let my_change = self.change.unwrap();

                    if other_change.eq(my_change) {
                        return Err(CheckError::SameValue);
                    } else if other_change.gt(my_change) {
                        return Ok(true);
                    } else if other_change.lt(my_change) {
                        return Ok(false);
                    }
                } else if other.patch > self.patch {
                    return Ok(true);
                } else if other.patch < self.patch {
                    return Ok(false);
                }
            } else if other.minor > self.minor {
                return Ok(true);
            } else if other.minor < self.minor {
                return Ok(false);
            }
        } else if other.major > self.major {
            return Ok(true);
        } else if other.major < self.major {
            return Ok(false);
        }

        return Err(CheckError::UnknownValues);
    }

    pub fn eq(&self, other: SemVer) -> bool {
        let mut same_change = false;

        if other.change.is_none() && self.change.is_none() {
            same_change = true;
        } else {
            if other.change.is_some() && self.change.is_some() {
                if other.change.unwrap().eq(self.change.unwrap()) {
                    same_change = true;
                }
            }
        }

        return other.major == self.major
            && other.minor == self.minor
            && other.patch == self.patch
            && same_change;
    }

    pub fn to_string(&self) -> String {
        let major = self.major.to_string();
        let minor = self.minor.to_string();

        if self.patch.is_some() {
            let patch = self.patch.unwrap().to_string();

            if self.change.is_some() {
                let change = self.change.unwrap().to_string();

                return format!("{}.{}.{}-{}", major, minor, patch, change);
            }

            return format!("{}.{}.{}", major, minor, patch);
        } else if self.change.is_some() {
            let change = self.change.unwrap().to_string();

            return format!("{}.{}-{}", major, minor, change);
        }

        return format!("{}.{}", major, minor);
    }
}
