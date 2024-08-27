use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

use crate::mapper;
use crate::mapper::MapperError;
use crate::settings::{ImageQuality, Settings};

#[test]
fn test_ensure_path_is_directory_removes_file() {
    let temp_dir = tempdir();
    let destination_path = &temp_dir.path().join("dst");
    File::create(destination_path).unwrap();

    mapper::ensure_path_is_directory(destination_path);

    assert!(destination_path.is_dir());
}

#[test]
fn test_ensure_path_is_directory_adds_directory() {
    let temp_dir = tempdir();
    let destination_path = &temp_dir.path().join("dst");

    mapper::ensure_path_is_directory(destination_path);

    assert!(destination_path.is_dir());
}

#[test]
fn test_ensure_path_is_directory_does_not_remove_directory() {
    let temp_dir = tempdir();
    let destination_path = &temp_dir.path().join("dst");

    fs::create_dir(destination_path).unwrap();
    let destination_file = &destination_path.join("file");
    File::create(destination_file).unwrap();

    mapper::ensure_path_is_directory(destination_path);

    assert!(destination_file.exists());
}

#[test]
fn test_map_directory_correctly_fills_empty_dst_with_videos() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, true);

    check_that_dst_structure_is_correct(dst_path, true);
}

#[test]
fn test_map_directory_correctly_fills_empty_dst_without_videos() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, false);

    check_that_dst_structure_is_correct(dst_path, false);
}

#[test]
fn test_map_directory_dst_already_correct() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    let src_entries_before = get_dir_entries(src_path);
    let dst_entries_before = get_dir_entries(dst_path);

    map_directory_ok(src_path, dst_path, true);

    let src_entries_between = get_dir_entries(src_path);
    let dst_entries_between = get_dir_entries(dst_path);

    map_directory_ok(src_path, dst_path, true);

    check_that_dst_structure_is_correct(dst_path, true);

    let src_entries_after = get_dir_entries(src_path);
    let dst_entries_after = get_dir_entries(dst_path);

    assert_eq!(src_entries_before, src_entries_between);
    assert_eq!(src_entries_between, src_entries_after);

    assert_ne!(dst_entries_before, dst_entries_between);
    assert_eq!(dst_entries_between, dst_entries_after);
}

#[test]
fn test_map_directory_removes_unwanted_src_file() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    // Note that the file is removed even if it exists at top level,
    // since it also exists in source, but is of an unwatned type
    map_directory_ok(src_path, dst_path, true);
    File::create(dst_path.join("text_file.txt")).unwrap();
    map_directory_ok(src_path, dst_path, true);

    check_that_dst_structure_is_correct(dst_path, true);
}

#[test]
fn test_map_directory_removes_non_existant_src_file() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, true);
    File::create(dst_path.join("dir1").join("does not exist.txt")).unwrap();
    map_directory_ok(src_path, dst_path, true);

    check_that_dst_structure_is_correct(dst_path, true);
}

#[test]
fn test_map_directory_removes_non_existant_src_dir() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, true);
    fs::create_dir(dst_path.join("dir1").join("dir4")).unwrap();
    map_directory_ok(src_path, dst_path, true);

    check_that_dst_structure_is_correct(dst_path, true);
}

#[test]
fn test_map_directory_removes_non_existant_src_image_double_extension() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, true);
    File::create(dst_path.join("dir1").join("does not exist.jpg.jpg")).unwrap();
    map_directory_ok(src_path, dst_path, true);

    check_that_dst_structure_is_correct(dst_path, true);
}

#[test]
fn test_map_directory_removes_non_existant_src_image_single_extension() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, true);
    File::create(dst_path.join("dir1").join("does not exist.jpg")).unwrap();
    map_directory_ok(src_path, dst_path, true);

    check_that_dst_structure_is_correct(dst_path, true);
}

#[test]
fn test_map_directory_removes_non_existant_src_video() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, true);
    File::create(dst_path.join("dir1").join("does not exist.m4v")).unwrap();
    map_directory_ok(src_path, dst_path, true);

    check_that_dst_structure_is_correct(dst_path, true);
}

