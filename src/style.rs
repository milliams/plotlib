/*!

Manage how elements should be drawn

*/

pub trait Line {
    fn colour<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>;

    fn get_colour(&self) -> &Option<String>;
}
