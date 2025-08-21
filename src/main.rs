use std::env;

use cliclack::select;
use walkdir::WalkDir;

fn main() {
    rcd();
}

fn rcd() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 && args.len() != 2 {
        eprintln!("Usage: rcd <dir_to_head_into> <depth [default = 5]>");
        std::process::exit(1);
    }

    if args.iter().any(|a| a == "-h" || a == "--help") {
        eprintln!("Usage: rcd <dir_to_head_into>");
        std::process::exit(0);
    }

    let search_dir = &args[1];
    let current_dir = env::current_dir().expect("failed to read current dir");
    let max = if args.len() == 3 {
        args[2].parse::<usize>().unwrap()
    } else {
        5
    };

    let mut it = WalkDir::new(current_dir).max_depth(max).into_iter();
    let mut matches: Vec<String> = Vec::new();

    while let Some(Ok(e)) = it.next() {
        if e.file_type().is_dir() {
            let p = e.path();
            if p.to_string_lossy().contains(search_dir) {
                matches.push(p.to_string_lossy().into_owned());
                // do not descend further
                it.skip_current_dir();
            }
        }
    }

    if matches.len() == 1 {
        println!("{}", matches[0]);
    } else if matches.len() > 1 {
        let prompt = "Multiple entries were found. Select one: ";

        let selection = select(prompt)
            .items(
                &matches
                    .iter()
                    .map(|c| (c.as_str(), c.as_str(), ""))
                    .collect::<Vec<_>>(),
            )
            .interact()
            .expect("Program shut down unexpectedly");

        println!("{}", &selection);
    } else {
        println!("No matches found.");
    }
}
