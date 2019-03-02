pub(crate) enum GridType<'a> {
    HorizontalOnly(&'a Grid),
    Both(&'a Grid),
}

pub struct Grid {
    pub nx: u32,
    pub ny: u32,
    pub color: String,
}

impl Default for Grid {
    fn default() -> Self {
        Grid::new(3, 3)
    }
}

impl Grid {
    pub fn new(nx: u32, ny: u32) -> Grid {
        Grid {
            nx,
            ny,
            color: "darkgrey".to_owned(),
        }
    }
}
