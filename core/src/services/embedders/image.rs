use crate::config::ClipModelConfig;
use anyhow::{Context, Error, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::clip;
use rayon::prelude::*;
use std::path::Path;

#[cfg_attr(test, mockall::automock)]
pub trait ImageEmbedder {
    /// Computes embeddings for a batch of images.
    fn embed(&self, image_paths: &Vec<String>) -> Result<Vec<Vec<f32>>>;
}

pub struct ClipImageEmbedder {
    config: clip::ClipConfig,
    model: clip::ClipModel,
    device: Device,
}

impl ClipImageEmbedder {
    pub fn new(clip_config: &ClipModelConfig) -> Result<Self> {
        let device = match clip_config.device.as_str() {
            "cpu" => Device::Cpu,
            "cuda" => Device::new_cuda(0)?,
            _ => Device::Cpu,
        };
        let config = clip::ClipConfig::vit_base_patch32();
        let model_path = Path::new(&clip_config.dir).join(&clip_config.safetensors_file);

        if !model_path.exists() {
            return Err(anyhow::anyhow!(
                "Model file not found: {}",
                model_path.display()
            ));
        }

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[&model_path], DType::F32, &device)
                .map_err(Error::from)
                .context("Failed to load model safetensors")?
        };
        let model = clip::ClipModel::new(vb, &config)
            .map_err(Error::from)
            .context("Failed to create CLIP model")?;

        Ok(Self {
            config,
            model,
            device,
        })
    }
}

impl ImageEmbedder for ClipImageEmbedder {
    /// Computes embeddings for a batch of images.
    fn embed(&self, image_paths: &Vec<String>) -> Result<Vec<Vec<f32>>> {
        let images = load_images(image_paths, self.config.image_size, &self.device)
            .context("Failed to load images")?;
        let tensors = self
            .model
            .get_image_features(&images)
            .map_err(Error::from)
            .context("Failed to compute image features")?;
        let embeddings = tensors
            .to_vec2::<f32>()
            .map_err(Error::from)
            .context("Failed to convert tensors to embeddings")?;
        Ok(embeddings)
    }
}

fn load_heic_image_data<T: AsRef<std::path::Path>>(path: T, image_size: usize) -> Result<Vec<u8>> {
    use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};

    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_file(
        path.as_ref()
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid path"))?,
    )
    .context("Failed to read HEIC file")?;
    let handle = ctx
        .primary_image_handle()
        .context("Failed to get primary image handle")?;

    let image = lib_heif
        .decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)
        .context("Failed to decode HEIC image")?;

    let resized_image = image
        .scale(image_size as u32, image_size as u32, None)
        .context("Failed to resize HEIC image")?;

    let planes = resized_image.planes();
    let interleaved_plane = planes
        .interleaved
        .ok_or_else(|| anyhow::anyhow!("No interleaved plane available"))?;
    let img_data = interleaved_plane.data.to_vec();

    Ok(img_data)
}

fn load_image_data<T: AsRef<std::path::Path>>(path: T, image_size: usize) -> Result<Vec<u8>> {
    let img = image::ImageReader::open(&path)
        .context("Failed to open image file")?
        .decode()
        .context("Failed to decode image")?;
    let img = img.resize_to_fill(
        image_size as u32,
        image_size as u32,
        image::imageops::FilterType::Triangle,
    );
    let img = img.to_rgb8();
    let img_data = img.into_raw();
    Ok(img_data)
}

fn load_images<T: AsRef<std::path::Path> + Sync>(
    paths: &Vec<T>,
    image_size: usize,
    device: &Device,
) -> Result<Tensor> {
    let image_data: Vec<Result<Vec<u8>, anyhow::Error>> = paths
        .par_iter()
        .map(|path| -> Result<Vec<u8>, anyhow::Error> {
            if path
                .as_ref()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.to_lowercase())
                == Some("heic".to_string())
            {
                load_heic_image_data(path, image_size)
            } else {
                load_image_data(path, image_size)
            }
        })
        .collect();

    let mut processed_data = Vec::new();
    for result in image_data {
        match result {
            Ok(data) => processed_data.push(data),
            Err(e) => return Err(e.context("Failed to process image data")),
        }
    }

    let mut images = Vec::new();
    for data in processed_data {
        let img = (|| {
            Tensor::from_vec(data, (image_size, image_size, 3), &device)?
                .permute((2, 0, 1))?
                .to_dtype(DType::F32)?
                .affine(2. / 255., -1.)
                .map_err(Error::from)
        })()
        .context("Failed to create tensor from image data")?;
        images.push(img);
    }

    let stacked_images = Tensor::stack(&images, 0)
        .map_err(Error::from)
        .context("Failed to stack image tensors")?;
    Ok(stacked_images)
}
