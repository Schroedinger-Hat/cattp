use std::process::exit;

use clap::Parser;
mod status_codes;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "404answernotfound @ Schrodinger Hat")]
struct Cli {
    #[clap(required = true)]
    status: i16,

    #[clap(short, long)]
    description: bool,

    #[clap(short, long)]
    no_open: bool,

    #[clap(short, long)]
    explain: bool,
}

fn print_status_code_description(status: i16) {
    println!(
        "Status Code Description: {}",
        status_codes::StatusCodes::get_description(status)
    );
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
        true => print_status_code_description(status),
        _ => (),
    }

    match args.no_open {
        false => {
            let url = format!("https://http.cat/{}", status);
            match webbrowser::open(&url) {
                Ok(_) => println!("Opened {} in browser", url),
                Err(_) => println!("Failed to open {} in browser", url),
            }
        }
        _ => print_status_code_description(status),
    }

    match args.explain {
        true => println!(
            "Explanation: {}",
            status_codes::StatusCodes::get_explanation(status)
        ),
        _ => (),
    }
}