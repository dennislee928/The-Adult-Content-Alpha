//! 使用 img_hash 計算 Perceptual Hash，輸出為 hex 字串供 Kafka/DB 比對。

use image::ImageReader;
use img_hash::{HashAlg, HasherConfig};
use std::io::Cursor;

/// 從圖片 bytes 計算 pHash，回傳 16 進位字串；失敗回傳 None。
pub fn compute_phash_hex(bytes: &[u8]) -> Option<String> {
    let image = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .ok()?
        .decode()
        .ok()?;

    let hasher = HasherConfig::with_bytes_type::<[u8; 8]>()
        .hash_alg(HashAlg::Gradient)
        .hash_size(8, 8)
        .to_hasher();
    let hash = hasher.hash_image(&image);
    Some(hex::encode(hash.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phash_empty_fails() {
        assert!(compute_phash_hex(&[]).is_none());
    }

    #[test]
    fn test_phash_invalid_image_returns_none() {
        assert!(compute_phash_hex(b"not an image").is_none());
    }
}