#[test]
fn test_map_directory_removes_existant_src_video_if_no_videos_desired() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, true);
    map_directory_ok(src_path, dst_path, false);

    check_that_dst_structure_is_correct(dst_path, false);
}

#[test]
fn test_map_directory_removes_non_existant_src_image_exif() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, true);
    File::create(
        dst_path
            .join("dir1")
            .join("   2001-01-01 11;22;33 does not exist.jpg.jpg"),
    )
    .unwrap();
    map_directory_ok(src_path, dst_path, true);

    check_that_dst_structure_is_correct(dst_path, true);
}

#[test]
fn test_map_directory_does_not_remove_correct_exif_image_in_dst() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, true);

    let file_path =
        &dst_path.join("   2010-03-14 11;22;33 small-with-exif.jpg.jpg");
    fs::remove_file(file_path).unwrap();
    fs::write(file_path, "some text").unwrap();

    map_directory_ok(src_path, dst_path, true);

    let recovered = fs::read_to_string(file_path).unwrap();
    assert_eq!(recovered, "some text");
}

#[test]
fn test_map_directory_does_not_remove_correct_image_in_dst() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, true);

    let file_path = &dst_path.join("small-without-exif.jpg.jpg");
    fs::remove_file(file_path).unwrap();
    fs::write(file_path, "some text").unwrap();

    map_directory_ok(src_path, dst_path, true);

    let recovered = fs::read_to_string(file_path).unwrap();
    assert_eq!(recovered, "some text");
}

#[test]
fn test_map_directory_does_not_remove_correct_video_in_dst() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, true);

    let file_path = &dst_path.join("video.m4v");
    fs::remove_file(file_path).unwrap();
    fs::write(file_path, "some text").unwrap();

    map_directory_ok(src_path, dst_path, true);

    let recovered = fs::read_to_string(file_path).unwrap();
    assert_eq!(recovered, "some text");
}

#[test]
fn test_map_directory_adds_missing_image_exif() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, true);

    let file_path =
        &dst_path.join("   2010-03-14 11;22;33 small-with-exif.jpg.jpg");
    fs::remove_file(file_path).unwrap();

    map_directory_ok(src_path, dst_path, true);

    check_that_dst_structure_is_correct(dst_path, true);
}

#[test]
fn test_map_directory_adds_missing_image() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, true);

    let file_path = &dst_path.join("small-without-exif.jpg.jpg");
    fs::remove_file(file_path).unwrap();

    map_directory_ok(src_path, dst_path, true);

    check_that_dst_structure_is_correct(dst_path, true);
}

#[test]
fn test_map_directory_adds_missing_video() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    create_src_structure_in_dir(src_path);

    map_directory_ok(src_path, dst_path, true);

    let file_path = &dst_path.join("video.m4v");
    fs::remove_file(file_path).unwrap();

    map_directory_ok(src_path, dst_path, true);

    check_that_dst_structure_is_correct(dst_path, true);
}

// So that the real image conversion is tested at least once
#[test]
fn test_map_directory_with_image_conversion() {
    for image_quality in [
        ImageQuality::Thumbnail,
        ImageQuality::Mobile,
        ImageQuality::Television,
    ] {
        let src_dir = tempdir();
        let src_path = &src_dir.path();
        let dst_dir = tempdir();
        let dst_path = &dst_dir.path();

        let image_path = src_path.join("small-with-exif.jpg");
        fs::copy("test_resources/small-with-exif.jpg", image_path).unwrap();

        let exp_src_entries = vec!["small-with-exif.jpg"];
        assert_dir_entries(&exp_src_entries, src_path);

        let settings = Settings {
            image_quality,
            verbose_print: false,
            include_videos: true,
        };

        mapper::map_directory(src_path, dst_path, settings).unwrap();

        // TODO: Check if they are actually images, so that conversion
        // didn't crash
        // Could have a print fun as opt
        let exp_dst_entries =
            vec!["   2010-03-14 11;22;33 small-with-exif.jpg.jpg"];
        assert_dir_entries(&exp_dst_entries, dst_path);
    }
}

