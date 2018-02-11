use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};

const PATH_TO_CONFIG: &'static str = "C:/Riot Games/League of Legends/Config";
const SETTINGS_JSON_NAME: &'static str  = "PersistedSettings.json";
const BACKUP_JSON_NAME: &'static str  = "PersistedSettingsBackup.json";
const PERMANENT_BACKUP_NAME: &'static str  = "PersistedSettingsPermanent.json";
const TEMPORARY_BACKUP_NAME: &'static str  = "PersistedSettingsTemporary.json";
const REGION_SETTINGS_FILE: &'static str = "LeagueClientSettings.yaml";

fn main() {
    loop {
        //Loop until user exits by using the "exit" command
        let user_option = main_menu();
        
        //Using the &'s and 'as &str' is called cross-borrowing to convert to a str type.
        match &user_option as &str {
            "help" => {
                print_possible_commands();
            },
            "exit" => {
                break;
            },
            "perm_save" => save_permanent_settings(),
            "perm_load" => load_permanent_settings(),
            "temp_save" => save_temporary_settings(),
            "temp_load" => load_temporary_settings(),
            "swap_region" => swap_region(),
            _ => {
                println!("That was not a valid command. Please try again.");
            }
        }
    }


}

fn handle_backup_of_original(config_contents: &String) {
    //This backs up PersistedSettings.json in the event a backup of it does not exist
    let path_to_backup = format!("{}/{}", PATH_TO_CONFIG, BACKUP_JSON_NAME);
    if std::path::Path::new(&path_to_backup).exists(){
        //File exists. Do nothing
        return;
    } else {
        output_to_file(path_to_backup, config_contents);
    }
}

fn save_permanent_settings() {
    //This creates PersistedSettingsPermanent.json
    let path_to_temp_save = format!("{}/{}", PATH_TO_CONFIG, PERMANENT_BACKUP_NAME);

    let mut config_contents_result = read_config_file();
    match config_contents_result {
        Ok(config_contents) => {
            handle_backup_of_original(&config_contents);
            output_to_file(path_to_temp_save, &config_contents);
        },
        Err(fail) => {
            println!("We failed the operation :(");
        }
    }
}

fn load_permanent_settings() {
    //This loads PersistedSettingsPermanent.json to PersistedSettings.json, essentially loading all saved League settings
    let path_to_temp_save = format!("{}/{}", PATH_TO_CONFIG, PERMANENT_BACKUP_NAME);
    let path_to_settings = format!("{}/{}", PATH_TO_CONFIG, SETTINGS_JSON_NAME);

    let mut permanent_config_contents = get_file_contents(path_to_temp_save);
    match permanent_config_contents {
        Ok(config_contents) => {
            handle_backup_of_original(&config_contents);
            output_to_file(path_to_settings, &config_contents);
        },
        Err(fail) => {
            println!("We failed the operation :(");
        }
    }
}

fn save_temporary_settings() {
    //This does the same as save_permanent_settings, but for PersistedSettingsTemporary.json
    let path_to_temp_save = format!("{}/{}", PATH_TO_CONFIG, TEMPORARY_BACKUP_NAME);

    let mut config_contents_result = read_config_file();
    match config_contents_result {
        Ok(config_contents) => {
            handle_backup_of_original(&config_contents);
            output_to_file(path_to_temp_save, &config_contents);
        },
        Err(fail) => {
            println!("We failed the operation :(");
        }
    }
}

fn load_temporary_settings() {
    //This does the same as load_permanent_settings, but for PersistedSettingsTemporary.json
    let path_to_temp_save = format!("{}/{}", PATH_TO_CONFIG, TEMPORARY_BACKUP_NAME);
    let path_to_settings = format!("{}/{}", PATH_TO_CONFIG, SETTINGS_JSON_NAME);

    let mut permanent_config_contents = get_file_contents(path_to_temp_save);
    match permanent_config_contents {
        Ok(config_contents) => {
            handle_backup_of_original(&config_contents);
            output_to_file(path_to_settings, &config_contents);
        },
        Err(fail) => {
            println!("We failed the operation :(");
        }
    }
}
fn swap_region() {
    //This function reads LeagueClientSettings.yaml and replaces NA1 or LA1 with the opposite. The client must be closed
    // for this function to work properly.
    let path_to_region_file = format!("{}/{}", PATH_TO_CONFIG, REGION_SETTINGS_FILE);
    let mut region_config_contents = get_file_contents(path_to_region_file);
    match region_config_contents {
        Ok(contents) => {
            let output_file = format!("{}/{}", PATH_TO_CONFIG, REGION_SETTINGS_FILE);
            if contents.contains("region: \"NA\"") {
                let updated_contents = contents.replace("region: \"NA\"", "region: \"LA1\"");
                output_to_file(output_file, &updated_contents);
            } else {
                let updated_contents = contents.replace("region: \"LA1\"", "region: \"NA\"");
                output_to_file(output_file, &updated_contents);

            }
        },
        Err(fail) => {
            println!("We failed to swap regions :(");
        }
    }
}

fn output_to_file(path: String, data: &String) -> std::io::Result<bool> {
    //Utility function for writing data to file
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(true)
}

fn get_file_contents(path_to_file: String) -> std::io::Result<String> {
    //Utility function for reading data from file
    let mut open_file = File::open(path_to_file)?;
    let mut file_contents = String::new();
    open_file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

fn read_config_file() -> std::io::Result<String> {
    //Utility function for getting PersistedSettings.json's data
    let config_path = format!("{}/{}", PATH_TO_CONFIG, SETTINGS_JSON_NAME);
    return get_file_contents(config_path);
}

fn main_menu() -> String {
    //Used to create the main loop, and prompt the user for their option
    print!("What do you want to do (help for help): ");
    io::stdout().flush();
    return get_user_input();
}

fn get_user_input() -> String {
    //Utility setting for actually getting user input
    let stdin = io::stdin();
    let line = stdin.lock()
        .lines()
        .next()
        .expect("There was no next line")
        .expect("The line could not be read");

    return line.trim().to_lowercase();
}

fn print_possible_commands() {
    println!("help: Prints this message");
    println!("perm_save: Creates a permanent copy of your settings");
    println!("perm_load: Loads an existing permanent copy of your settings");
    println!("temp_save: Creates a temporary backup of your settings");
    println!("temp_load: Loads a temporary backup of your settings");
    println!("swap_region: Swaps region between NA and LAN");
    println!("exit: Exits this program");
}