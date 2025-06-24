use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::clip;
use tokenizers::Tokenizer;

pub struct TextEmbedder {
    config: clip::ClipConfig,
    model: clip::ClipModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl TextEmbedder {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = clip::ClipConfig::vit_base_patch32();
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&["model.safetensors"], DType::F32, &Device::Cpu)?
        };
        let tokenizer = Tokenizer::from_file("tokenizer.json").unwrap();
        let model = clip::ClipModel::new(vb, &config)?;
        Ok(Self {
            config,
            model,
            tokenizer,
            device: Device::Cpu,
        })
    }

    /// Computes embeddings for a given text.
    pub fn embed(&self, text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        let encoding = self.tokenizer.encode(text, true).unwrap();
        let tokens = encoding.get_ids();
        let token_ids = Tensor::new(tokens, &self.device)?.unsqueeze(0)?;
        let text_features = self.model.get_text_features(&token_ids)?;
        let embedding_vec = text_features.squeeze(0)?.to_vec1::<f32>()?;
        Ok(embedding_vec)
    }
}
