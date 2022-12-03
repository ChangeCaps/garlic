use std::fmt::Display;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Direction {
    #[default]
    Column,
    Row,
}

impl Direction {
    #[inline]
    pub const fn is_vertical(&self) -> bool {
        matches!(self, Direction::Column)
    }

    #[inline]
    pub const fn is_horizontal(&self) -> bool {
        matches!(self, Direction::Row)
    }

    #[inline]
    pub const fn is_row(&self) -> bool {
        self.is_horizontal()
    }

    #[inline]
    pub const fn is_column(&self) -> bool {
        self.is_vertical()
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Column => write!(f, "column"),
            Direction::Row => write!(f, "row"),
        }
    }
}
