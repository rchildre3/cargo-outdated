use semver::Version;

/// Enum which represents the update status of a package
#[derive(Debug)]
pub enum Status {
    Unchanged,
    Removed,
    Version(Version),
}

impl Status {
    pub fn from_versions(from: &Version, to: Option<&Version>) -> Status {
        if let Some(to) = to {
            if from == to {
                Status::Unchanged
            } else {
                Status::Version(to.clone())
            }
        } else {
            Status::Removed
        }
    }

    pub fn is_changed(&self) -> bool { !matches!(*self, Status::Unchanged) }
}

impl ::std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Status::Unchanged => write!(f, "---"),
            Status::Removed => write!(f, "Removed"),
            Status::Version(ref v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug)]
pub struct PkgStatus {
    pub compat: Status,
    pub latest: Status,
}
