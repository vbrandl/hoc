use hoc::count::count_repositories;

use tempfile::TempDir;

#[test]
fn no_repos() {
    let repos = TempDir::new().unwrap();
    assert_eq!(0, count_repositories(&repos).unwrap());
}

#[test]
fn no_repos_for_provider() {
    let repos = TempDir::new().unwrap();
    let _provider = TempDir::new_in(&repos).unwrap();
    assert_eq!(0, count_repositories(&repos).unwrap());
}

#[test]
fn no_repos_for_owner() {
    let repos = TempDir::new().unwrap();
    let provider = TempDir::new_in(&repos).unwrap();
    let _owner = TempDir::new_in(&provider).unwrap();
    assert_eq!(0, count_repositories(&repos).unwrap());
}

#[test]
fn one_repo_for_owner() {
    let repos = TempDir::new().unwrap();
    let provider = TempDir::new_in(&repos).unwrap();
    let owner = TempDir::new_in(&provider).unwrap();
    let _repo = TempDir::new_in(&owner).unwrap();
    assert_eq!(1, count_repositories(&repos).unwrap());
}

#[test]
fn two_repos_for_owner() {
    let repos = TempDir::new().unwrap();
    let provider = TempDir::new_in(&repos).unwrap();
    let owner = TempDir::new_in(&provider).unwrap();
    let _repo1 = TempDir::new_in(&owner).unwrap();
    let _repo2 = TempDir::new_in(&owner).unwrap();
    assert_eq!(2, count_repositories(&repos).unwrap());
}

#[test]
fn two_repos_for_two_providers() {
    let repos = TempDir::new().unwrap();
    let provider1 = TempDir::new_in(&repos).unwrap();
    let owner1 = TempDir::new_in(&provider1).unwrap();
    let _repo1 = TempDir::new_in(&owner1).unwrap();
    let provider2 = TempDir::new_in(&repos).unwrap();
    let owner2 = TempDir::new_in(&provider2).unwrap();
    let _repo2 = TempDir::new_in(&owner2).unwrap();
    assert_eq!(2, count_repositories(&repos).unwrap());
}

#[test]
fn two_subdirs_in_one_repo() {
    let repos = TempDir::new().unwrap();
    let provider = TempDir::new_in(&repos).unwrap();
    let owner = TempDir::new_in(&provider).unwrap();
    let repo = TempDir::new_in(&owner).unwrap();
    let _subdir1 = TempDir::new_in(&repo).unwrap();
    let _subdir2 = TempDir::new_in(&repo).unwrap();
    assert_eq!(1, count_repositories(&repos).unwrap());
}
