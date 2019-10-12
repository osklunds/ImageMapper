
use clap::{App, AppSettings, Arg, ArgMatches};

pub struct Settings {
    source_path: PathBuf,
    destination_path: PathBuf,
    verbose_print: bool,
    include_videos: bool,
    image_quality: ImageQuality
}

pub enum ImageQuality {
    Mobile,
    Television
}

/*

App::new("Rusty Self-Stabilizing Snapshots: Application")
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .about("The application code, that is an instance of a snapshot node.")
        .arg(node_id_argument())
        .arg(arguments::hosts_file(
            "The file with host ids, addresses and ports.",
        ))
        .arg(color_argument())
        .arg(write_argument())
        .arg(snapshot_argument())
        .arg(arguments::variant())
        .arg(arguments::print_client_operations())
        .arg(arguments::run_length())
        .arg(arguments::record_evaluation_info())
        .get_matches()
*/