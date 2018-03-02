/*!
The `page` module provides structures for laying out and rendering multiple views.
*/

use std::path::Path;
use std::ffi::OsStr;

use svg;
use svg::Node;
use svg::Document;

use view::ContinuousView;

/**
A single page page laying out the views in a grid
*/
pub struct Page<'a> {
    views: Vec<&'a ContinuousView<'a>>,
    num_views: u32,
}

impl<'a> Page<'a> {
    /**
    Creates a plot containing a single view
    */
    pub fn single(view: &'a ContinuousView) -> Self {
        Page {
            views: vec![view],
            num_views: 1,
        }
    }

    /**
    Render the plot to an svg document
    */
    pub fn to_svg(&self) -> svg::Document {
        let mut document = Document::new().set("viewBox", (0, 0, 600, 400));
        // TODO put multiple views in correct places
        for &view in &self.views {
            let view_group = view.to_svg(500., 340.)
                .set("transform", format!("translate({}, {})", 50, 360));
            document.append(view_group);
        }
        document
    }

    /**
    Render the plot to an `String`
    */
    pub fn to_text(&self) -> String {
        // TODO compose multiple views into a plot
        let view = self.views[0];
        view.to_text(90, 30)
    }

    /**
    Save the plot to a file.

    The type of file will be based on the file extension.
    */

    pub fn save<P>(&self, path: P)
    where
        P: AsRef<Path>,
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
