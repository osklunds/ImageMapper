use super::*;

use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

use crate::settings::ImageQuality;

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

    let small_without_exif_jpg_path =
        dir_path.join(SMALL_WITHOUT_EXIF_JPG_NAME);
    fs::copy(SMALL_WITHOUT_EXIF_JPG_PATH, small_without_exif_jpg_path).unwrap();

    let small_without_exif_png_path =
        dir_path.join(SMALL_WITHOUT_EXIF_PNG_NAME);
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
const SMALL_WITHOUT_EXIF_JPG_PATH: &str =
    "test_resources/small-without-exif.jpg";

const SMALL_WITHOUT_EXIF_PNG_NAME: &str = "small-without-exif.png";
const SMALL_WITHOUT_EXIF_PNG_PATH: &str =
    "test_resources/small-without-exif.png";

const SETTINGS: MapperSettings = MapperSettings {
    app_settings: Settings {
        image_quality: ImageQuality::Mobile,
        verbose_print: false,
        include_videos: true,
    },
    open_compress_and_save_image: no_convert_image
};

const SETTINGS_NO_VIDEO: MapperSettings = MapperSettings {
    app_settings: Settings {
        image_quality: ImageQuality::Mobile,
        verbose_print: false,
        include_videos: false,
    },
    open_compress_and_save_image: no_convert_image
};

pub fn no_convert_image(
    source_path: &Path,
    destination_path: &Path,
    _settings: &Settings,
) -> bool {
    unwrap!(
        fs::copy(source_path, destination_path),
        "Could not copy from \"{}\" to \"{}\"",
        source_path.display(),
        destination_path.display()
    );
    true
}

#[test]
fn test_map_directory_correctly_fills_empty_dst_with_videos() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);

    check_that_dst_structure_is_correct_if_videos(dst_path, true);
}

fn check_that_dst_structure_is_correct_if_videos(
    dst_path: &Path,
    videos: bool,
) {
    let mut exp_dir_items = vec![
        "   2010-03-14 11;22;33 small-with-exif.jpg.jpg",
        "dir1",
        "dir1/   2010-03-14 11;22;33 small-with-exif.jpg.jpg",
        "dir2",
        "dir2/subdir1",
        "dir2/subdir1/   2010-03-14 11;22;33 small-with-exif.jpg.jpg",
        "dir2/subdir2",
        "dir2/   2010-03-14 11;22;33 small-with-exif.jpg.jpg",
        "dir3",
        "small-without-exif.jpg.jpg",
        "small-without-exif.png.jpg",
    ];

    if videos {
        exp_dir_items.push("video.m4v");
    }

    assert_dir_items(&exp_dir_items, dst_path);
}

fn assert_dir_items(exp_dir_items: &[&str], path: &Path) {
    let dir_items = get_dir_items(path);
    let dir_items: Vec<&str> = dir_items.lines().collect();
    
    for (exp_line, line) in std::iter::zip(exp_dir_items, &dir_items) {
        assert_eq!(exp_line, line);
    }

    // Check length for debuggability
    assert_eq!(exp_dir_items.len(), dir_items.len());

    // Then check all as an extra check if the above checks are buggy
    assert_eq!(exp_dir_items, dir_items);
}

fn get_dir_items(path: &Path) -> String {
    let result = Command::new("bash")
        .arg("-c")
        .arg("find *")
        .current_dir(path)
        .output()
        .expect("failed to execute process")
        .stdout;
    std::str::from_utf8(&result).unwrap().to_string()
}

fn check_dir1(dst_path: &Path) {
    let dir1_path = &dst_path.join("dir1");
    assert!(dir1_path.exists());

    assert!(dir1_path.join(SMALL_WITH_EXIF_DST_NAME).exists());

    assert_eq!(fs::read_dir(dir1_path).unwrap().count(), 1);
}

const SMALL_WITH_EXIF_DST_NAME: &str =
    "   2010-03-14 11;22;33 small-with-exif.jpg.jpg";

fn check_dir2(dst_path: &Path) {
    let dir2_path = dst_path.join("dir2");
    assert!(dir2_path.exists());

    check_subdir1(&dir2_path);
    check_subdir2(&dir2_path);
    assert!(dst_path
        .join("dir2")
        .join(SMALL_WITH_EXIF_DST_NAME)
        .exists());

    assert_eq!(fs::read_dir(dir2_path).unwrap().count(), 3);
}

fn check_subdir1(dir2_path: &Path) {
    let subdir1_path = dir2_path.join("subdir1");

    assert!(subdir1_path.exists());
    assert!(subdir1_path.join(SMALL_WITH_EXIF_DST_NAME).exists());

    assert_eq!(fs::read_dir(subdir1_path).unwrap().count(), 1);
}

fn check_subdir2(dir2_path: &Path) {
    let subdir2_path = dir2_path.join("subdir2");
    assert!(subdir2_path.exists());

    assert_eq!(fs::read_dir(subdir2_path).unwrap().count(), 0);
}

fn check_dir3(dst_path: &Path) {
    let dir3_path = dst_path.join("dir3");
    assert!(dir3_path.exists());

    assert_eq!(fs::read_dir(dir3_path).unwrap().count(), 0);
}

#[test]
fn test_map_directory_correctly_fills_empty_dst_without_videos() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS_NO_VIDEO);

    check_that_dst_structure_is_correct_if_videos(dst_path, false);
}

