use std::fs;
use std::io::Write;
use std::path::Path;
use std::{env, io};

// const
const PROCESS_FAILED: i32 = 0;
const PROCESS_REGISTER: i32 = 1;
const PROCESS_COPY_EXTENSION_EXIST: i32 = 2;
const PROCESS_COPY_EXTENSION_WITHOUT: i32 = 3;

fn main() {
    // create template directory
    if !check_template_dir() {
        match env::var_os("HOME") {
            Some(path) => path,
            None => {
                println!("Could not find home directory.");
                return;
            }
        };
        if !create_tempalte_dir() {
            println!("Could not find home directory.");
            return;
        }
    }

    // arg check
    let args: Vec<String> = env::args().collect();

    let process_status: i32 = check_args(args.clone());

    if process_status == PROCESS_FAILED {
        return;
    } else if process_status == PROCESS_REGISTER {
        let register_dir: String;
        if let Some(extension) = get_file_extension(args[2].clone()) {
            register_dir = extension;
        } else {
            register_dir = "various".to_string();
        }
        // check extension dir
        if !check_extension_dir(register_dir.clone()) {
            // create extension dir
            if !create_extension_dir(register_dir.clone()) {
                println!("Could not find extension directory.");
                return;
            }
        }

        // copy file
        if !register_template_file(register_dir, args[2].clone()) {
            return;
        }

        println!("template file copy done!!");
    } else if process_status == PROCESS_COPY_EXTENSION_WITHOUT {
        // select dir
        let (proc, selected_dir) = select_extension_dir();
        if proc {
            // select template file
            let (proc, selected_file) = select_copy_file(selected_dir);
            if proc {
                // copy file
                if !template_file_copy(selected_file) {
                    return;
                }
                println!("template file copy done!!");
            } else {
                println!("Failed to select file.");
            }
        } else {
            println!("Failed to select dir.");
        }
    }
}

fn check_template_dir() -> bool {
    let home_dir = match env::var_os("HOME") {
        Some(path) => path,
        None => {
            return false;
        }
    };

    let template_dir = Path::new(&home_dir).join(".template");

    if template_dir.exists() {
        true
    } else {
        false
    }
}

fn create_tempalte_dir() -> bool {
    let home_dir = match env::var_os("HOME") {
        Some(path) => path,
        None => {
            return false;
        }
    };
    let template_dir = Path::new(&home_dir).join(".template");
    if let Err(err) = std::fs::create_dir(&template_dir) {
        eprintln!("Failed to create diretory: {}", err);
        return false;
    }
    true
}

fn check_args(args: Vec<String>) -> i32 {
    if args.len() < 2 {
        print_usage();
        return PROCESS_FAILED;
    }

    let option = &args[1];

    match option.as_str() {
        "-r" => {
            if args.len() == 3 {
                PROCESS_REGISTER
            } else {
                println!("No value specified");
                print_usage();
                PROCESS_FAILED
            }
        }
        "-c" => {
            if args.len() == 2 {
                PROCESS_COPY_EXTENSION_WITHOUT
            } else if args.len() == 3 {
                PROCESS_COPY_EXTENSION_EXIST
            } else {
                println!("There are many arguments");
                print_usage();
                PROCESS_FAILED
            }
        }
        _ => {
            println!("Invalid option");
            print_usage();
            PROCESS_FAILED
        }
    }
}

fn print_usage() {
    println!("Usage: easy-template <option> [filename | extension name]");
    println!("Option:");
    println!("  -r [filename]         Register");
    println!("  -c [extension name]   file copy");
}

fn get_file_extension(filename: String) -> Option<String> {
    let components: Vec<&str> = filename.split('.').collect();
    if components.len() > 1 {
        Some(components.last().unwrap().to_string())
    } else {
        None
    }
}

fn check_extension_dir(dirname: String) -> bool {
    let home_dir = match env::var_os("HOME") {
        Some(path) => path,
        None => {
            return false;
        }
    };

    let exetension_dir = Path::new(&home_dir).join(".template").join(dirname);

    if exetension_dir.exists() {
        true
    } else {
        false
    }
}

