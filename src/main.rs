mod gnome;
mod install;
mod search;

use clap::{Parser, Subcommand};


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
    url: String
}


#[derive(Subcommand)]
enum Commands {
    /// Install an extension
    Install { url: String },

    /// Search extensions by keyword
    Search { keywords: String, shell: Option<i32> },
}

fn main() {
    let args = Args::parse();

    // TODO: validate that the url is well-formed

    match install::install(args.url.to_string()) {
        Ok(success) => { 
            println!("Got body: {:?}", success);
        },
        Err(error) => { panic!("Error: {}", error)},
    }
}
