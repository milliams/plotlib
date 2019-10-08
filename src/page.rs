/*!
The `page` module provides structures for laying out and rendering multiple views.
*/

use std::ffi::OsStr;
use std::path::Path;

use svg;
use svg::Document;
use svg::Node;

use crate::errors::Result;
use crate::view::View;

use failure::ResultExt;

/**
A single page page laying out the views in a grid
*/
pub struct Page<'a> {
    views: Vec<&'a dyn View>,
    num_views: u32,
    dimensions: (u32, u32),
}

impl<'a> Page<'a> {
    /**
    Creates an empty page container for plots to be added to
    */
    pub fn empty() -> Self {
        Page {
            views: Vec::new(),
            num_views: 0,
            dimensions: (600, 400),
        }
    }

    /**
    Creates a plot containing a single view
    */
    pub fn single(view: &'a dyn View) -> Self {
        Page::empty().add_plot(view)
    }

    /// Set the dimensions of the plot.
    pub fn dimensions(mut self, x: u32, y: u32) -> Self {
        self.dimensions = (x, y);
        self
    }

    /// Add a view to the plot
    pub fn add_plot(mut self, view: &'a dyn View) -> Self {
        self.views.push(view);
        self.num_views += 1;
        self
    }

    /**
    Render the plot to an svg document
    */
    pub fn to_svg(&self) -> Result<svg::Document> {
        let (width, height) = self.dimensions;
        let mut document = Document::new().set("viewBox", (0, 0, width, height));

        let x_margin = 90; // should actually depend on y-axis label font size
        let y_margin = 60;
        let x_offset = 0.6 * f64::from(x_margin);
        let y_offset = 0.6 * f64::from(y_margin);

        // TODO put multiple views in correct places
        for &view in &self.views {
            let view_group = view
                .to_svg(f64::from(width - x_margin), f64::from(height - y_margin))?
                .set(
                    "transform",
                    format!("translate({}, {})", x_offset, f64::from(height) - y_offset),
                );
            document.append(view_group);
        }
        Ok(document)
    }

    /**
    Render the plot to an `String`
    */
    pub fn to_text(&self) -> Result<String> {
        let (width, height) = self.dimensions;
        // TODO compose multiple views into a plot
        let view = self.views[0];
        view.to_text(width, height)
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
            Some("svg") => svg::save(path, &self.to_svg()?)
                .context("saving svg")
                .map_err(From::from),
            _ => Ok(()),
        }
    }
}
