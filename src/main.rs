
mod git_actions;
mod params_checker;

use std::{env, process};
use ignore::{WalkBuilder, WalkState};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use params_checker::{params_checker};

/**
 * Walk through the provided path, recursively in parallel detect Git repository.
 * Open them and check their status.
 */
fn get_git_results(path_to_directories: &str) -> Result<i32, String> {
    let mut git_repositories_scanned = 0;
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
        return Err("Error: No files have been detected with the path you provided.".to_string());
     }
     for entry in entries.to_vec() {
        let path = entry.path().display();
        if path.to_string().ends_with(".git") {
            let path_to_open_repo = path.to_string().replace("/.git", "");
            // Get status of git repository.
            let result = git_actions::is_git_working_directory_clean(&path_to_open_repo);
            match result {
                Ok(repo_data) => {
                    if repo_data.is_empty_status {
                        println!("✅ - {}:\n \tWorking directory {} {} clean.", path_to_open_repo, repo_data.branch_name, if repo_data.is_detached {"DETACHED"} else {""});
                    } else {
                        println!("❌ - {}\n \tWorking directory {} {} clean.", path_to_open_repo, repo_data.branch_name, if repo_data.is_detached {"DETACHED"} else {""})
                    }
                    git_repositories_scanned += 1;
                },
                Err(err) => return Err(format!("An Error has been encountered while detecting statuses on git repository: {}\n - {}.", path_to_open_repo, err))
            }
        }
     }
     return Ok(git_repositories_scanned);
}


fn main() {
    let now = Instant::now();
    let argv: Vec<String> = env::args().collect();
    let config = params_checker(&argv);
    match config {
        Ok(config) => {
            if config.default {
                println!("No argument -p passed, taking the current directory as default. ('./')\n");
                match get_git_results(config.path) {
                    Ok(git_repositories_scanned) => {
                        println!("\nRepositories scanned: {:?}", git_repositories_scanned);
                    },
                    Err(msg) => {
                        eprintln!("{}", msg);
                        process::exit(1);
                    }
                };
            }
            match get_git_results(config.path) {
                Ok(git_repositories_scanned) => {
                    println!("\nRepositories scanned: {:?}", git_repositories_scanned);
                },
                Err(msg) => {
                    eprintln!("{}", msg);
                    process::exit(1);
                }
            };
        },
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    };
    println!("\nScanned in: {:.2?}", now.elapsed());
}


#[cfg(test)]
mod tests {
    use crate::get_git_results;

    #[test]
    fn test_params_checker_when_invalid_path() {
        let path = "path_that_does_not_exists";
        let result = get_git_results(path);
        match result {
            Ok(_) => todo!(),
            Err(msg) => {
                assert_eq!(msg, "Error: No files have been detected with the path you provided.")
            }
        }
    }

    #[test]
    fn test_params_checker_it_should_not_detect_any_repo() {
        let path = "./src";
        let result = get_git_results(path);
        match result {
            Ok(git_repositories_scanned) => assert_eq!(git_repositories_scanned, 0),
            Err(_) => todo!()
        }
    }

    #[test]
    fn test_params_checker_it_works() {
        let path = "./";
        let result = get_git_results(path);
        match result {
            Ok(git_repositories_scanned) => assert_eq!(git_repositories_scanned, 1),
            Err(_) => todo!()
        }
    }
}