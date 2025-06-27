use anyhow::{Context, Error, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::clip;
use mockall::automock;
use tokenizers::Tokenizer;

#[automock]
pub trait TextEmbedder {
    /// Computes embeddings for a given text.
    fn embed(&self, text: &str) -> Result<Vec<f32>>;
}

pub struct ClipTextEmbedder {
    config: clip::ClipConfig,
    model: clip::ClipModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl ClipTextEmbedder {
    pub fn new() -> Result<Self> {
        let config = clip::ClipConfig::vit_base_patch32();
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&["model.safetensors"], DType::F32, &Device::Cpu)
                .context("Failed to load model safetensors")?
        };
        let tokenizer = Tokenizer::from_file("tokenizer.json")
            .map_err(Error::from_boxed)
            .context("Failed to open tokenizer")?;
        let model = clip::ClipModel::new(vb, &config).context("Failed to load model")?;
        Ok(Self {
            config,
            model,
            tokenizer,
            device: Device::Cpu,
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