#[test]
fn test_map_directory_removes_unwanted_src_file() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);
    File::create(dst_path.join("text_file.txt")).unwrap();
    map_directory_int(src_path, dst_path, &SETTINGS);

    check_that_dst_structure_is_correct_if_videos(dst_path, true);
}

#[test]
fn test_map_directory_removes_non_existant_src_file() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);
    File::create(dst_path.join("does not exist.txt")).unwrap();
    map_directory_int(src_path, dst_path, &SETTINGS);

    check_that_dst_structure_is_correct_if_videos(dst_path, true);
}

#[test]
fn test_map_directory_removes_non_existant_src_dir() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);
    fs::create_dir(dst_path.join("dir4")).unwrap();
    map_directory_int(src_path, dst_path, &SETTINGS);

    check_that_dst_structure_is_correct_if_videos(dst_path, true);
}

#[test]
fn test_map_directory_removes_non_existant_src_image_double_extension() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);
    File::create(dst_path.join("does not exist.jpg.jpg")).unwrap();
    map_directory_int(src_path, dst_path, &SETTINGS);

    check_that_dst_structure_is_correct_if_videos(dst_path, true);
}

#[test]
fn test_map_directory_removes_non_existant_src_image_single_extension() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);
    File::create(dst_path.join("does not exist.jpg")).unwrap();
    map_directory_int(src_path, dst_path, &SETTINGS);

    check_that_dst_structure_is_correct_if_videos(dst_path, true);
}

#[test]
fn test_map_directory_removes_non_existant_src_video() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);
    File::create(dst_path.join("does not exist.m4v")).unwrap();
    map_directory_int(src_path, dst_path, &SETTINGS);

    check_that_dst_structure_is_correct_if_videos(dst_path, true);
}

#[test]
fn test_map_directory_removes_existant_src_video_if_no_videos_desired() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);
    map_directory_int(src_path, dst_path, &SETTINGS_NO_VIDEO);

    check_that_dst_structure_is_correct_if_videos(dst_path, false);
}

#[test]
fn test_map_directory_removes_non_existant_src_image_exif() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);
    File::create(
        dst_path.join("   2001-01-01 11;22;33 does not exist.jpg.jpg"),
    )
    .unwrap();
    map_directory_int(src_path, dst_path, &SETTINGS);

    check_that_dst_structure_is_correct_if_videos(dst_path, true);
}

#[test]
fn test_map_directory_does_not_remove_correct_exif_image_in_dst() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);

    let file_path = &dst_path.join(SMALL_WITH_EXIF_DST_NAME);
    fs::remove_file(file_path).unwrap();
    fs::write(file_path, "some text").unwrap();

    map_directory_int(src_path, dst_path, &SETTINGS);

    let recovered = fs::read_to_string(file_path).unwrap();
    assert_eq!(recovered, "some text");
}

#[test]
fn test_map_directory_does_not_remove_correct_image_in_dst() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);

    let file_path = &dst_path.join("small-without-exif.jpg.jpg");
    fs::remove_file(file_path).unwrap();
    fs::write(file_path, "some text").unwrap();

    map_directory_int(src_path, dst_path, &SETTINGS);

    let recovered = fs::read_to_string(file_path).unwrap();
    assert_eq!(recovered, "some text");
}

#[test]
fn test_map_directory_does_not_remove_correct_video_in_dst() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);

    let file_path = &dst_path.join("video.m4v");
    fs::remove_file(file_path).unwrap();
    fs::write(file_path, "some text").unwrap();

    map_directory_int(src_path, dst_path, &SETTINGS);

    let recovered = fs::read_to_string(file_path).unwrap();
    assert_eq!(recovered, "some text");
}

#[test]
fn test_map_directory_adds_missing_image_exif() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);

    let file_path = &dst_path.join(SMALL_WITH_EXIF_DST_NAME);
    fs::remove_file(file_path).unwrap();

    map_directory_int(src_path, dst_path, &SETTINGS);

    check_that_dst_structure_is_correct_if_videos(dst_path, true);
}

#[test]
fn test_map_directory_adds_missing_image() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);

    let file_path = &dst_path.join("small-without-exif.jpg.jpg");
    fs::remove_file(file_path).unwrap();

    map_directory_int(src_path, dst_path, &SETTINGS);

    check_that_dst_structure_is_correct_if_videos(dst_path, true);
}

#[test]
fn test_map_directory_adds_missing_video() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_int(src_path, dst_path, &SETTINGS);

    let file_path = &dst_path.join("video.m4v");
    fs::remove_file(file_path).unwrap();

    map_directory_int(src_path, dst_path, &SETTINGS);

    check_that_dst_structure_is_correct_if_videos(dst_path, true);
}

// So that the real image conversion is tested at least once
#[test]
fn test_map_directory_with_image_conversion() {
    let src_dir = tempfile::tempdir().unwrap();
    let src_path = &src_dir.path();
    let dst_dir = tempfile::tempdir().unwrap();
    let dst_path = &dst_dir.path();

    create_src_structure_in_dir(src_path);

    let settings = Settings {
        image_quality: ImageQuality::Mobile,
        verbose_print: false,
        include_videos: true,
    };

    map_directory(src_path, dst_path, settings);

    check_that_dst_structure_is_correct_if_videos(dst_path, true);
}
