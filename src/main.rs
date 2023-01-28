use std::{env, fs, os::unix};

mod loaders_utils;

fn main() {
    if change_dir().is_err()
    {
        return;
    }
    if backup_original().is_err()
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

fn backup_original() -> Result<(), ()> {
    let spinner = loaders_utils::get_spinner("Copy original");
    let result = fs::copy("video0", "video0.original");
    match result {
        Ok(_) => {
            spinner.finish_with_message("Original copied");
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
    let result = unix::fs::symlink("/dev/video2", "/dev/vide0");
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
    }}
