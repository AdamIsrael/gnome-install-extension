mod gnome;
mod install;
mod search;

use std::process::ExitCode;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Turn off all output
    #[arg(short = 'q', long)]
    quiet: bool,

    /// Be more verbose
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Do everything except install the extension
    #[arg(long)]
    dry_run: bool,

    /// The URL, UUID, or keyword(s) to the extension on extensions.gnome.org
    search: Vec<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    // // There's no reason why we can't install by url, uuid, or keyword(s)
    // let url = match Url::parse(args.args.as_str()) {
    //     Ok(url) => url,
    //     Err(_e) => {
    //         if !args.quiet {
    //             println!("Unable to parse URL.");
    //         }
    //         return ExitCode::FAILURE;
    //     }
    // };

    // Require a search argument
    if args.search.len() == 0 {
        use clap::CommandFactory;
        let mut cmd = Args::command();
        cmd.print_help().unwrap();
        return ExitCode::FAILURE;
    }

    match install::install(&args) {
        Ok(_uuid) => {
            if !args.quiet && !args.dry_run {
                println!("Please restart your gnome-session.");
            }
            ExitCode::SUCCESS
        }
        Err(_error) => ExitCode::FAILURE,
    }
}
