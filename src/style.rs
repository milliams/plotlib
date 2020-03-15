//! Manage how elements should be drawn

//! All style structs follows the 'optional builder' pattern:
//! Each field is a `Option` which start as `None`.
//! They can all be set with setter methods, and instances
//! can be overlaid with another one to set many at once.
//! Settings will be cloned in and out of it.

/// The style that line corners should use
#[derive(Debug, Clone, Copy)]
pub enum LineJoin {
    Miter,
    Round,
}

#[derive(Debug, Default, Clone)]
pub struct LineStyle {
    pub colour: Option<String>,
    pub width: Option<f32>,
    pub linejoin: Option<LineJoin>,
}
impl LineStyle {
    pub fn new() -> Self {
        LineStyle {
            colour: None,
            width: None,
            linejoin: None,
        }
    }

    pub fn overlay(&mut self, other: &Self) {
        if let Some(ref v) = other.colour {
            self.colour = Some(v.clone())
        }

        if let Some(ref v) = other.width {
            self.width = Some(*v)
        }

        if let Some(ref v) = other.linejoin {
            self.linejoin = Some(*v)
        }
    }
    pub fn colour<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.colour = Some(value.into());
        self
    }
    pub fn get_colour(&self) -> String {
        self.colour.clone().unwrap_or_else(|| "black".into())
    }

    pub fn width<T>(mut self, value: T) -> Self
    where
        T: Into<f32>,
    {
        self.width = Some(value.into());
        self
    }
    pub fn get_width(&self) -> f32 {
        self.width.unwrap_or_else(|| 2.0)
    }

    pub fn linejoin<T>(mut self, value: T) -> Self
    where
        T: Into<LineJoin>,
    {
        self.linejoin = Some(value.into());
        self
    }
    pub fn get_linejoin(&self) -> LineJoin {
        self.linejoin.unwrap_or_else(|| LineJoin::Round)
    }
}

/// The marker that should be used for the points of the scatter plot
#[derive(Debug, Clone, Copy)]
pub enum PointMarker {
    Circle,
    Square,
    Cross,
}


#[derive(Debug, Default, Clone)]
pub struct PointStyle {
    marker: Option<PointMarker>,
    colour: Option<String>,
    size: Option<f32>,
}
impl PointStyle {
    pub fn new() -> Self {
        PointStyle {
            marker: None,
            colour: None,
            size: None,
        }
    }

    pub fn overlay(&mut self, other: &Self) {
        if let Some(ref v) = other.marker {
            self.marker = Some(*v)
        }

        if let Some(ref v) = other.colour {
            self.colour = Some(v.clone())
        }

        if let Some(v) = other.size {
            self.size = Some(v)
        }
    }
    pub fn marker<T>(mut self, value: T) -> Self
    where
        T: Into<PointMarker>,
    {
        self.marker = Some(value.into());
        self
    }
    pub fn get_marker(&self) -> PointMarker {
        self.marker.unwrap_or(PointMarker::Circle)
    }

    pub fn colour<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.colour = Some(value.into());
        self
    }
    pub fn get_colour(&self) -> String {
        self.colour.clone().unwrap_or_else(|| "".into())
    }

    pub fn size<T>(mut self, value: T) -> Self
    where
        T: Into<f32>,
    {
        self.size = Some(value.into());
        self
    }
    pub fn get_size(&self) -> f32 {
        self.size.unwrap_or(5.0)
    }
}


#[derive(Debug, Default)]
pub struct BoxStyle {
    fill: Option<String>,
}
impl BoxStyle {
    pub fn new() -> Self {
        BoxStyle { fill: None }
    }

    pub fn overlay(&mut self, other: &Self) {
        if let Some(ref v) = other.fill {
            self.fill = Some(v.clone())
        }
    }

    pub fn fill<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.fill = Some(value.into());
        self
    }
    pub fn get_fill(&self) -> String {
        self.fill.clone().unwrap_or_else(|| "".into())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linestyle_plain_overlay() {
        let mut p = LineStyle::new();
        p.overlay(&LineStyle::new().colour("red").linejoin(LineJoin::Miter).width(1.));
        assert_eq!(p.get_colour(), "red".to_string());
        assert_eq!(p.get_width(), 1.);
        if let LineJoin::Miter = p.get_linejoin() {
        } else {
            panic!()
        }
    }
}
