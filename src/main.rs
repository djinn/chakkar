use clap::{Arg, Command};
use log::{info, error};
use chakkar::{Config, crush};

fn main() {
    // Initialize logger
    env_logger::init();

    // Parse command-line arguments using clap
    let matches = Command::new("Chakkar")
        .version("0.1")
        .about("Crushes PNG images with style")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_parser(clap::value_parser!(String))
            .required(true)
            .help("Input PNG file"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_parser(clap::value_parser!(String))
            .help("Output PNG file"))
        .arg(Arg::new("inplace")
            .short('p')
            .long("inplace")
            .action(clap::ArgAction::SetTrue)
            .help("Overwrite input file with compressed version"))
        .arg(Arg::new("verbose")
            .short('v')
            .action(clap::ArgAction::Count)
            .help("Increase verbosity"))
        .arg(Arg::new("chunk_size")
            .short('c')
            .long("chunk_size")
            .value_parser(clap::value_parser!(usize))
            .default_value("8")
            .help("Set chunk size for processing"))
        .get_matches();

    // Build configuration from arguments
    let config = Config::from_matches(&matches);

    // Run the main processing function
    if let Err(e) = crush(&config) {
        error!("Error: {}", e);
        std::process::exit(1);
    }

    info!("Successfully processed image.");
}
