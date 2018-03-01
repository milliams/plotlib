/*!

Manage how elements should be drawn

*/

pub trait Line {
    fn colour<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>;

    fn get_colour(&self) -> &Option<String>;

    fn width<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<f32>;

    fn get_width(&self) -> &Option<f32>;
}

/**
The marker that should be used for the points of the scatter plot
*/
#[derive(Debug, Clone)]
pub enum Marker {
    Circle,
    Square,
    Cross,
}

pub trait Point {
    fn marker<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Marker>;

    fn get_marker(&self) -> &Option<Marker>;

    fn colour<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>;

    fn get_colour(&self) -> &Option<String>;

    fn size<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<f32>;

    fn get_size(&self) -> &Option<f32>;
}

pub trait Bar {
    fn fill<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>;

    fn get_fill(&self) -> &Option<String>;
}
