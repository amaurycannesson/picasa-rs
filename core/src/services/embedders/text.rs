use crate::config::ClipModelConfig;
use anyhow::{Context, Error, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::clip;
use std::path::Path;
use tokenizers::Tokenizer;

#[cfg_attr(test, mockall::automock)]
pub trait TextEmbedder {
    /// Computes embeddings for a given text.
    fn embed(&self, text: &str) -> Result<Vec<f32>>;
}

pub struct ClipTextEmbedder {
    model: clip::ClipModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl ClipTextEmbedder {
    pub fn new(clip_config: &ClipModelConfig) -> Result<Self> {
        let config = clip::ClipConfig::vit_base_patch32();
        let device = match clip_config.device.as_str() {
            "cpu" => Device::Cpu,
            "cuda" => Device::new_cuda(0)?,
            _ => Device::Cpu,
        };
        let model_path = Path::new(&clip_config.dir).join(&clip_config.safetensors_file);
        let tokenizer_path = Path::new(&clip_config.dir).join(&clip_config.tokenizer_file);

        if !model_path.exists() {
            return Err(anyhow::anyhow!(
                "Model file not found: {}",
                model_path.display()
            ));
        }

        if !tokenizer_path.exists() {
            return Err(anyhow::anyhow!(
                "Tokenizer file not found: {}",
                tokenizer_path.display()
            ));
        }

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[&model_path], DType::F32, &device)
                .context("Failed to load model safetensors")?
        };
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(Error::from_boxed)
            .context("Failed to open tokenizer")?;
        let model = clip::ClipModel::new(vb, &config).context("Failed to load model")?;

        Ok(Self {
            model,
            tokenizer,
            device,
        })
    }
}

impl TextEmbedder for ClipTextEmbedder {
    /// Computes embeddings for a given text.
    fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(Error::from_boxed)?;
        let tokens = encoding.get_ids();
        let token_ids = Tensor::new(tokens, &self.device)?.unsqueeze(0)?;
        let text_features = self.model.get_text_features(&token_ids)?;
        let embedding_vec = text_features.squeeze(0)?.to_vec1::<f32>()?;
        Ok(embedding_vec)
    }
}
