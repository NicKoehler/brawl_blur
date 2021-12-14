extern crate clearscreen;
use std::{io::Write, path::PathBuf, thread::sleep};

mod blur;

fn main() {
    clearscreen::clear().unwrap();
    let mut unix_path: String = String::new();
    let image_path = match std::env::consts::OS {
        "windows" => std::path::Path::new(
            "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Brawlhalla\\mapArt\\Backgrounds\\",
        ),
        "linux" => {
            unix_path.push_str(std::env::var("HOME").unwrap().as_str());
            unix_path.push_str("/.steam/steam/steamapps/common/Brawlhalla/mapArt/Backgrounds/");
            std::path::Path::new(&unix_path)
        }
        "macos" => {
            unix_path.push_str(std::env::var("HOME").unwrap().as_str());
            unix_path.push_str("/Library/Application Support/Steam/steamapps/common/Brawlhalla/Brawlhalla.app/Contents/Resources/mapArt/Backgrounds");
            std::path::Path::new(&unix_path)
        }
        _ => {
            println!("OS not supported.");
            sleep(std::time::Duration::from_millis(3000));
            std::process::exit(1);
        }
    };

    let backup_path = image_path.join("BAK");

    if !image_path.exists() {
        println!("Brawlhalla is not installed.");
        sleep(std::time::Duration::from_millis(3000));
        std::process::exit(1);
    }

    // get a vector of all the files in the directory with the extension .png to be blurred as path
    let files = std::fs::read_dir(image_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".jpg"))
        .map(|e| e.path())
        .collect::<Vec<PathBuf>>();

    let mut user_selection = String::new();
    println!("Brawlhalla Map Blurrer • by NicKoehler\n");

    loop {
        print!(
            "1 • Blur the background of the maps.\n2 • Restore the original images.\n3 • Exit.\n\nSelect an option > "
        );
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut user_selection).unwrap();

        match user_selection.trim() {
            "1" => {
                clearscreen::clear().unwrap();

                // if the BAK folder doesn't exist, create it
                if !&backup_path.exists() {
                    std::fs::create_dir(&backup_path).unwrap();
                }

                match &backup_path.read_dir().unwrap().count() {

                    0 => {
                        // create a mutable string
                        let mut blurred = String::new();
                        let sigma: f32;

                        println!();

                        loop {

                            // ask the user for the blurred value
                            print!("Insert a numeric value for the blur (Recommended: 20) > ");
                            std::io::stdout().flush().unwrap();

                            // read the input
                            std::io::stdin()
                            .read_line(&mut blurred)
                            .unwrap();

                            // convert the input to a float
                            if blurred.trim().parse::<f32>().is_ok() {
                                sigma = blurred.trim().parse().unwrap();
                                break;
                            }

                            clearscreen::clear().unwrap();
                            println!("'{}' is not a number, insert a number.", blurred.trim());
                            blurred.clear();

                        }

                        blur::blur_images(&files, sigma, &backup_path);
                        clearscreen::clear().unwrap();
                        println!("Images blurred successfully.\n");
                    },
                    _ => println!("There are images backups, restore the original images.\n"),
                };
            }

            "2" => {
                // if the backup folder is empty or doesn't exist, don't do anything
                match backup_path.read_dir().unwrap().count() {
                    0 => {
                        clearscreen::clear().unwrap();
                        println!("There aren't images to restore.\n");
                    }
                    _ => {
                        blur::restore_images(&backup_path);
                        clearscreen::clear().unwrap();
                        println!("Images restored successfully.\n");
                    }
                };
            }

            "3" => {
                std::process::exit(0);
            }

            _ => {
                clearscreen::clear().unwrap();
                println!("Invalid option.\n");
            }
        };

        // clears the input
        user_selection.clear();
    }
}
