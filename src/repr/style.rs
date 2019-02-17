
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

    pub fn fill<T: Into<String>>(&mut self, value: T) -> &mut Self {
        self.fill = Some(value.into());
        self
    }

    pub fn get_fill(&self) -> &Option<String> {
        &self.fill
    }
}




#[derive(Debug, Default)]
pub struct LineStyle {
    colour: Option<String>,
    width: Option<f32>,
}

impl LineStyle {
    pub fn new() -> Self {
        LineStyle {
            colour: None,
            width: None,
        }
    }

    pub fn overlay(&mut self, other: &Self) {
        if let Some(ref v) = other.colour {
            self.colour = Some(v.clone())
        }

        if let Some(ref v) = other.width {
            self.width = Some(*v)
        }
    }
}

impl LineStyle {
    pub fn colour<T: Into<String>>(&mut self, value: T) -> &mut Self {
        self.colour = Some(value.into());
        self
    }

    pub fn get_colour(&self) -> &Option<String> {
        &self.colour
    }

    pub fn width<T: Into<f32>>(&mut self, value: T) -> &mut Self {
        self.width = Some(value.into());
        self
    }

    pub fn get_width(&self) -> &Option<f32> {
        &self.width
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
/// `PointStyle` follows the 'optional builder' pattern
/// Each field is a `Option` which start as `None`
/// Each can be set with setter methods and instances
/// of `PointStyle` can be overlaid to set many at once.
/// Settings will be cloned in and out of it.
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

    pub fn marker<T: Into<PointMarker>>(&mut self, value: T) -> &mut Self {
        self.marker = Some(value.into());
        self
    }

    pub fn get_marker(&self) -> &Option<PointMarker> {
        &self.marker
    }

    pub fn colour<T: Into<String>>(&mut self, value: T) -> &mut Self {
        self.colour = Some(value.into());
        self
    }

    pub fn get_colour(&self) -> &Option<String> {
        &self.colour
    }

    pub fn size<T: Into<f32>>(&mut self, value: T) -> &mut Self {
        self.size = Some(value.into());
        self
    }

    pub fn get_size(&self) -> &Option<f32> {
        &self.size
    }
}
