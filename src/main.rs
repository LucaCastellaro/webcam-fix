use std::{env, fs::{self, remove_file}, os::unix, path::Path};

use indicatif::ProgressBar;

mod loaders_utils;

fn main() {
    if change_dir().is_err()
    {
        return;
    }
    if create_link().is_err()
    {
        return;
    }

    let final_spinner = loaders_utils::get_spinner("Done");
    final_spinner.finish_with_message("Done");
}

fn change_dir() -> Result<(), ()> {
    let spinner = loaders_utils::get_spinner("Navigate to folder /dev");
    let result = env::set_current_dir("/dev");
    match result {
        Ok(_) => {
            spinner.finish_with_message("Inside folder /dev");
            return Ok(());
        },
        Err(err) => {
            spinner.abandon();
            println!("ERROR: {}", err.to_string());
            return Err(());
        }
    }
}

fn create_link() -> Result<(), ()> {
    let spinner = loaders_utils::get_spinner("Create link");

    let original = Path::new("/dev/video2");
    let link = Path::new("/dev/video0");

    let result = delete_file(link, &spinner);
    if result.is_err() {
        return Err(());
    }

    let result = unix::fs::symlink(original, link);
    match result {
        Ok(_) => {
            spinner.finish_with_message("Link created");
            return Ok(());
        },
        Err(err) => {
            spinner.abandon();
            println!("ERROR: {}", err.to_string());
            return Err(());
        }
    }
}

fn delete_file(link: &Path, spinner: &ProgressBar) -> Result<(), ()> {
    if Path::exists(link) {
        spinner.set_message(format!("Delete {:?}", link));
        let result = fs::remove_file(link);
        match result {
            Ok(_) => {
                spinner.set_message(format!("File {:?} deleted", link));
                return Ok(());
            },
            Err(err) => {
                spinner.abandon();
                println!("ERROR: {}", err.to_string());
                return Err(());
            }
        }
    }

    return Ok(());
}