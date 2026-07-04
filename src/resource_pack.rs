use std::fs::File;
use std::io::{Cursor, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::ZipWriter;
use zip::{result::ZipError, write::SimpleFileOptions};
use photon_rs::PhotonImage;
use image::{ImageBuffer, Rgba};

pub fn generate_resource_pack(
    src_dir: &Path,
    dynamic_image: &PhotonImage,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    if !src_dir.is_dir() {
        return Err(ZipError::FileNotFound.into());
    }

    let mut cursor = std::io::Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(&mut cursor);
    let options = SimpleFileOptions::default()
        .unix_permissions(0o755);

    let walkdir = WalkDir::new(src_dir);
    for entry_result in walkdir.into_iter() {
        let entry = entry_result.map_err(|e| format!("Error traversing {src_dir:?}: {e}"))?;
        let path = entry.path();
        let path_stripped = path.strip_prefix(src_dir)?;
        let path_as_string = path_stripped
            .to_str()
            .ok_or_else(|| format!("{:?} is a Non UTF-8 Path", path_stripped.display()))?
            .to_owned();

        if path.is_file() {
            tracing::info!("Adding file {:?} as {:?} ...", path.display(), path_stripped.display());
            zip.start_file(&path_as_string, options)?;
            let mut f = File::open(path)?;
            std::io::copy(&mut f, &mut zip)?;
        } else if !path_stripped.as_os_str().is_empty() {
            tracing::info!("Adding dir '{}' as '{}' ...", path.display(), path_stripped.display());
            zip.add_directory(&path_as_string, options)?;
        }
    }

    tracing::info!("Injecting dynamic image into resource pack...");
    
    let raw_pixels = dynamic_image.get_raw_pixels();
    let width = dynamic_image.get_width();
    let height = dynamic_image.get_height();

    let img_buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, raw_pixels)
        .ok_or("Failed to create image buffer from raw pixels")?;
    let dynamic_img = image::DynamicImage::ImageRgba8(img_buffer);

    let mut image_bytes: Vec<u8> = Vec::new();
    let mut image_cursor = Cursor::new(&mut image_bytes);
    dynamic_img.write_to(&mut image_cursor, image::ImageFormat::Png)?;

    zip.start_file("assets/minecraft/textures/custom/board.png", options)?;
    zip.write_all(&image_bytes)?;

    zip.finish()?;
    
    Ok(cursor.into_inner())
}