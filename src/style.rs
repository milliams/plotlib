//! Manage how elements should be drawn

//! All style structs follows the 'optional builder' pattern:
//! Each field is a `Option` which start as `None`.
//! They can all be set with setter methods, and instances
//! can be overlaid with another one to set many at once.
//! Settings will be cloned in and out of it.

/**
The style that line corners should use
*/
#[derive(Debug, Clone)]
pub enum LineJoin {
    Miter,
    Round,
}

#[derive(Debug, Default)]
pub struct LineStyle {
    colour: Option<String>,
    width: Option<f32>,
    linejoin: Option<LineJoin>,
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
            self.linejoin = Some(v.clone())
        }
    }
    pub fn colour<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.colour = Some(value.into());
        self
    }

    pub fn get_colour(&self) -> &Option<String> {
        &self.colour
    }

    pub fn width<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<f32>,
    {
        self.width = Some(value.into());
        self
    }

    pub fn get_width(&self) -> &Option<f32> {
        &self.width
    }

    pub fn linejoin<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<LineJoin>,
    {
        self.linejoin = Some(value.into());
        self
    }

    pub fn get_linejoin(&self) -> &Option<LineJoin> {
        &self.linejoin
    }
}

/**
The marker that should be used for the points of the scatter plot
*/
#[derive(Debug, Clone)]
pub enum PointMarker {
    Circle,
    Square,
    Cross,
}


#[derive(Debug, Default)]
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
            self.marker = Some(v.clone())
        }

        if let Some(ref v) = other.colour {
            self.colour = Some(v.clone())
        }

        if let Some(v) = other.size {
            self.size = Some(v)
        }
    }
    pub fn marker<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<PointMarker>,
    {
        self.marker = Some(value.into());
        self
    }

    pub fn get_marker(&self) -> &Option<PointMarker> {
        &self.marker
    }

    pub fn colour<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.colour = Some(value.into());
        self
    }

    pub fn get_colour(&self) -> &Option<String> {
        &self.colour
    }

    pub fn size<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<f32>,
    {
        self.size = Some(value.into());
        self
    }

    pub fn get_size(&self) -> &Option<f32> {
        &self.size
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

    pub fn fill<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.fill = Some(value.into());
        self
    }

    pub fn get_fill(&self) -> &Option<String> {
        &self.fill
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linestyle_simple() {
        let s = LineStyle::new();
        assert_eq!(*s.get_colour(), None);
        assert_eq!(*s.get_width(), None);
        assert!(match s.get_linejoin() {
            None => true,
            _ => false,
        });
    }

    #[test]
    fn test_linestyle_plain_overlay() {
        let mut p = LineStyle::new();
        p.overlay(LineStyle::new().colour("red").linejoin(LineJoin::Miter).width(1.));
        assert_eq!(*p.get_colour(), Some("red".into()));
        assert_eq!(*p.get_width(), Some(1.));
        assert!(match p.get_linejoin() {
            Some(LineJoin::Miter) => true,
            _ => false,
        });
    }
}
