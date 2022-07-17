use std::fs::File;
use tiny_skia::Pixmap;

pub struct DivoomAnimationBuilder {
    width: u32,
    height: u32,
    frames: Vec<Pixmap>,
}

// Ctor and basic functions
impl DivoomAnimationBuilder {
    pub fn new(canvas_size: u32) -> DivoomAnimationBuilder {
        DivoomAnimationBuilder {
            width: canvas_size,
            height: canvas_size,
            frames: vec![],
        }
    }

    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    pub fn new_frame(&mut self) -> &mut Pixmap {
        let pixmap = Pixmap::new(self.width, self.height).unwrap();
        self.frames.push(pixmap);
        self.frames.last_mut().unwrap()
    }
}

// Load from GIF files
impl DivoomAnimationBuilder {
    pub fn from_gif(
        gif_file_path: &str
    ) -> std::io::Result<DivoomAnimationBuilder> {
        let mut builder = DivoomAnimationBuilder::new(0);

        let input = File::open(gif_file_path)?;

        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);

        let mut decoder = options.read_info(input).unwrap();
        while let Some(frame) = decoder.read_next_frame().unwrap() {
            if builder.width == 0 {
                builder.width = frame.width();
            } else {
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
            }

            if builder.height == 0 {
                builder.height = frame.height();
            } else {
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
            }

            let frame_pixmap = DivoomAnimationBuilder::convert_gif_frame_to_skia_pixmap(frame);
            builder.frames.push(frame_pixmap);
        }

        Ok(builder)
    }

    fn convert_gif_frame_to_skia_pixmap(frame: &gif::Frame) -> Pixmap {
        let mut frame_pixmap = Pixmap::new(frame.width as u32, frame.height as u32).unwrap();
        assert_eq!(frame_pixmap.data().len(), frame.buffer.len());

        frame_pixmap.data_mut().copy_from_slice(&frame.buffer[0..]);
        frame_pixmap
    }
}

