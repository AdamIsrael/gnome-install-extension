mod gnome;
mod install;
mod search;

use std::process::ExitCode;

use clap::{Parser};
use url::Url;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Turn off all output
    #[arg(short = 'q', long)]
    quiet: bool,

    /// Be more verbose
    #[arg(short = 'v', long)]
    verbose: bool,

    /// The URL to the extension on extensions.gnome.org
    url: String,
}

fn main() -> ExitCode {
    let args = Args::parse();

    // TODO: validate that the url is well-formed
    let url = match Url::parse(args.url.as_str()) {
        Ok(url) => url,
        Err(_e) => {
            if !args.quiet {
                println!("Unable to parse URL.");
            }
            return ExitCode::FAILURE;
        }
    };

    match install::install(url.to_string()) {
        Ok(_uuid) => {
            if !args.quiet {
                println!("Please restart your gnome-session.");
            }
            ExitCode::SUCCESS
        }
        Err(_error) => ExitCode::FAILURE
    }
}
