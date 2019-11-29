/*!
The `page` module provides structures for laying out and rendering multiple views.
*/

use std::ffi::OsStr;
use std::path::Path;

use svg;
use svg::node::element::{Definitions, Marker, Polygon};
use svg::Document;
use svg::Node;

use crate::errors::Result;
use crate::view::View;

use failure::ResultExt;

/**
A single page page laying out the views in a grid
*/
pub struct Page<'a> {
    views: Vec<(u8, u8, &'a dyn View)>,
    margin: (u16, u16),
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
            margin: (10, 10),
            num_views: 0,
            dimensions: (600, 400),
        }
    }

    /**
    Creates a plot containing a single view
    */
    pub fn single(view: &'a dyn View) -> Self {
        Page::empty().add_plot(view, 0, 0)
    }

    /// Set the dimensions of the plot.
    pub fn dimensions(mut self, x: u32, y: u32) -> Self {
        self.dimensions = (x, y);
        self
    }

    /// Set the margins of the plot.
    pub fn margin(mut self, x: u16, y: u16) -> Self {
        self.margin = (x, y);
        self
    }

    /// Add a view to the plot
    pub fn add_plot(mut self, view: &'a dyn View, row: u8, column: u8) -> Self {
        self.views.push((row, column, view));
        self.num_views += 1;
        self
    }

    /**
    Render the plot to an svg document
    */
    pub fn to_svg(&self) -> Result<svg::Document> {
        let (width, height) = self.dimensions;

        let rows = self
            .views
            .iter()
            .fold(0, |acc, f| if acc < f.0 { f.0 } else { acc })
            + 1;

        let columns = self
            .views
            .iter()
            .fold(0, |acc, f| if acc < f.1 { f.1 } else { acc })
            + 1;

        let width_per_view = width as f64 / rows as f64 - (self.margin.0 as f64 * 2.);
        let height_per_view = height as f64 / columns as f64 - (self.margin.1 as f64 * 2.);

        // Add the markers to create the arrow like axis
        let mut document = Document::new().set("viewBox", (0, 0, width, height)).add(
            Definitions::new().add(
                Marker::new()
                    .set("id", "arrowhead")
                    .set("markerWidth", "10")
                    .set("markerHeight", "7")
                    .set("refX", 0)
                    .set("refY", 3.5)
                    .set("orient", "auto")
                    .add(Polygon::new().set("points", "0 0, 10 3.5, 0 7")),
            ),
        );

        // TODO put multiple views in correct places
        for &view in &self.views {
            let view_group = view.2.to_svg(width_per_view, height_per_view)?.set(
                "transform",
                format!(
                    "translate({}, {})",
                    width_per_view * view.0 as f64
                        + self.margin.0 as f64 * if view.0 != 0 { 2. } else { 1. },
                    height_per_view * view.1 as f64
                        + self.margin.1 as f64 * if view.1 != 0 { 2. } else { 1. }
                ),
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
        view.2.to_text(width, height)
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