#[test]
fn test_source_does_not_exist() {
    let src_dir = tempdir();
    let mut src_path = src_dir.path().to_path_buf();
    src_path.push("does_not_exist");

    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    assert!(dst_path.is_dir());

    let result = mapper::map_directory(&src_path, &dst_path, SETTINGS);

    assert_eq!(Err(MapperError::SrcDoesNotExist), result);
}

#[test]
fn test_source_is_not_a_dir() {
    let src_dir = tempdir();
    let mut src_path = src_dir.path().to_path_buf();
    src_path.push("file");
    fs::write(&src_path, b"content").unwrap();
    assert!(src_path.is_file());

    let dst_dir = tempdir();
    let dst_path = &dst_dir.path();
    assert!(dst_path.is_dir());

    let result = mapper::map_directory(&src_path, &dst_path, SETTINGS);

    assert_eq!(Err(MapperError::SrcDoesNotExist), result);
}

#[test]
fn test_destination_does_not_exist() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    assert!(src_path.is_dir());

    let dst_dir = tempdir();
    let mut dst_path = dst_dir.path().to_path_buf();
    dst_path.push("does_not_exist");

    let result = mapper::map_directory(&src_path, &dst_path, SETTINGS);

    assert_eq!(Err(MapperError::DstDoesNotExist), result);
}

#[test]
fn tst_destination_is_not_a_dir() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();
    assert!(src_path.is_dir());

    let dst_dir = tempdir();
    let mut dst_path = dst_dir.path().to_path_buf();
    dst_path.push("file");
    fs::write(&dst_path, b"content").unwrap();
    assert!(dst_path.is_file());

    let result = mapper::map_directory(&src_path, &dst_path, SETTINGS);

    assert_eq!(Err(MapperError::DstDoesNotExist), result);
}

#[test]
fn test_destination_dir_inside_source_dir() {
    let src_dir = tempdir();
    let src_path = &src_dir.path();

    let dst_path = src_path.join("subdir");
    fs::create_dir(&dst_path).unwrap();

    let result = mapper::map_directory(&src_path, &dst_path, SETTINGS);

    assert_eq!(Err(MapperError::DstInsideSrc), result);
}

#[test]
fn test_source_and_destination_dir_are_the_same() {
    let dir = tempdir();
    let path = &dir.path();

    let result = mapper::map_directory(&path, &path, SETTINGS);

    assert_eq!(Err(MapperError::SrcInsideDst), result);
}

#[test]
fn test_source_dir_inside_destination_dir() {
    let root_dir = tempdir();
    let root_path = root_dir.path();

    // So that canonicalize is tested
    let src_path = root_path.join("d").join("..").join("a").join("b");
    let dst_path = root_path.join("c").join("..").join("a");

    fs::create_dir_all(&src_path).unwrap();
    fs::create_dir_all(&dst_path).unwrap();

    let result = mapper::map_directory(&src_path, &dst_path, SETTINGS);

    assert_eq!(Err(MapperError::SrcInsideDst), result);
}

#[test]
fn test_destination_dir_has_a_file_source_dir_empty() {
    let src_dir = tempdir();
    let src_path = src_dir.path();
    let dst_dir = tempdir();
    let dst_path = dst_dir.path();

    let dst_file_path = dst_path.join("image.jpg");
    fs::write(&dst_file_path, b"content").unwrap();

    let src_entries = src_path.read_dir().unwrap().collect::<Vec<_>>();
    assert!(src_entries.is_empty());

    assert_eq!(
        Err(MapperError::DstTopLevelEntryNotInSrc(dst_file_path.clone())),
        mapper::map_directory(&src_path, &dst_path, SETTINGS),
    );
}

