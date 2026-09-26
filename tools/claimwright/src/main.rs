mod publication;
mod substrate;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.get(1).map(String::as_str) == Some("publication")
        && args.get(2).map(String::as_str) == Some("init-review")
    {
        match publication::cli::init_review(&args) {
            Ok(message) => println!("{message}"),
            Err(message) => {
                eprintln!("error: {message}");
                process::exit(publication::cli::error_exit_code(&message));
            }
        }
        return;
    }
    if args.get(1).map(String::as_str) == Some("publication")
        && args.get(2).map(String::as_str) == Some("check")
    {
        match publication::cli::check_review(&args) {
            Ok((text, code)) => {
                print!("{text}");
                process::exit(code);
            }
            Err(message) => {
                eprintln!("error: {message}");
                process::exit(publication::cli::error_exit_code(&message));
            }
        }
    }
    if args.get(1).map(String::as_str) == Some("publication")
        && args.get(2).map(String::as_str) == Some("similarity")
        && args.get(3).map(String::as_str) == Some("generate")
    {
        if let Err(message) = publication::cli::validate_options(
            &args,
            4,
            &[
                "--artifact",
                "--comparison-corpus",
                "--output",
                "--ngram-size",
                "--jaccard-threshold",
            ],
            &[],
        ) {
            eprintln!("error: {message}");
            process::exit(2);
        }
        let mut artifact = None;
        let mut corpus = None;
        let mut output = None;
        let mut ngram = 8usize;
        let mut threshold = 0.7f64;
        let mut i = 4;
        while i < args.len() {
            match args[i].as_str() {
                "--artifact" => {
                    i += 1;
                    artifact = args.get(i)
                }
                "--comparison-corpus" => {
                    i += 1;
                    corpus = args.get(i)
                }
                "--output" => {
                    i += 1;
                    output = args.get(i)
                }
                "--ngram-size" => {
                    i += 1;
                    ngram = args[i].parse().unwrap_or(0)
                }
                "--jaccard-threshold" => {
                    i += 1;
                    threshold = args[i].parse().unwrap_or(f64::NAN)
                }
                x => {
                    eprintln!("error: unknown option: {x}");
                    process::exit(2)
                }
            }
            i += 1;
        }
        if ngram == 0 || !threshold.is_finite() || !(0.0..=1.0).contains(&threshold) {
            eprintln!(
                "error: ngram size must be positive and Jaccard threshold must be between 0 and 1"
            );
            process::exit(2);
        }
        match (artifact, corpus, output) {
            (Some(a), Some(c), Some(o)) => match publication::similarity::generate(
                std::path::Path::new(a),
                std::path::Path::new(c),
                std::path::Path::new(o),
                ngram,
                threshold,
            ) {
                Ok(()) => println!("Generated similarity candidate report: {o}"),
                Err(e) => {
                    eprintln!("error: {e}");
                    process::exit(3)
                }
            },
            _ => {
                eprintln!("error: --artifact, --comparison-corpus, and --output are required");
                process::exit(2)
            }
        }
        return;
    }
    match substrate::run(&args) {
        substrate::Outcome::Success => {
            println!("ClaimWright check passed: policy substrate is present.");
        }
        substrate::Outcome::UsageError(message) => {
            eprintln!("{}", message);
            process::exit(2);
        }
        substrate::Outcome::Failure(failures) => {
            for failure in failures {
                eprintln!("error: {}", failure);
            }
            process::exit(1);
        }
    }
}
