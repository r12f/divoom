use crate::{DivoomAPIError, DivoomAPIResult};
use std::path::PathBuf;
use log::debug;
use tiny_skia::Pixmap;

pub(crate) struct DivoomAnimationTemplateRenderer {
    render_opt: usvg::Options,
}

impl DivoomAnimationTemplateRenderer {
    pub fn new(resource_dir: String) -> Self {
        let mut opt = usvg::Options::default();
        opt.resources_dir = Some(PathBuf::from(resource_dir));
        opt.fontdb.load_system_fonts();

        debug!("Loading font completed! {} fonts are loaded.", opt.fontdb.len());
        debug!("All loaded font families: {}.", opt.fontdb.faces().iter().map(|f| f.family).collect());

        DivoomAnimationTemplateRenderer { render_opt: opt }
    }

    pub fn render(&self, svg_text: &str) -> DivoomAPIResult<Pixmap> {
        let rtree = match usvg::Tree::from_str(svg_text, &self.render_opt.to_ref()) {
            Err(e) => {
                return Err(DivoomAPIError::ParameterError(format!(
                    "Parsing SVG failed! Error = {:?}",
                    e
                )))
            }
            Ok(v) => v,
        };

        let pixmap_size = rtree.svg_node().size.to_screen_size();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
        resvg::render(
            &rtree,
            usvg::FitTo::Original,
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        )
        .unwrap();

        Ok(pixmap)
    }
}
