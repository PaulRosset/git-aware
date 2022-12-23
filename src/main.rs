
use ignore::WalkBuilder;
use git2::{Repository, StatusOptions};


fn is_git_present_on_path(path: String) -> bool {
    let splitted_path = path.split("/");
    match splitted_path.last() {
        Some(last_element) => return last_element == ".git",
        None => {
            return false
        }
    }
}


fn main() {
    for result in WalkBuilder::new("/Users/paulrosset/Documents").hidden(false).build() {
        match result {
            Ok(entry) => {
                let path = entry.path().display();
                // Means that we detected git
                if path.to_string().ends_with(".git") {
                    let path_to_open_repo = path.to_string().replace("/.git", "");
                    let repo = match Repository::open(&path_to_open_repo) {
                        Ok(repo) => repo,
                        Err(err) => panic!("Failed to init: {}", err)
                    }; 
                    let mut status_opts = StatusOptions::new();
                    status_opts.include_untracked(true);
                    let statuses = repo.statuses(Some(&mut status_opts));
                    let statuses = match statuses {
                        Ok(statuses) => statuses,
                        Err(err) => panic!("Error while trying to get statuses from the repo: {}", err),
                    };
                    if statuses.is_empty() {
                        println!("{}: Working directory clean", path_to_open_repo);
                    } else {
                        println!("{}: Working directiory dirty", path_to_open_repo)
                    }
                }
            },
            Err(err) => println!("{}", err)
        }
    }
}