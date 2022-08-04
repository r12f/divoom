use std::path::PathBuf;
use tiny_skia::Pixmap;

pub struct DivoomAnimationTemplateRenderer {
    render_opt: usvg::Options,
}

impl DivoomAnimationTemplateRenderer {
    pub fn new(resource_dir: String) -> Self {
        let mut opt = usvg::Options::default();
        opt.resources_dir = Some(PathBuf::from(resource_dir));
        opt.fontdb.load_system_fonts();

        DivoomAnimationTemplateRenderer { render_opt: opt }
    }

    pub fn render(&self, svg_text: &str) -> Pixmap {
        let rtree = usvg::Tree::from_str(svg_text, &self.render_opt.to_ref()).unwrap();
        let pixmap_size = rtree.svg_node().size.to_screen_size();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
        resvg::render(
            &rtree,
            usvg::FitTo::Original,
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        )
        .unwrap();
        pixmap
    }
}
