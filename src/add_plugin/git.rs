use git2::{Direction, Repository};
use tempfile;

pub(crate) enum PackageVersion {
    Commit(String),
}

pub(crate) fn get_package_latest_version(s: &str) -> PackageVersion  {
    let temp_dir = tempfile::tempdir().unwrap();
    let repo = Repository::init_bare(temp_dir.path()).unwrap();
    let remote_url = format!("https://github.com/{}", s);
    let mut remote = repo.remote_anonymous(&remote_url).unwrap();
    remote.connect(Direction::Fetch).unwrap();
    let references = remote.list().unwrap();
    
    if let Some(head_ref) = references.iter().find(|r| r.name() == "HEAD") {
        PackageVersion::Commit(head_ref.oid().to_string())
    } else {
        panic!("Couldn't get the latest version of the package {s}")
    }
}
