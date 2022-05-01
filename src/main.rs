use clap::{Arg, Command}; // Command line
use std::error::Error;

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

// Document handling
mod epub;
mod mobi;
mod pdf;
mod rename_file;
mod utils;

// Useful stuff

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = Command::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .long_about("This program will do something.")
        .arg(
            Arg::new("read")
                .value_name("filename(S)")
                .help("One or more filename(s) to process. Wildcards and multiple_occurrences filenames (e.g. 2019*.pdf 2020*.pdf) are supported.")
                .takes_value(true)
                .multiple_occurrences(true),
        )
        .arg( // Hidden debug parameter
            Arg::new("debug")
                .short('d')
                .long("debug")
                .multiple_occurrences(true)
                .help("Output debug information as we go. Supply it twice for trace-level logs.")
                .takes_value(false)
                .hide(true),
        )
        .arg( // Don't print any information
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .multiple_occurrences(false)
                .help("Don't produce any output except errors while working.")
                .takes_value(false)
        )
        .arg( // Print summary information
            Arg::new("print-summary")
                .short('p')
                .long("print-summary")
                .multiple_occurrences(false)
                .help("Print summary detail for each session processed.")
                .takes_value(false)
        )
        .arg( // Don't export detail information
            Arg::new("detail-off")
                .short('o')
                .long("detail-off")
                .multiple_occurrences(false)
                .help("Don't export detailed information about each filename processed.")
                .takes_value(false)
        )
        .arg( // Don't export detail information
            Arg::new("dry-run")
                .short('r')
                .long("dry-run")
                .multiple_occurrences(false)
                .help("Performs a dry-run without executing any actual changes.")
                .takes_value(false)
        )
        .arg( // Rename filenames
            Arg::new("rename")
                .short('n')
                .long("rename-filename")
                .multiple_occurrences(false)
                .help("Rename filenames based on the provided pattern as they are processed.")
                .takes_value(true)
        )
        .get_matches();

    // create a log builder
    let mut logbuilder = Builder::new();

    // Figure out what log level to use.
    if cli_args.is_present("quiet") {
        logbuilder.filter_level(LevelFilter::Off);
    } else {
        match cli_args.occurrences_of("debug") {
            0 => logbuilder.filter_level(LevelFilter::Info),
            1 => logbuilder.filter_level(LevelFilter::Debug),
            _ => logbuilder.filter_level(LevelFilter::Trace),
        };
    }

    // Initialize logging
    logbuilder.target(Target::Stdout).init();

    // Initialize variables
    let mut tags;
    let dry_run = cli_args.is_present("dry-run");

    // Do the work
    for filename in cli_args.values_of("read").unwrap() {
        log::debug!("Processing filename {}", filename);
        let ext = utils::get_extension(filename);

        tags = match ext.as_ref() {
            "pdf" => {
                log::info!("Processing PDF filename {}", filename);
                let pdf_m = pdf::get_metadata(filename);
                if let Ok(pdf_d) = pdf_m {
                    pdf_d
                } else {
                    log::error!("Error processing PDF filename {}", filename);
                    continue;
                }
            }
            "epub" => {
                log::info!("Processing EPUB filename {}", filename);
                epub::get_metadata(filename)?
            }
            "mobi" => {
                log::info!("Processing MOBI filename {}", filename);
                mobi::get_metadata(filename)?
            }
            _ => {
                log::warn!("Unknown filename {}", filename);
                crate::utils::new_hashmap()
            }
        };

        tags.insert(
            "Year".to_string(),
            utils::get_year(tags.get("Date").unwrap_or(&"".to_string())),
        );

        if !cli_args.is_present("detail-off") && !cli_args.is_present("quiet") {
            utils::print_metadata(&tags);
        }

        if cli_args.is_present("rename") {
            let pattern = cli_args.value_of("rename").unwrap_or("");
            let res = rename_file::rename_file(filename, &tags, pattern, dry_run)?;
            if !cli_args.is_present("quiet") {
                log::info!("{} --> {}", filename, res);
            }
        }
    }

    // Everything is a-okay in the end
    Ok(())
} // fn run()

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// The actual executable function that gets called when the program in invoked.
fn main() {
    std::process::exit(match run() {
        Ok(_) => 0, // everying is hunky dory - exit with code 0 (success)
        Err(err) => {
            log::error!("{}", err.to_string().replace('\"', ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
