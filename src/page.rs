/*!
The `page` module provides structures for laying out and rendering multiple views.
*/

use std::ffi::OsStr;
use std::path::Path;

use errors::Result;
use svg;
use svg::Document;
use svg::Node;

use failure::ResultExt;

use view::View;

/**
A single page page laying out the views in a grid
*/
pub struct Page<'a> {
    views: Vec<&'a View>,
}

impl<'a> Page<'a> {
    /**
    Creates a plot containing a single view
    */
    pub fn single(view: &'a View) -> Self {
        Page { views: vec![view] }
    }

    /**
    Render the plot to an svg document
    */
    pub fn to_svg(&self) -> Result<svg::Document> {
        let mut document = Document::new().set("viewBox", (0, 0, 600, 400));
        // TODO put multiple views in correct places
        for &view in &self.views {
            let view_group = view.to_svg(500., 340.)
                .context("creating an svg")?
                .set("transform", format!("translate({}, {})", 50, 360));
            document.append(view_group);
        }
        Ok(document)
    }

    /**
    Render the plot to an `String`
    */
    pub fn to_text(&self) -> Result<String> {
        // TODO compose multiple views into a plot
        let view = self.views[0];
        view.to_text(90, 30)
    }

    /**
    Save the plot to a file.

    The type of file will be based on the file extension.
    */

    pub fn save<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        match path.as_ref().extension().and_then(OsStr::to_str) {
            Some("svg") => {
                svg::save(path, &self.to_svg().context("saving plot")?)?;
            }
            _ => {
                // some default
            }
        }

        Ok(())
    }
}