#[test]
fn test_destination_dir_has_a_dir_source_dir_empty() {
    let src_dir = tempdir();
    let src_path = src_dir.path();
    let dst_dir = tempdir();
    let dst_path = dst_dir.path();

    let dir_in_dst_path = dst_path.join("some_dir");
    fs::create_dir(&dir_in_dst_path).unwrap();

    let src_entries = src_path.read_dir().unwrap().collect::<Vec<_>>();
    assert!(src_entries.is_empty());

    assert_eq!(
        Err(MapperError::DstTopLevelEntryNotInSrc(
            dir_in_dst_path.clone()
        )),
        mapper::map_directory(&src_path, &dst_path, SETTINGS),
    );
}

#[test]
fn test_destination_dir_has_a_file_source_dir_does_not() {
    let src_dir = tempdir();
    let src_path = src_dir.path();
    let dst_dir = tempdir();
    let dst_path = dst_dir.path();

    create_src_structure_in_dir(src_path);
    map_directory_ok(&src_path, &dst_path, true);

    let file_only_in_dst = dst_path.join("some_file");
    fs::write(&file_only_in_dst, b"content").unwrap();

    assert_eq!(
        Err(MapperError::DstTopLevelEntryNotInSrc(
            file_only_in_dst.clone()
        )),
        mapper::map_directory(&src_path, &dst_path, SETTINGS),
    );
}

#[test]
fn test_destination_dir_has_a_dir_source_dir_does_not() {
    let src_dir = tempdir();
    let src_path = src_dir.path();
    let dst_dir = tempdir();
    let dst_path = dst_dir.path();

    create_src_structure_in_dir(src_path);
    map_directory_ok(&src_path, &dst_path, true);

    let dir_only_in_dst = dst_path.join("some_dir");
    fs::create_dir(&dir_only_in_dst).unwrap();

    assert_eq!(
        Err(MapperError::DstTopLevelEntryNotInSrc(
            dir_only_in_dst.clone()
        )),
        mapper::map_directory(&src_path, &dst_path, SETTINGS),
    );
}

