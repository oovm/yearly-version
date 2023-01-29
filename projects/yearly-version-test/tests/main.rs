use std::str::FromStr;
use yearly_version::{SemVer, Version, VersionError::MissingPart, VersionTag};

#[test]
fn ensure_size() {
    assert_eq!(std::mem::size_of::<Version>(), 8);
    assert_eq!(std::mem::size_of::<SemVer>(), 40);
}

#[test]
fn parse_initial() {
    assert_eq!(Version::from_str("0"), Err(MissingPart { part: "year".to_string(), offset: 0 }));
    assert_eq!(Version::from_str("0.0"), Err(MissingPart { part: "major".to_string(), offset: 2 }));
    assert_eq!(Version::from_str("0.0.0"), Err(MissingPart { part: "minor".to_string(), offset: 4 }));
    assert_eq!(Version::from_str("0.0.0.0"), Ok(Version { year: 0, major: 0, minor: 0, patch: 0 }));
}

#[test]
fn cast_semver() {
    let sym: SemVer = Version { year: 0, major: 0, minor: 0, patch: 0 }.into();
    println!("{}", sym);
    let sym: SemVer =
        VersionTag { number: Version { year: 0, major: 0, minor: 0, patch: 0 }, tag: "nightly".to_string() }.into();
    println!("{}", sym);
}
