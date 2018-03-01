/*!

Manage how elements should be drawn

*/

pub trait Line {
    fn colour<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>;

    fn get_colour(&self) -> &Option<String>;
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
}
