//! Contains a single function to build the CLI
use clap::{Arg, ArgAction, Command};

/// Builds the CLI so the main file doesn't get cluttered.
pub fn build() -> Command {
    Command::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        // .author(clap::crate_authors!("\n"))
        .long_about("This program display eBook metadata and rename files based on this metadata.")
        .arg(
            Arg::new("read")
                .value_name("filename(s)")
                .help("One or more filename(s) to process. Wildcards and multiple_occurrences filenames (e.g. 2019*.pdf 2020*.pdf) are supported.")
                .num_args(1..)
                .required(true)
                .action(ArgAction::Append),
        )
        .arg( // Hidden debug parameter
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Output debug information as we go. Supply it twice for trace-level logs.")
                .hide(true)
                .num_args(0)
                .action(ArgAction::Count)
        )
        .arg( // Don't print any information
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Don't produce any output except errors while working.")
                .num_args(0)
                .action(ArgAction::SetTrue)
        )
        .arg( // Don't export detail information
            Arg::new("detail-off")
                .short('o')
                .long("detail-off")
                .help("Don't print detailed information about each filename processed (handy when renaming files).")
                .num_args(0)
                .action(ArgAction::SetTrue)
        )
        .arg( // Don't export detail information
            Arg::new("dry-run")
                .short('r')
                .long("dry-run")
                .help("Performs a dry-run without executing any actual changes.")
                .num_args(0)
                .action(ArgAction::SetTrue)
        )
        .arg( // Rename filenames
            Arg::new("rename-pattern")
                .short('n')
                .long("rename-file")
                .help("Change filenames based on the provided pattern as they are processed.")
                .num_args(1)
                .action(ArgAction::Set)
                .required(false)
                .hide(false),
        )
}
