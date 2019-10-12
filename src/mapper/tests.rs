
use super::*;

#[test]
fn test_ensure_path_is_directory_removes_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let destination_path = &temp_dir.path().join("dst");
    File::create(destination_path).unwrap();

    ensure_path_is_directory(destination_path);

    assert!(destination_path.is_dir());
}

#[test]
fn test_ensure_path_is_directory_adds_directory() {
    let temp_dir = tempfile::tempdir().unwrap();
    let destination_path = &temp_dir.path().join("dst");
    
    ensure_path_is_directory(destination_path);

    assert!(destination_path.is_dir());
}

#[test]
fn test_ensure_path_is_directory_does_not_remove_directory() {
    let temp_dir = tempfile::tempdir().unwrap();
    let destination_path = &temp_dir.path().join("dst");
    
    fs::create_dir(destination_path).unwrap();
    let destination_file = &destination_path.join("file");
    File::create(destination_file).unwrap();

    ensure_path_is_directory(destination_path);

    assert!(destination_file.exists());        
}
