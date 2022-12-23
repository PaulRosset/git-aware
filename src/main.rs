
mod git_actions;

use std::{env, process};
use ignore::{WalkBuilder, WalkState};
use std::sync::{Arc, Mutex};
use std::time::Instant;

/**
 * Walk through the provided path, recursively in parallel detect Git repository.
 * Open them and check their status.
 */
fn get_git_results(path_to_directories: &str) {
    let entries = Arc::new(Mutex::new(vec![]));
    // Wall through targeted directory in parallel
    let files = WalkBuilder::new(path_to_directories).hidden(false).build_parallel();
    files.run(|| {
        let entries = entries.clone();
        Box::new(move |result| {
            if let Ok(file) = result {
                entries.lock().unwrap().push(file);
            }
            WalkState::Continue
        })
     });
     let entries = entries.lock().unwrap().to_vec();
     if entries.len() == 0 {
        eprintln!("Error: No files have been detected with the path you provided.");
        process::exit(1);
     }
     for entry in entries.to_vec() {
        let path = entry.path().display();
        if path.to_string().ends_with(".git") {
            let path_to_open_repo = path.to_string().replace("/.git", "");
            // Get status of git repository.
            let result = git_actions::is_git_working_directory_clean(&path_to_open_repo);
            match result {
                Ok(is_repo_clean) => {
                    if is_repo_clean {
                        println!("✅ - {}:\n \tWorking directory clean.", path_to_open_repo);
                    } else {
                        println!("❌ - {}\n \tWorking directory dirty", path_to_open_repo);
                    }
                },
                Err(err) => panic!("An Error has been encountered while detecting statuses on git repository: {}\n - {}.", path_to_open_repo, err)
            }
        }
     }
}


fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No argument -p passed, taking the current directory as default. ('./')\n");
        get_git_results("./");
        return;
    }
    let get_p_flag = args.contains(&"-p".to_string());
    if !get_p_flag {
        eprintln!("The flag -p followed with the right path is mandatory to specify a customized path.");
        process::exit(1);
    }
    match args.last() {
        Some(path) => get_git_results(path),
        None => {
            eprintln!("You must specify specify a path after using -p option: -p MYPATH.");
            process::exit(1);
        }
    };
    println!("\nScanned in: {:.2?}", now.elapsed());
}