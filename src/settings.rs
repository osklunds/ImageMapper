
use std::path::PathBuf;

use clap::{App, AppSettings, Arg, ArgMatches};

pub struct Settings {
    pub image_quality: ImageQuality,
    pub verbose_print: bool,
    pub include_videos: bool,
}

pub enum ImageQuality {
    Mobile,
    Television,
}

impl Settings {
    pub fn new_from_matches(matches: &ArgMatches) -> Settings {
        let image_quality = matches.value_of("image quality").unwrap();
        let image_quality = match image_quality {
            "Mobile" => ImageQuality::Mobile,
            "TV"     => ImageQuality::Television,
            _        => panic!("Unknown image quality selected."),
        };

        let verbose_print = matches.is_present("verbose");
        let include_videos = matches.is_present("include-videos");

        Settings {
            image_quality: image_quality,
            verbose_print: verbose_print,
            include_videos: include_videos
        }
    }
}

pub fn get_matches<'a>() -> ArgMatches<'a> {
        App::new("ImageMapper")
        .setting(AppSettings::DisableVersion)
        .about("Maps the source directory structure to an equivalent structure in the destination directory. The differences are: 1. Images will be downscaled and compressed. 2. Images will get their exif date/time prepended to their file names. 3. Images (and optionally videos) are the only files that will be kept.")
        .max_term_width(90)
        .arg(source_path_argument())
        .arg(destination_path_argument())
        .arg(image_quality_argument())
        .arg(verbose_print_argument())
        .arg(include_videos_argument())
        .get_matches()
    }

fn source_path_argument<'a>() -> Arg<'a, 'a> {
    Arg::with_name("source directory")
        .required(true)
        .takes_value(true)
        .help("The path to the directory that will be mapped.")
}

pub fn source_path_from_matches(matches: &ArgMatches) -> PathBuf {
    let source_path = matches.value_of("source directory").unwrap();
    PathBuf::from(source_path)
}

fn destination_path_argument<'a>() -> Arg<'a, 'a> {
    Arg::with_name("destination directory")
        .required(true)
        .takes_value(true)
        .help("The path to the directory where the result of the mapping will be placed.")
}

pub fn destination_path_from_matches(matches: &ArgMatches) -> PathBuf {
    let destination_path = matches.value_of("destination directory").unwrap();
    PathBuf::from(destination_path)
}

fn image_quality_argument<'a>() -> Arg<'a, 'a> {
    Arg::with_name("image_quality")
        .required(true)
        .takes_value(true)
        .possible_values(&["Mobile", "TV"])
        .help("Select if the images should be converted to the mobile quality (1024x1024, x % compression) or the TV quality (1920x1080, y % compression)")
}

fn verbose_print_argument<'a>() -> Arg<'a, 'a> {
    Arg::with_name("verbose")
        .short("v")
        .long("verbose")
        .takes_value(false)
        .help("Print when a directory is enterted and when a file is added/delted. No matter of this setting, errors will always be printed.")
}

fn include_videos_argument<'a>() -> Arg<'a, 'a> {
    Arg::with_name("video")
        .short("i")
        .long("include-videos")
        .takes_value(false)
        .help("Instead of just images, with this option, videos will also be included in the destination. Note that they will just be copied as-is without any conversion.")
}
