use crate::scatter;
use crate::histogram;
use crate::text_render;
use crate::svg_render;

pub trait Render {
    fn to_svg(&self) -> svg_render::SVG;
    fn to_text(&self) -> text_render::Text;
}

impl Render for scatter::Scatter {
    fn to_text(&self) -> text_render::Text {
        text_render::Text {data: text_render::draw_scatter(self)}
    }

    fn to_svg(&self) -> svg_render::SVG {
        svg_render::draw_scatter(self)
    }
}

impl Render for histogram::Histogram {
    fn to_text(&self) -> text_render::Text {
        text_render::Text {data: text_render::draw_histogram(self)}
    }

    fn to_svg(&self) -> svg_render::SVG {
        svg_render::draw_histogram(self)
    }
}