fn create_extension_dir(dirname: String) -> bool {
    let home_dir = match env::var_os("HOME") {
        Some(path) => path,
        None => {
            return false;
        }
    };

    let exetension_dir = Path::new(&home_dir).join(".template").join(dirname);
    if let Err(err) = std::fs::create_dir(&exetension_dir) {
        eprintln!("Failed to create diretory: {}", err);
        return false;
    }
    true
}

fn register_template_file(dirname: String, filename: String) -> bool {
    let home_dir = match env::var_os("HOME") {
        Some(path) => path,
        None => {
            return false;
        }
    };

    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("Failed to get current directory: {}", err);
            return false;
        }
    };

    let sorce_path = current_dir.join(filename.clone());
    let destination_path = Path::new(&home_dir)
        .join(".template")
        .join(dirname)
        .join(filename);

    match fs::copy(&sorce_path, &destination_path) {
        Ok(_) => true,
        Err(err) => {
            eprintln!("Failed to copy file: {}", err);
            return false;
        }
    }
}

fn select_extension_dir() -> (bool, String) {
    let template_dir = dirs::home_dir().unwrap().join(".template");

    let entires = match fs::read_dir(&template_dir) {
        Ok(entries) => entries,
        Err(err) => {
            println!("Failed to read .tempalte directory: {}", err);
            return (false, "".to_string());
        }
    };

    let mut index = 1;
    let mut entry_count = 0;
    let mut dir_paths: Vec<String> = Vec::new();

    for entry in entires {
        if let Ok(entry) = entry {
            if let Some(name) = entry.file_name().to_str() {
                println!("{}: {}", index, name);
                index += 1;
                entry_count += 1;
                dir_paths.push(entry.path().to_string_lossy().into_owned());
            }
        }
    }

    let selection = get_user_input("Enter the number: ").unwrap_or_else(|_| String::new());

    let number = match selection.parse::<usize>() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input.");
            return (false, "".to_string());
        }
    };

    if number > entry_count || number <= 0 {
        println!("Invalid number.");
        return (false, "".to_string());
    }

    (true, dir_paths[number - 1].clone())
}

fn get_user_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn select_copy_file(dir_name: String) -> (bool, String) {
    let path_buf_dir = Path::new(&dir_name).to_path_buf();

    let files = match fs::read_dir(&path_buf_dir) {
        Ok(files) => files,
        Err(err) => {
            println!("Failed to read extension directory: {}", err);
            return (false, "".to_string());
        }
    };

    let mut index = 1;
    let mut file_count = 0;
    let mut file_name_array: Vec<String> = Vec::new();

    for file in files {
        if let Ok(file) = file {
            if let Some(name) = file.file_name().to_str() {
                println!("{}: {}", index, name);
                index += 1;
                file_count += 1;
                file_name_array.push(file.path().to_string_lossy().into_owned());
            }
        }
    }

    let selection = get_user_input("Enter the number: ").unwrap_or_else(|_| String::new());

    let number = match selection.parse::<usize>() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input.");
            return (false, "".to_string());
        }
    };

    if number > file_count || number <= 0 {
        println!("Invalid number.");
        return (false, "".to_string());
    }

    (true, file_name_array[number - 1].clone())
}

