use git2::{Repository, StatusOptions, Error};

/**
 * Open a Git repository and perform the equivalent of a Git status to determine
 * if the working directory could be considered dirty or not.
 */
pub fn is_git_working_directory_clean(path_to_project: &String) -> Result<bool, Error> {
    // Open a valid Git repository
    let repo =  match Repository::open(&path_to_project) {
        Ok(repo) => repo,
        Err(err) => return Err(err)
    };

    // Create a StatusOptions object
    let mut status_opts = StatusOptions::new();
    // Ask to also include untracked files
    status_opts.include_untracked(true);

    // Get the status of the repository at the root of the project depending of the status options provided
    let statuses = repo.statuses(Some(&mut status_opts));
    let statuses = match statuses {
        Ok(statuses) => statuses,
        Err(err) => return Err(err),
    };
    Ok(statuses.is_empty())
}