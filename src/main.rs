use std::process::exit;

use clap::Parser;
mod status_codes;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "404answernotfound @ Schrodinger Hat")]
struct Cli {
    #[clap(short, long)]
    status: i16,

    #[clap(short, long)]
    description: bool,

    #[clap(short, long)]
    open: bool,
}

fn main() {
    let args = Cli::parse();
    let status = args.status;

    // Check that status code is valid
    if !status_codes::StatusCodes::is_valid(status) {
        println!("Status code is invalid");
        exit(1);
    }

    // Print description
    match args.description {
        true => println!(
            "Status Code Description: {}",
            status_codes::StatusCodes::get_description(status)
        ),
        _ => (),
    }

    match args.open {
        true => {
            let url = format!("https://http.cat/{}", status);
            match webbrowser::open(&url) {
                Ok(_) => println!("Opened {} in browser", url),
                Err(_) => println!("Failed to open {} in browser", url),
            }
        }
        _ => exit(0),
    }
}
