use super::*;

#[test]
fn parse_initial() {
    assert_eq!(Version::from_str("0"), Err(MissingPart { part: "major".to_string(), offset: 0 }));
    assert_eq!(Version::from_str("x"), Err(MissingPart { part: "major".to_string(), offset: 0 }));

    assert_eq!(Version::from_str("0.0"), Err(MissingPart { part: "minor".to_string(), offset: 2 }));
    assert_eq!(Version::from_str("0.x"), Err(MissingPart { part: "minor".to_string(), offset: 2 }));
    assert_eq!(Version::from_str("x.x"), Err(InvalidPart { part: "year".to_string(), start: 0, end: 1 }));

    assert_eq!(Version::from_str("0.0.0"), Err(MissingPart { part: "patch".to_string(), offset: 4 }));
    assert_eq!(Version::from_str("0.0.x"), Err(MissingPart { part: "patch".to_string(), offset: 4 }));
    assert_eq!(Version::from_str("0.x.x"), Err(InvalidPart { part: "major".to_string(), start: 2, end: 3 }));
    assert_eq!(Version::from_str("x.x.x"), Err(InvalidPart { part: "year".to_string(), start: 0, end: 1 }));

    assert_eq!(Version::from_str("0.0.0.0"), Ok(Version { year: 0, major: 0, minor: 0, patch: 0 }));
    assert_eq!(Version::from_str("0.0.0.x"), Err(InvalidPart { part: "patch".to_string(), start: 6, end: 7 }));
    assert_eq!(Version::from_str("0.0.x.x"), Err(InvalidPart { part: "minor".to_string(), start: 4, end: 5 }));
    assert_eq!(Version::from_str("0.x.x.x"), Err(InvalidPart { part: "major".to_string(), start: 2, end: 3 }));
    assert_eq!(Version::from_str("x.x.x.x"), Err(InvalidPart { part: "year".to_string(), start: 0, end: 1 }));

    assert_eq!(Version::from_str("0.0.0.0.0"), Err(InvalidPart { part: "patch".to_string(), start: 6, end: 9 }));
}

#[test]
fn parse_version() {
    assert_eq!(Version::from_str("0.0.0.0"), Ok(Version { year: 0, major: 0, minor: 0, patch: 0 }));
    assert_eq!(Version::from_str("0.0.0.256"), Ok(Version { year: 0, major: 0, minor: 0, patch: 256 }));
    assert_eq!(Version::from_str("0.0.0.65535"), Ok(Version { year: 0, major: 0, minor: 0, patch: 65535 }));
    assert_eq!(Version::from_str("0.0.0.65536"), Err(InvalidPart { part: "patch".to_string(), start: 6, end: 11 }));

    assert_eq!(Version::from_str("0.0.255.65535"), Ok(Version { year: 0, major: 0, minor: 255, patch: 65535 }));
    assert_eq!(Version::from_str("0.0.256.65536"), Err(InvalidPart { part: "minor".to_string(), start: 4, end: 7 }));

    assert_eq!(Version::from_str("0.255.255.65535"), Ok(Version { year: 0, major: 255, minor: 255, patch: 65535 }));
    assert_eq!(Version::from_str("0.256.256.65536"), Err(InvalidPart { part: "major".to_string(), start: 2, end: 5 }));

    assert_eq!(Version::from_str("2020.255.255.65535"), Ok(Version { year: 2020, major: 255, minor: 255, patch: 65535 }));
    assert_eq!(Version::from_str("65535.255.255.65535"), Ok(Version { year: 65535, major: 255, minor: 255, patch: 65535 }));
    assert_eq!(
        Version::from_str("4294967295.255.255.65535"),
        Ok(Version { year: 4294967295, major: 255, minor: 255, patch: 65535 })
    );
    assert_eq!(Version::from_str("4294967296.256.256.65536"), Err(InvalidPart { part: "year".to_string(), start: 0, end: 10 }));
}
