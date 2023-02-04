mod gnome;
mod install;
mod search;

use std::process::ExitCode;

use clap::{Parser, Subcommand};
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

#[derive(Subcommand)]
enum Commands {
    /// Install an extension
    Install { url: String },

    /// Search extensions by keyword
    Search {
        keywords: String,
        shell: Option<i32>,
    },
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
            return ExitCode::SUCCESS;
        }
        Err(_error) => return ExitCode::FAILURE,
    };
}
