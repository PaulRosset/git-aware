pub struct Config<'a> {
    pub default: bool,
    pub options: Vec<String>,
    pub path: &'a str,
}

pub fn params_checker(argv: &Vec<String>) -> Result<Config, String> {
    if argv.len() == 1 {
        return Ok(Config {
            default: true,
            options: [].to_vec(),
            path: "./",
        })
    }
    let get_p_flag = argv.len() > 1 && argv[1] == "-p";
    if !get_p_flag {
        return Err("The flag -p followed with the right path is mandatory to specify a customized path.".to_string());
    }
    match argv.last() {
        Some(path) => {
            if path == "-p" {
                return Err("You must specify specify a path after using -p option: -p MYPATH.".to_string())
            }
            return Ok(Config {
                default: false,
                options: vec!["-p".to_string()],
                path: path,
            })
        },
        None => {
            return Err("You must specify specify a path after using -p option: -p MYPATH.".to_string());
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::params_checker;

    #[test]
    fn test_params_checker_when_no_params() {
        let argv = vec!["gitaware".to_string()];
        let config = params_checker(&argv).unwrap();
        assert!(config.default, "we should take the default path");
        assert_eq!(config.path, "./");
    }

    #[test]
    fn should_fail_as_no_proper_path_is_provided() {
        let argv = vec!["gitaware".to_string(), "./".to_string()];
        let config = params_checker(&argv);
        match config {
            Ok(_) => todo!(),
            Err(msg) => assert_eq!(msg, "The flag -p followed with the right path is mandatory to specify a customized path."),
        }
    }

    #[test]
    fn should_fail_as_no_proper_path_is_provided_badly() {
        let argv = vec!["gitaware".to_string(), "./".to_string(), "-p".to_string()];
        let config = params_checker(&argv);
        match config {
            Ok(_) => todo!(),
            Err(msg) => assert_eq!(msg, "The flag -p followed with the right path is mandatory to specify a customized path."),
        }
    }

    #[test]
    fn should_fail_as_no_proper_path_is_provided_after_option() {
        let argv = vec!["gitaware".to_string(), "-p".to_string()];
        let config = params_checker(&argv);
        match config {
            Ok(_) => todo!(),
            Err(msg) => assert_eq!(msg, "You must specify specify a path after using -p option: -p MYPATH."),
        }
    }

    #[test]
    fn should_works() {
        let argv = vec!["gitaware".to_string(), "-p".to_string(), "./".to_string()];
        let config = params_checker(&argv);
        match config {
            Ok(config) => {
                assert!(config.options.contains(&"-p".to_string()));
                assert_eq!(config.path, "./");
            },
            Err(_) => todo!(),
        }
    }
}