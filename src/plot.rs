use std::path::Path;
use std::ffi::OsStr;

use svg;
use svg::Node;
use svg::Document;

use view::View;

pub struct Plot<'a> {
    pub views: Vec<&'a View<'a>>,
    num_views: u32,
}

impl<'a> Plot<'a> {
    pub fn single(view: &'a View) -> Self {
        Plot {
            views: vec![view],
            num_views: 1,
        }
    }

    pub fn to_svg(&self) -> svg::Document {
        let mut document = Document::new().set("viewBox", (0, 0, 600, 400));
        for &view in self.views.iter() {
            let view_group = view.to_svg(500., 350.)
                .set("transform", format!("translate({}, {})", 50, 370));
            document.append(view_group);
        }
        document
    }

    pub fn to_text(&self) -> String {
        // TODO compose multiple views into a plot
        let view = self.views[0];
        view.to_text(90, 30)
    }

    pub fn save<P>(&self, path: P)
        where P: AsRef<Path>
    {
        match path.as_ref().extension().and_then(OsStr::to_str) {
            Some("svg") => {
                svg::save(path, &self.to_svg()).unwrap();
            }
            _ => {
                // some default
            }
        }
    }
}
