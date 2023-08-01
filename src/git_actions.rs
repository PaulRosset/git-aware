use git2::{Repository, StatusOptions, Error};

pub struct RepoData {
    pub is_empty_status: bool,
    pub branch_name: String,
    pub is_detached: bool,
}

/**
 * Open a Git repository and perform the equivalent of a Git status to determine
 * if the working directory could be considered dirty or not.
 */
pub fn is_git_working_directory_clean(path_to_project: &String) -> Result<RepoData, Error> {
    // Open a valid Git repository
    let repo =  match Repository::open(&path_to_project) {
        Ok(repo) => repo,
        Err(err) => return Err(err)
    };

    let head = match repo.head() {
        Ok(head) => head,
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
    Ok(RepoData { is_empty_status: statuses.is_empty(), branch_name: if let Some(branch_name) = head.name() { branch_name.to_string() } else { "".to_string() } , is_detached: !head.is_branch() })
}