use std::sync::LazyLock;

use base64::engine::{Engine, general_purpose::STANDARD};
use image::{codecs::jpeg::JpegEncoder, imageops::FilterType};
use tokio::fs;
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    repositories::file_system::DATA_PATH,
};

const MAX_DIMENSION: u32 = 1920;

static IMAGES_PATH: LazyLock<String> = LazyLock::new(|| format!("{}/images", DATA_PATH));
static RECIPES_PATH: LazyLock<String> = LazyLock::new(|| format!("{}/recipes", *IMAGES_PATH));

pub struct Images;

impl Images {
    pub async fn save_recipe_image(image_data: String) -> AppResult<String> {
        fs::create_dir_all(&*RECIPES_PATH).await?;
        let input_bytes = decode_base64(&image_data)?;
        let output_bytes = process_image(&input_bytes)?;
        let filename = format!("{}.jpg", Uuid::now_v7());
        let path = format!("{}/{}", *RECIPES_PATH, filename);
        fs::write(&path, output_bytes).await?;
        let path_as_root = format!("/{}", path);
        Ok(path_as_root)
    }
}

fn process_image(bytes: &[u8]) -> AppResult<Vec<u8>> {
    let mut img = image::load_from_memory(bytes)?;
    if img.width() > MAX_DIMENSION || img.height() > MAX_DIMENSION {
        img = img.resize(MAX_DIMENSION, MAX_DIMENSION, FilterType::Lanczos3);
    }
    let img = img.to_rgb8();
    let mut output = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut output, 85);
    encoder.encode_image(&img)?;
    Ok(output)
}

impl From<image::ImageError> for AppError {
    fn from(_error: image::ImageError) -> Self {
        AppError::UnsupportedImageFormat
    }
}

fn decode_base64(input: &str) -> AppResult<Vec<u8>> {
    let payload = if input.starts_with("data:") {
        input.split_once(',').ok_or(AppError::InvalidData)?.1
    } else {
        input
    };
    Ok(STANDARD.decode(payload)?)
}
