use std::str::FromStr;
use yearly_version::{
    SemVer, Version,
    VersionError::{InvalidPart, MissingPart},
    VersionTag,
};
mod test_parser;
#[test]
fn ensure_size() {
    assert_eq!(std::mem::size_of::<Version>(), 8);
    assert_eq!(std::mem::size_of::<SemVer>(), 40);
}


#[test]
fn cast_u64() {
    let num: u64 = Version { year: 0, major: 0, minor: 0, patch: 0 }.into();
    assert_eq!(num, 0);
    let num: u64 = Version { year: 0, major: 0, minor: 0, patch: 1 }.into();
    assert_eq!(num, 0);
}

#[test]
fn cast_semver() {
    let sym: SemVer = Version { year: 0, major: 0, minor: 0, patch: 0 }.into();
    assert_eq!(format!("{sym}"), "0.0.0");
    let sym: SemVer = Version { year: 0, major: 0, minor: 0, patch: 1 }.into();
    assert_eq!(format!("{sym}-pre.1"), "0.0.0");

    let sym: SemVer =
        VersionTag { number: Version { year: 0, major: 0, minor: 0, patch: 0 }, tag: "nightly".to_string() }.into();
    println!("{}", sym);
}
