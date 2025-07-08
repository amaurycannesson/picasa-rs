use image::{DynamicImage, GenericImageView, ImageFormat};
use lru::LruCache;
use std::num::NonZero;
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;

pub struct ImageService {
    // LRU cache for thumbnails in memory
    thumbnail_cache: RwLock<LruCache<String, Vec<u8>>>,
    // Cache directory for generated thumbnails
    cache_dir: PathBuf,
    // Thumbnail sizes
    sizes: ImageSizes,
}

#[derive(Clone)]
pub struct ImageSizes {
    pub thumbnail: (u32, u32), // 150x150 for grid view
    pub preview: (u32, u32),   // 400x400 for detail view
    pub medium: (u32, u32),    // 800x600 for modal view
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl ImageService {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            thumbnail_cache: RwLock::new(LruCache::new(NonZero::new(500).unwrap())), // Keep 500 thumbnails in memory
            cache_dir,
            sizes: ImageSizes {
                thumbnail: (150, 150),
                preview: (400, 400),
                medium: (800, 600),
            },
        }
    }

    // Generate and cache thumbnail if it doesn't exist
    pub async fn get_thumbnail(&self, photo_path: &str) -> anyhow::Result<Vec<u8>> {
        let cache_key = format!("thumb_{}", self.hash_path(photo_path));

        // Check memory cache first
        {
            let cache = self.thumbnail_cache.read().await;
            if let Some(data) = cache.peek(&cache_key) {
                return Ok(data.clone());
            }
        }

        // Check disk cache
        let thumb_path = self.cache_dir.join(format!("{}.webp", cache_key));
        if thumb_path.exists() {
            let data = tokio::fs::read(&thumb_path).await?;

            // Store in memory cache
            {
                let mut cache = self.thumbnail_cache.write().await;
                cache.put(cache_key, data.clone());
            }

            return Ok(data);
        }

        // Generate thumbnail
        let data = self
            .generate_thumbnail(photo_path, self.sizes.thumbnail)
            .await?;

        // Save to disk cache
        tokio::fs::create_dir_all(&self.cache_dir).await?;
        tokio::fs::write(&thumb_path, &data).await?;

        // Store in memory cache
        {
            let mut cache = self.thumbnail_cache.write().await;
            cache.put(cache_key, data.clone());
        }

        Ok(data)
    }

    async fn generate_thumbnail(
        &self,
        photo_path: &str,
        size: (u32, u32),
    ) -> anyhow::Result<Vec<u8>> {
        let img = self.load_image(photo_path)?;
        let thumbnail = img.thumbnail(size.0, size.1);

        let mut buffer = Vec::new();
        thumbnail.write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::WebP)?;

        Ok(buffer)
    }

    fn load_image(&self, photo_path: &str) -> anyhow::Result<DynamicImage> {
        let path = Path::new(photo_path);
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "heif" | "heic" => self.load_heif_image(photo_path),
            _ => Ok(image::open(photo_path)?),
        }
    }

    fn load_heif_image(&self, photo_path: &str) -> anyhow::Result<DynamicImage> {
        use anyhow::Context;
        use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};

        let lib_heif = LibHeif::new();
        let ctx = HeifContext::read_from_file(photo_path).context("Failed to read HEIC file")?;
        let handle = ctx
            .primary_image_handle()
            .context("Failed to get primary image handle")?;

        let width = handle.width();
        let height = handle.height();

        let image = lib_heif
            .decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)
            .context("Failed to decode HEIC image")?;

        let planes = image.planes();
        let interleaved_plane = planes
            .interleaved
            .ok_or_else(|| anyhow::anyhow!("No interleaved plane available"))?;
        let img_data = interleaved_plane.data.to_vec();

        let image_buffer = image::RgbImage::from_raw(width, height, img_data)
            .ok_or_else(|| anyhow::anyhow!("Failed to create image buffer from HEIF data"))?;

        Ok(DynamicImage::ImageRgb8(image_buffer))
    }

    pub fn crop_image(&self, photo_path: String, bbox: BoundingBox) -> anyhow::Result<Vec<u8>> {
        let img = self.load_image(&photo_path)?;

        // Ensure coordinates are within image bounds
        let (img_width, img_height) = img.dimensions();
        let x = bbox.x.max(0) as u32;
        let y = bbox.y.max(0) as u32;
        let width = bbox.width.max(0) as u32;
        let height = bbox.height.max(0) as u32;

        // Clamp dimensions to image bounds
        let crop_width = width.min(img_width.saturating_sub(x));
        let crop_height = height.min(img_height.saturating_sub(y));

        if crop_width == 0 || crop_height == 0 {
            return Err(anyhow::anyhow!("Invalid crop dimensions"));
        }

        let cropped = img.crop_imm(x, y, crop_width, crop_height);

        let mut buffer = Vec::new();
        cropped.write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::WebP)?;

        Ok(buffer)
    }

    fn hash_path(&self, path: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}
