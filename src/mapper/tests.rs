
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

/*
dir_path
    dir1
        small-with-exif.jpg
    dir2
        subdir1
            small-with-exif.jpg
        subdir2
        small-with-exif.jpg
    dir3
    small-with-exif.jpg
    small-without-exif.jpg
    small-without-exif.png
    text_file.txt
    word.docx
    video.m4v
*/
fn create_src_structure_in_dir(dir_path: &Path) {
    create_dir1_in_dir(dir_path);
    create_dir2_in_dir(dir_path);

    create_dir_with_name_in_dir("dir3", dir_path);
    create_small_image_with_exif_in_dir(dir_path);
    
    let small_without_exif_jpg_path = dir_path.join(SMALL_WITHOUT_EXIF_JPG_NAME);
    fs::copy(SMALL_WITHOUT_EXIF_JPG_PATH, small_without_exif_jpg_path).unwrap();

    let small_without_exif_png_path = dir_path.join(SMALL_WITHOUT_EXIF_PNG_NAME);
    fs::copy(SMALL_WITHOUT_EXIF_PNG_PATH, small_without_exif_png_path).unwrap();

    let text_file_path = dir_path.join("text_file.txt");
    File::create(text_file_path).unwrap();

    let word_path = dir_path.join("word.docx");
    File::create(word_path).unwrap();

    let movie_path = dir_path.join("video.m4v");
    File::create(movie_path).unwrap();
}

fn create_dir1_in_dir(dir_path: &Path) {
    let dir1_path = create_dir_with_name_in_dir("dir1", dir_path);
    create_small_image_with_exif_in_dir(&dir1_path);
}

fn create_dir_with_name_in_dir(name: &str, dir_path: &Path) -> PathBuf {
    let subdir_path = dir_path.join(name);
    fs::create_dir(&subdir_path).unwrap();
    subdir_path
}

fn create_small_image_with_exif_in_dir(dir_path: &Path) {
    let image_path = dir_path.join(SMALL_WITH_EXIF_JPG_NAME);
    fs::copy(SMALL_WITH_EXIF_JPG_PATH, image_path).unwrap();
}

fn create_dir2_in_dir(dir_path: &Path) {
    let dir2_path = create_dir_with_name_in_dir("dir2", dir_path);

    create_subdir1_in_dir(&dir2_path);
    create_subdir2_in_dir(&dir2_path);
    create_small_image_with_exif_in_dir(&dir2_path);
}

fn create_subdir1_in_dir(dir_path: &Path) {
    let subdir1_path = create_dir_with_name_in_dir("subdir1", dir_path);
    create_small_image_with_exif_in_dir(&subdir1_path);
}

fn create_subdir2_in_dir(dir_path: &Path) {
    create_dir_with_name_in_dir("subdir2", dir_path);
}

const SMALL_WITH_EXIF_JPG_NAME: &str = "small-with-exif.jpg";
const SMALL_WITH_EXIF_JPG_PATH: &str = "test_resources/small-with-exif.jpg";


const SMALL_WITHOUT_EXIF_JPG_NAME: &str = "small-without-exif.jpg";
const SMALL_WITHOUT_EXIF_JPG_PATH: &str = "test_resources/small-without-exif.jpg";

const SMALL_WITHOUT_EXIF_PNG_NAME: &str = "small-without-exif.png";
const SMALL_WITHOUT_EXIF_PNG_PATH: &str = "test_resources/small-without-exif.png";

#[test]
fn test_that_map_directory_creates_the_correct_dst_structure() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory(src_path, dst_path);

    check_dir1(dst_path);
    check_dir2(dst_path);

    assert!(dst_path.join("dir3").exists());
    assert!(dst_path.join(SMALL_WITH_EXIF_DST_NAME).exists());
    assert!(dst_path.join("small-without-exif.jpg.jpg").exists());
    assert!(dst_path.join("small-without-exif.png.jpg").exists());
}

fn check_dir1(dst_path: &Path) {
    assert!(dst_path.join("dir1").exists());
    assert!(dst_path.join("dir1").join(SMALL_WITH_EXIF_DST_NAME).exists());
}

const SMALL_WITH_EXIF_DST_NAME: &str = "   2010-03-14 11;22;33 small-with-exif.jpg.jpg";

fn check_dir2(dst_path: &Path) {
    assert!(dst_path.join("dir2").exists());

    assert!(dst_path.join("dir2").join("subdir1").exists());
    assert!(dst_path.join("dir2").join("subdir1").join(SMALL_WITH_EXIF_DST_NAME).exists());

    assert!(dst_path.join("dir2").join("subdir2").exists());

    assert!(dst_path.join("dir2").join(SMALL_WITH_EXIF_DST_NAME).exists());
}