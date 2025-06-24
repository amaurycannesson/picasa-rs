use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::clip;
use rayon::prelude::*;

pub struct ImageEmbedder {
    config: clip::ClipConfig,
    model: clip::ClipModel,
    device: Device,
}

impl ImageEmbedder {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let device = Device::Cpu;
        let config = clip::ClipConfig::vit_base_patch32();
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&["model.safetensors"], DType::F32, &device)?
        };
        let model = clip::ClipModel::new(vb, &config)?;
        Ok(Self {
            config,
            model,
            device,
        })
    }

    /// Computes embeddings for a batch of images.
    pub fn embed(
        &self,
        image_paths: &Vec<String>,
    ) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        let images = load_images(image_paths, self.config.image_size, &self.device)?;
        let tensors = self.model.get_image_features(&images)?;
        let embeddings = tensors.to_vec2::<f32>()?;
        Ok(embeddings)
    }
}

fn load_heic_image_data<T: AsRef<std::path::Path>>(
    path: T,
    image_size: usize,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};

    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_file(path.as_ref().to_str().unwrap())?;
    let handle = ctx.primary_image_handle()?;

    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;

    let resized_image = image.scale(image_size as u32, image_size as u32, None)?;

    let planes = resized_image.planes();
    let interleaved_plane = planes.interleaved.ok_or("No interleaved plane available")?;
    let img_data = interleaved_plane.data.to_vec();

    Ok(img_data)
}

fn load_image_data<T: AsRef<std::path::Path>>(
    path: T,
    image_size: usize,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let img = image::ImageReader::open(path)?.decode()?;
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
) -> Result<Tensor, Box<dyn std::error::Error>> {
    let image_data: Vec<Result<Vec<u8>, String>> = paths
        .par_iter()
        .map(|path| -> Result<Vec<u8>, String> {
            let result = if path
                .as_ref()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.to_lowercase())
                == Some("heic".to_string())
            {
                load_heic_image_data(path, image_size)
            } else {
                load_image_data(path, image_size)
            };
            result.map_err(|e| e.to_string())
        })
        .collect();

    let mut processed_data = Vec::new();
    for result in image_data {
        match result {
            Ok(data) => processed_data.push(data),
            Err(e) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))),
        }
    }

    let mut images = Vec::new();
    for data in processed_data {
        let img = Tensor::from_vec(data, (image_size, image_size, 3), &device)?
            .permute((2, 0, 1))?
            .to_dtype(DType::F32)?
            .affine(2. / 255., -1.)?;
        images.push(img);
    }

    let stacked_images = Tensor::stack(&images, 0)?;
    Ok(stacked_images)
}
