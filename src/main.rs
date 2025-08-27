use std::env;
use std::sync::Arc;
use std::sync::Mutex;

use cliclack::select;
use ignore::WalkBuilder;
use ignore::WalkState;

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

    let matches = Arc::new(Mutex::new(Vec::<String>::new()));

    WalkBuilder::new(&current_dir)
        .max_depth(Some(max))
        .build_parallel()
        .run(|| {
            let matches = Arc::clone(&matches);
            Box::new(move |result| {
                let e = match result {
                    Ok(v) => v,
                    Err(_) => return WalkState::Continue,
                };

                if e.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    let p = e.path();
                    if p.to_string_lossy().contains(search_dir) {
                        matches
                            .lock()
                            .unwrap()
                            .push(p.to_string_lossy().into_owned());
                        // Do not descend further
                        return WalkState::Skip;
                    }
                }
                WalkState::Continue
            })
        });

    let matches: Vec<String> = Arc::try_unwrap(matches).unwrap().into_inner().unwrap();

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
