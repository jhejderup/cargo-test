extern crate semver;


use semver::Version;
use semver::VersionReq;


fn main() {
    let r = VersionReq::parse(">= 2.0.0").unwrap();
    let v = Version::parse("1.0.0").unwrap();

    let r1 = VersionReq::parse(">= 1.0.0").unwrap();
    let v1 = Version::parse("1.0.0").unwrap();
    
    
    assert!(r.to_string() == ">= 2.0.0".to_string());
    assert!(r.matches(&v));
    assert!(r1.to_string() == ">= 1.0.0".to_string());
    assert!(r1.matches(&v1));
    
}




