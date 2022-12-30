pub mod change;
pub mod errors;
pub mod semver;

pub use change::SemVerChangeID;
pub use semver::SemVer;

pub fn max(semvers: &Vec<SemVer>) -> SemVer {
    let mut latest = semvers.get(0).unwrap();

    for semver in semvers {
        let gt = semver.gt(latest);

        if gt.is_ok() && gt.unwrap() {
            latest = &semver;
        }
    }

    return latest.clone();
}

pub fn min(semvers: &Vec<SemVer>) -> SemVer {
    let mut oldest = semvers.get(0).unwrap();

    for semver in semvers {
        let lt = semver.lt(oldest);

        if lt.is_ok() && lt.unwrap() {
            oldest = &semver;
        }
    }

    return oldest.clone();
}

pub fn range(semvers: Vec<SemVer>) -> (SemVer, SemVer) {
    let latest = max(&semvers);
    let oldest = min(&semvers);

    return (oldest, latest);
}

pub fn range_str(semvers: Vec<SemVer>) -> String {
    let latest = max(&semvers).to_string();
    let oldest = min(&semvers).to_string();

    return format!("{} - {}", oldest, latest);
}

pub fn deduplicate(semvers: Vec<SemVer>) -> Vec<SemVer> {
    let mut deduped: Vec<SemVer> = Vec::new();

    for semver in semvers {
        if !deduped.contains(&semver) {
            deduped.push(semver);
        }
    }

    return deduped;
}