#[test]
fn test_destination_dir_has_multiple_entries_source_dir_does_not() {
    let src_dir = tempdir();
    let src_path = src_dir.path();
    let dst_dir = tempdir();
    let dst_path = dst_dir.path();

    create_src_structure_in_dir(src_path);
    map_directory_ok(&src_path, &dst_path, true);

    let file_only_in_dst1 = dst_path.join("file1");
    fs::write(&file_only_in_dst1, b"content").unwrap();
    let file_only_in_dst2 = dst_path.join("file2");
    fs::write(&file_only_in_dst2, b"content").unwrap();

    assert_eq!(
        Err(MapperError::DstTopLevelEntryNotInSrc(
            file_only_in_dst1.clone()
        )),
        mapper::map_directory(&src_path, &dst_path, SETTINGS),
    );
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn create_src_structure_in_dir(path: &Path) {
    create_dir1_in_dir(path);
    create_dir2_in_dir(path);

    create_dir_with_name_in_dir("dir3", path);
    create_small_image_with_exif_in_dir(path, "small-with-exif.jpg");

    let small_without_exif_jpg_path = path.join("small-without-exif.jpg");
    fs::copy(
        "test_resources/small-without-exif.jpg",
        small_without_exif_jpg_path,
    )
    .unwrap();

    let small_without_exif_png_path = path.join("small-without-exif.png");
    fs::copy(
        "test_resources/small-without-exif.png",
        small_without_exif_png_path,
    )
    .unwrap();

    let text_file_path = path.join("text_file.txt");
    File::create(text_file_path).unwrap();

    let word_path = path.join("word.docx");
    File::create(word_path).unwrap();

    let movie_path = path.join("video.m4v");
    File::create(movie_path).unwrap();

    let exp_dir_entries = vec![
        "dir1",
        "dir1/small-with-exif.jpg",
        "dir2",
        "dir2/subdir1",
        "dir2/subdir1/small-with-exifåäöあ!@#$%^&*().jpg",
        "dir2/subdir2",
        "dir2/small-with-exif.jpg",
        "dir3",
        "small-with-exif.jpg",
        "small-without-exif.jpg",
        "small-without-exif.png",
        "text_file.txt",
        "video.m4v",
        "word.docx",
    ];
    assert_dir_entries(&exp_dir_entries, path);
}

fn create_dir1_in_dir(dir_path: &Path) {
    let dir1_path = create_dir_with_name_in_dir("dir1", dir_path);
    create_small_image_with_exif_in_dir(&dir1_path, "small-with-exif.jpg");
}

fn create_dir_with_name_in_dir(name: &str, dir_path: &Path) -> PathBuf {
    let subdir_path = dir_path.join(name);
    fs::create_dir(&subdir_path).unwrap();
    subdir_path
}

fn create_small_image_with_exif_in_dir(dir_path: &Path, image_name: &str) {
    let image_path = dir_path.join(image_name);
    fs::copy("test_resources/small-with-exif.jpg", image_path).unwrap();
}

fn create_dir2_in_dir(dir_path: &Path) {
    let dir2_path = create_dir_with_name_in_dir("dir2", dir_path);

    create_subdir1_in_dir(&dir2_path);
    create_subdir2_in_dir(&dir2_path);
    create_small_image_with_exif_in_dir(&dir2_path, "small-with-exif.jpg");
}

fn create_subdir1_in_dir(dir_path: &Path) {
    let subdir1_path = create_dir_with_name_in_dir("subdir1", dir_path);
    create_small_image_with_exif_in_dir(
        &subdir1_path,
        "small-with-exifåäöあ!@#$%^&*().jpg",
    );
}

fn create_subdir2_in_dir(dir_path: &Path) {
    create_dir_with_name_in_dir("subdir2", dir_path);
}

fn check_that_dst_structure_is_correct(dst_path: &Path, videos: bool) {
    let mut exp_dir_entries = vec![
        "   2010-03-14 11;22;33 small-with-exif.jpg.jpg",
        "dir1",
        "dir1/   2010-03-14 11;22;33 small-with-exif.jpg.jpg",
        "dir2",
        "dir2/subdir1",
        "dir2/subdir1/   2010-03-14 11;22;33 small-with-exifåäöあ!@#$%^&*().jpg.jpg",
        "dir2/subdir2",
        "dir2/   2010-03-14 11;22;33 small-with-exif.jpg.jpg",
        "dir3",
        "small-without-exif.jpg.jpg",
        "small-without-exif.png.jpg",
    ];

    if videos {
        exp_dir_entries.push("video.m4v");
    }

    assert_dir_entries(&exp_dir_entries, dst_path);
}

fn assert_dir_entries(exp_dir_entries: &[&str], path: &Path) {
    let dir_entries = get_dir_entries(path);

    for (exp_line, line) in std::iter::zip(exp_dir_entries, &dir_entries) {
        assert_eq!(
            exp_line, line,
            "exp:\n{:?} act:\n{:?}",
            exp_dir_entries, dir_entries
        );
    }

    // Check length for debuggability
    assert_eq!(exp_dir_entries.len(), dir_entries.len());

    // Then check all as an extra check if the above checks are buggy
    assert_eq!(exp_dir_entries, dir_entries);
}

fn get_dir_entries(path: &Path) -> Vec<String> {
    let result = Command::new("bash")
        .arg("-c")
        .arg("find *")
        .current_dir(path)
        .output()
        .expect("failed to execute process")
        .stdout;
    std::str::from_utf8(&result)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn tempdir() -> TempDir {
    tempfile::tempdir().unwrap()
}

const SETTINGS: Settings = Settings {
    image_quality: ImageQuality::Mobile,
    verbose_print: false,
    include_videos: true,
};

fn map_directory_ok(src_path: &Path, dst_path: &Path, include_videos: bool) {
    let settings = Settings {
        image_quality: ImageQuality::Mobile,
        verbose_print: false,
        include_videos,
    };
    mapper::map_directory_custom_opts(
        src_path,
        dst_path,
        settings,
        no_convert_image,
    )
    .unwrap();
}

pub fn no_convert_image(
    source_path: &Path,
    destination_path: &Path,
    _settings: &Settings,
) -> bool {
    fs::copy(source_path, destination_path).unwrap();
    true
}
