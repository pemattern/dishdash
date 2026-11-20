use std::{path::PathBuf, sync::LazyLock};

use image::ImageFormat;
use tokio::fs;
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    repositories::file_system::DATA_PATH,
};

static IMAGES_PATH: LazyLock<PathBuf> = LazyLock::new(|| DATA_PATH.join("images"));
static RECIPES_PATH: LazyLock<PathBuf> = LazyLock::new(|| IMAGES_PATH.join("recipes"));

pub struct Images;

impl Images {
    pub async fn save_recipe_image(recipe_id: Uuid, bytes: &[u8]) -> AppResult<PathBuf> {
        let mut path = RECIPES_PATH.join(recipe_id.to_string());
        fs::create_dir_all(&path).await?;
        let extension = detect_image_extension(bytes)?;
        let filename = format!("{}.{}", Uuid::now_v7(), extension);
        path.push(filename);
        fs::write(&path, bytes).await?;
        Ok(path)
    }
}

fn detect_image_extension(bytes: &[u8]) -> AppResult<&'static str> {
    let format = image::guess_format(bytes)?;
    match format {
        ImageFormat::Jpeg => Ok("jpg"),
        ImageFormat::Png => Ok("png"),
        ImageFormat::WebP => Ok("webp"),
        ImageFormat::Gif => Ok("gif"),
        ImageFormat::Avif => Ok("avif"),
        _ => Err(AppError::UnsupportedImageFormat),
    }
}

impl From<image::ImageError> for AppError {
    fn from(_error: image::ImageError) -> Self {
        AppError::UnsupportedImageFormat
    }
}
