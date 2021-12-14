use fastblur::gaussian_blur;
use std::path::{Path, PathBuf};

// define a function that takes a vector of image paths and a blur value
// and blur the images

pub fn blur_images(images: &Vec<PathBuf>, blur_value: f32, backup_path: &Path) {
    // iterate over the images
    for image_path in images {
        // backup the original image in a BAK folder
        let temp_path = backup_path.join(&image_path.file_name().unwrap());

        std::fs::copy(&image_path, &temp_path).unwrap();

        // load image
        let image = image::open(&image_path).unwrap();

        let jpg_data = image.as_rgb8().unwrap();

        let (width, height) = jpg_data.dimensions();

        let mut data: Vec<[u8; 3]> = Vec::new();

        // iterate over the pixels
        for y in 0..height {
            for x in 0..width {
                let pixel = jpg_data.get_pixel(x, y);
                data.push([pixel[0], pixel[1], pixel[2]]);
            }
        }

        // blur the image
        gaussian_blur(
            &mut data,
            usize::try_from(width).unwrap(),
            usize::try_from(height).unwrap(),
            blur_value,
        );

        // create a new image from Vec<[u8; 3]>
        let mut new_image = image::ImageBuffer::new(width, height);

        // inserting the data into the new image
        for (index, pixel) in data.iter().enumerate() {
            let x = index as u32 % width;
            let y = index as u32 / width;
            new_image.put_pixel(x, y, image::Rgb([pixel[0], pixel[1], pixel[2]]));
        }

        // save the new image
        new_image.save(&image_path).unwrap();

        println!(
            "{:?} -> sfocata correttamente.",
            &image_path.file_name().unwrap()
        );
    }
}

pub fn restore_images(backup_path: &Path) {
    // get the parent folder of the backup folder
    let parent_folder = backup_path.parent().unwrap();

    // iterate over the images
    for image_path in std::fs::read_dir(backup_path).unwrap() {
        // get the path of the image
        let image_path = image_path.unwrap().path();

        // get the filename
        let filename = image_path.file_name().unwrap();

        // restore the image
        std::fs::rename(&image_path, parent_folder.join(filename)).unwrap();
        println!(
            "{} -> ripristinata correttamente.",
            image_path.file_name().unwrap().to_str().unwrap()
        );
    }
}
