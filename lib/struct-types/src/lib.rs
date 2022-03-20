pub mod broom;
pub mod node;
pub mod queue;

#[derive(Debug, PartialEq)]
pub struct GrayscaleMap {
    pub pixels: Vec<u8>,
    pub size: (usize, usize),
}

impl GrayscaleMap {
    /// GrayScaleMapを初期化する
    /// ```
    /// assert_eq!(1024 * 1024, struct_types::GrayscaleMap::new((1024, 1024)).unwrap().pixels.len());
    /// ```
    ///
    /// サイズに指定したいずれかが0の場合Noneを返す
    /// ```
    /// assert_eq!(None, struct_types::GrayscaleMap::new((0, 1024)));
    /// assert_eq!(None, struct_types::GrayscaleMap::new((1024, 0)));
    /// assert_eq!(None, struct_types::GrayscaleMap::new((0, 0)));
    /// ```
    pub fn new(size: (usize, usize)) -> Option<GrayscaleMap> {
        match size {
            (width, height) if width == 0 || height == 0 => None,
            (width, height) => Some(GrayscaleMap {
                pixels: vec![0; width * height],
                size,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_new_image() {
        let width = 1024;
        let height = 576;
        let image = GrayscaleMap {
            pixels: vec![0; width * height],
            size: (width, height),
        };

        assert_eq!((width, height), image.size);
        assert_eq!(width * height, image.pixels.len());
    }

    #[test]
    fn note_empty_image() {
        let image = GrayscaleMap::new((1024, 1024));
        assert_eq!(1024 * 1024, image.as_ref().unwrap().pixels.len());
        assert_eq!((1024usize, 1024usize), image.as_ref().unwrap().size);
    }

    #[test]
    fn zero_width() {
        let image = GrayscaleMap::new((0, 1024));
        assert_eq!(None, image);
    }

    #[test]
    fn zero_height() {
        let image = GrayscaleMap::new((1024, 0));
        assert_eq!(None, image);
    }

    #[test]
    fn zero() {
        let image = GrayscaleMap::new((0, 0));
        assert_eq!(None, image);
    }
}
