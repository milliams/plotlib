use std::path::Path;

pub trait Save {
    fn save<P>(&self, path: P) where P: AsRef<Path>;
}
