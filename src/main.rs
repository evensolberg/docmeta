use clap::parser::ValueSource;
use std::error::Error;

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

// Document handling
mod cli;
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
    let cli_args = cli::build().get_matches();

    // create a log builder
    let mut logbuilder = Builder::new();

    // Get the command line arguments
    let dry_run = cli_args.get_flag("dry-run");
    let quiet = cli_args.get_flag("quiet");
    let detail_off = cli_args.get_flag("detail-off");
    let rename_present = cli_args.value_source("rename-pattern") == Some(ValueSource::CommandLine);

    // Figure out what log level to use.
    if quiet {
        logbuilder.filter_level(LevelFilter::Off);
    } else {
        match cli_args.get_count("debug") {
            0 => logbuilder.filter_level(LevelFilter::Info),
            1 => logbuilder.filter_level(LevelFilter::Debug),
            _ => logbuilder.filter_level(LevelFilter::Trace),
        };
    }

    // Initialize logging
    logbuilder.target(Target::Stdout).init();

    // Initialize variables
    let mut tags;

    // Do the work
    for filename in cli_args.get_many::<String>("read").unwrap_or_default() {
        log::debug!("Processing filename {filename}");
        let ext = utils::get_extension(filename);

        tags = match ext.as_ref() {
            "pdf" => {
                log::info!("Processing PDF: {filename}");
                let pdf_m = pdf::get_metadata(filename);
                match pdf_m {
                    Ok(pdf_d) => pdf_d,
                    Err(e) => {
                        log::error!("Error processing PDF: {filename}. Error: {e}");
                        return Err(e);
                    }
                }
            }
            "epub" => {
                log::info!("Processing EPUB: {filename}");
                epub::get_metadata(filename)?
            }
            "mobi" => {
                log::info!("Processing MOBI: {filename}");
                mobi::get_metadata(filename)?
            }
            _ => {
                log::warn!("Unknown file type: {filename}");
                crate::utils::new_hashmap()
            }
        };

        log::debug!("tags: {tags:?}");
        if tags.contains_key("Date") {
            tags.insert(
                "Year".to_string(),
                utils::get_year(tags.get("Date").unwrap_or(&String::new())),
            );
        }

        if !detail_off && !quiet {
            utils::print_metadata(&tags);
        }

        if rename_present {
            let empty_str = String::new();
            let pattern = cli_args
                .get_one::<String>("rename-pattern")
                .unwrap_or(&empty_str);
            let res = rename_file::rename_file(filename, &tags, pattern, dry_run)?;
            if !quiet {
                log::info!("{filename} --> {res}");
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
        Ok(()) => 0, // everying is hunky dory - exit with code 0 (success)
        Err(err) => {
            log::error!("{}", err.to_string().replace('\"', ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
