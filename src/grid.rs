#![deny(missing_docs)]

//! Configure a grid on a plot.
//!
//! Grids allow for easier estimating of data values. This module allows the configuration of grids
//! on plots.
//!
//! Grids are created by creating a `Grid` definition, and adding it to a plot:
//!
//! The grid lines for `plotlib` are rendered
//! _underneath_ the data so as to not detract from the data.
//!
//! # Examples
//!
//! ```rust
//! # use plotlib::view::ContinuousView;
//! use plotlib::grid::Grid;
//! # use plotlib::style::LineStyle;
//! # use plotlib::view::View;
//!
//! # let l1 = plotlib::repr::Line::new(vec![(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)])
//! #    .style(LineStyle::new().colour("burlywood"));
//! // let l1 = Line::new() ...
//! let mut v = ContinuousView::new().add(l1);
//!
//! // 3 vertical lines and 8 horizontal lines
//! v.add_grid(Grid::new(3, 8));
//!
//! // Render plot
//! ```

// Internal type representing the logic of when do we render only horizontal lines, and when do we
// render a full grid
pub(crate) enum GridType<'a> {
    HorizontalOnly(&'a Grid),
    Both(&'a Grid),
}

/// Configuration for the grid on a plot
///
/// Supports changing the number of grid lines for the x and y dimensions.
/// **Note:** for categorical plots, only horizontal lines will be shown.
pub struct Grid {
    /// Number of vertical grid lines (defaults to 3)
    pub nx: u32,
    /// Number of horizontal grid lines (defaults to 3)
    pub ny: u32,
    /// Color of the grid lines (defaults to "darkgrey")
    pub color: String,
}

impl Default for Grid {
    fn default() -> Self {
        Grid::new(3, 3)
    }
}

impl Grid {
    /// Create a new grid with `nx` vertical and `ny` horizontal grid lines
    ///
    /// The default colour is "darkgrey".
    pub fn new(nx: u32, ny: u32) -> Grid {
        Grid {
            nx,
            ny,
            color: "darkgrey".to_owned(),
        }
    }
}
