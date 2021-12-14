use std::{io::Write, path::PathBuf};

mod blur;

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    clear_screen();
    let mut user_home: String = std::env::var("HOME").unwrap();
    let image_path = match std::env::consts::OS {
        "windows" => std::path::Path::new(
            "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Brawlhalla\\mapArt\\Backgrounds\\",
        ),
        _ => {
            user_home.push_str("/.steam/steam/steamapps/common/Brawlhalla/mapArt/Backgrounds/");
            std::path::Path::new(&user_home)
        }
    };

    let backup_path = image_path.join("BAK");

    // get a vector of all the files in the directory with the extension .png to be blurred as path
    let files = std::fs::read_dir(image_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".jpg"))
        .map(|e| e.path())
        .collect::<Vec<PathBuf>>();

    let mut scelta = String::new();
    println!("Brawlhalla Map Blurrer • NicKoehler\n");

    loop {
        print!(
            "1 • Sfoca lo sfondo delle mappe.\n2 • Ripristina le immagini originali.\n3 • Chiudi il programma.\n\nScegli > "
        );
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut scelta).unwrap();

        match scelta.trim() {
            "1" => {
                clear_screen();

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
                            print!("Inserisci il valore di sfocatura > ");
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

                            clear_screen();
                            println!("Valore '{}' non valido, inserisci un numero.", blurred.trim());
                            blurred.clear();

                        }

                        blur::blur_images(&files, sigma, &backup_path);
                        clear_screen();
                        println!("Immagini sfocate correttamente.\n");
                    },
                    _ => println!("Esistono dei backup precedenti. Ripristina prima di effettuare una nuova sfocatura.\n"),
                };
            }

            "2" => {
                // if the backup folder is empty or doesn't exist, don't do anything
                match backup_path.read_dir().unwrap().count() {
                    0 => {
                        clear_screen();
                        println!("Non ci sono immagini da ripristinare.\n");
                    }
                    _ => {
                        blur::restore_images(&backup_path);
                        clear_screen();
                        println!("Immagini ripristinate correttamente.\n");
                    }
                };
            }

            "3" => {
                std::process::exit(0);
            }

            _ => {
                clear_screen();
                println!("Opzione non valida.\n");
            }
        };

        // clears the input
        scelta.clear();
    }
}