fn template_file_copy(template_file_name: String) -> bool {
    // get current dir
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("Failed to get current directory: {}", err);
            return false;
        }
    };
    // split extention
    let components: Vec<&str> = template_file_name.split('.').collect();
    // components lats element is extention
    let extention = components.last().unwrap().to_string();    
    // input file name
    let input_file_name = get_user_input("Enter the file name: ").unwrap_or_else(|_| String::new());
    // Copy template files to the current directory
    // If the file name is not specified, the file name is the same as the template file name.
    if input_file_name == "" {
        let no_specified_file_name = "template".to_string() + "." + &extention;
        let sorce_path = Path::new(&template_file_name).to_path_buf();
        let destination_path = current_dir.join(no_specified_file_name.clone());
        match fs::copy(&sorce_path, &destination_path) {
            Ok(_) => println!("template file copy done!!"),
            Err(err) => {
                eprintln!("Failed to copy file: {}", err);
                return false;
            }
        }
        return true;
    }
    let specified_file_name = input_file_name.clone() + "." + &extention;
    let sorce_path = Path::new(&template_file_name).to_path_buf();
    let destination_path = current_dir.join(specified_file_name.clone());

    match fs::copy(&sorce_path, &destination_path) {
        Ok(_) => println!("{} copy done!!", specified_file_name),
        Err(err) => {
            eprintln!("Failed to copy file: {}", err);
            return false;
        }
    }

    return true;

}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    use super::*;

    #[test]
    fn test_check_template_dir() {
        let template_dir = dirs::home_dir().unwrap().join(".template");
        if template_dir.exists() {
            std::fs::remove_dir(template_dir.clone()).unwrap();
        }
        // create an existing directory
        std::fs::create_dir(template_dir.clone()).unwrap();

        // test
        assert_eq!(check_template_dir(), true);

        // delete the created directory
        std::fs::remove_dir(template_dir).unwrap();
    }

    #[test]
    fn test_check_dir_not_found() {
        // set a non-existent environment variable
        std::env::set_var("HOME", "");

        // test
        assert_eq!(check_template_dir(), false);
    }

    #[test]
    fn test_create_template_dir() {
        let template_dir = dirs::home_dir().unwrap().join(".template");
        if template_dir.exists() {
            std::fs::remove_dir(template_dir.clone()).unwrap();
        }
        let temp_dir = tempfile::tempdir().unwrap();
        env::set_var("HOME", temp_dir.path());

        let result = create_tempalte_dir();

        let template_dir = temp_dir.path().join(".template");
        assert_eq!(result, true);
        assert_eq!(fs::metadata(template_dir).is_ok(), true);
    }

    #[test]
    fn test_check_args_register_with_value() {
        let args = vec![
            String::from("program_name"),
            String::from("-r"),
            String::from("value"),
        ];

        assert_eq!(check_args(args), PROCESS_REGISTER);
    }

    #[test]
    fn test_check_args_register_without_value() {
        let args = vec![String::from("program_name"), String::from("-r")];

        assert_eq!(check_args(args), PROCESS_FAILED);
    }

    #[test]
    fn test_check_args_copy_with_extension() {
        let args = vec![
            String::from("program_name"),
            String::from("-c"),
            String::from("value"),
        ];

        assert_eq!(check_args(args), PROCESS_COPY_EXTENSION_EXIST);
    }

    #[test]
    fn test_check_args_copy_without_extension() {
        let args = vec![String::from("program_name"), String::from("-c")];

        assert_eq!(check_args(args), PROCESS_COPY_EXTENSION_WITHOUT);
    }

    #[test]
    fn test_check_args_invalid_option() {
        let args = vec![String::from("program_name"), String::from("-x")];

        assert_eq!(check_args(args), PROCESS_FAILED);
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(
            get_file_extension("exmaple.txt".to_string()),
            Some("txt".to_string())
        );

        assert_eq!(get_file_extension("no_extention".to_string()), None);

        assert_eq!(
            get_file_extension(".hidden".to_string()),
            Some("hidden".to_string())
        );

        assert_eq!(get_file_extension("".to_string()), None);
    }

    #[test]
    fn test_check_extension_dir() {
        let extension_dir = dirs::home_dir().unwrap().join(".template").join("txt");
        if extension_dir.exists() {
            std::fs::remove_dir(extension_dir.clone()).unwrap();
        }

        // false test
        assert_eq!(check_extension_dir("txt".to_string()), false);

        // create an existing directory
        std::fs::create_dir(extension_dir.clone()).unwrap();

        // true test
        assert_eq!(check_extension_dir("txt".to_string()), true);

        // delete the created directory
        std::fs::remove_dir(extension_dir).unwrap();
    }

    #[test]
    fn test_create_extension_dir() {
        let extension_dir = dirs::home_dir().unwrap().join(".template").join("txt");
        if extension_dir.exists() {
            std::fs::remove_dir(extension_dir.clone()).unwrap();
        }
        if create_tempalte_dir() {
            let temp_dir = tempfile::tempdir().unwrap();
            env::set_var("HOME", temp_dir.path());

            let result = create_extension_dir("txt".to_string());

            let extension_dir = temp_dir.path().join(".template").join("txt");
            assert_eq!(result, true);
            assert_eq!(fs::metadata(extension_dir).is_ok(), true);
        }
    }

    #[test]
    fn test_register_template_file() {
        let file_path = "example.txt";
        let tempalte_dirname = "txt".to_string();

        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to create file: {}", err);
                return;
            }
        };

        match file.write_all(b"This is n example file") {
            Ok(_) => return,
            Err(err) => eprintln!("Failed to write to file: {}", err),
        }

        let template_dir = dirs::home_dir().unwrap().join(".template");
        let extension_dir = template_dir.join("txt");

        if template_dir.exists() && !extension_dir.exists() {
            if !create_extension_dir(tempalte_dirname.clone()) {
                return;
            }
        } else if !template_dir.exists() && !extension_dir.exists() {
            if !create_tempalte_dir() || !create_extension_dir(tempalte_dirname.clone()) {
                return;
            }
        }

        let result = register_template_file(tempalte_dirname, file_path.to_string());
        let destination_path = extension_dir.join(file_path);

        assert_eq!(result, true);
        assert_eq!(destination_path.exists(), true);

        std::fs::remove_dir(destination_path.clone()).unwrap();
        std::fs::remove_dir(extension_dir.clone()).unwrap();
        std::fs::remove_dir(template_dir.clone()).unwrap();
    }

    #[test]
    fn test_get_user_input() {
        let input = "Hello Wolrd\n";
        let mut stdin = std::io::Cursor::new(input);

        let result = get_user_input("Enter your input:").unwrap();

        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_select_extension_dir() {
        // create template and exetend dir
        let extension_dir = dirs::home_dir().unwrap().join(".template").join("txt");
        if !extension_dir.exists() {
            let template_dir = dirs::home_dir().unwrap().join(".template");
            if template_dir.exists() {
                if !create_extension_dir("txt".to_string()) {
                    eprintln!("Failed to create extend dir");
                }
            } else {
                if create_tempalte_dir() {
                    if !create_extension_dir("txt".to_string()) {
                        eprintln!("Failed to create extend dir");
                    }
                } else {
                    eprintln!("Failed to create template dir");
                }
            }
        }

        // select_extension_dir test
        let (proc, selected_dir) = select_extension_dir();
        if proc {
            let string_exetend_dir = extension_dir.to_string_lossy().into_owned();
            assert_eq!(selected_dir, string_exetend_dir);
        } else {
            eprintln!("Failed to select dir")
        }
    }

    #[test]
    fn test_select_copy_file() {
        // create template and exetend dir
        let extension_dir = dirs::home_dir().unwrap().join(".template").join("test");
        if !extension_dir.exists() {
            let template_dir = dirs::home_dir().unwrap().join(".template");
            if template_dir.exists() {
                if !create_extension_dir("test".to_string()) {
                    eprintln!("Failed to create extend dir");
                }
            } else {
                if create_tempalte_dir() {
                    if !create_extension_dir("test".to_string()) {
                        eprintln!("Failed to create extend dir");
                    }
                } else {
                    eprintln!("Failed to create template dir");
                }
            }
        }

        // test file create
        let file_names = ["file1", "file2", "file3"];
        let selection_file = format!("{}/{}", extension_dir.to_string_lossy(), file_names[2]);

        for (index, file_name) in file_names.iter().enumerate() {
            let file_content = format!("This is test file {}", index);
            let file_path = format!("{}/{}", extension_dir.to_string_lossy(), file_name);
            fs::write(file_path, file_content).unwrap();
        }

        let (result, selected_file) =
            select_copy_file(extension_dir.to_string_lossy().into_owned());

        // test dir and test file delete
        fs::remove_dir_all(extension_dir).unwrap();

        assert!(result);
        assert_eq!(selection_file, selected_file);
    }
}
