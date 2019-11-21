use super::Rect;

/// The four different directions a Turn can point
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// A vertice along a PlanePath
pub struct Turn {
    x: f64,
    y: f64,
    direction: Direction,
}

impl Turn {
    /// Construct a new Turn
    pub fn new(pos: (f64, f64), direction: Direction) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            direction,
        }
    }

    /// Returns the Direction that the Turn points
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// Returns true if the referenced Rect passes into the turn
    pub fn touching(&self, rect: &Rect) -> bool {
        rect.x() > self.x - 10.0
            && rect.x() < self.x + 10.0
            && rect.y() > self.y - 10.0
            && rect.y() < self.y + 10.0
    }
}

/// A set of Turns for a Plane to follow
pub struct PlanePath {
    turns: Vec<Turn>,
}

impl PlanePath {
    /// Construct a new PlanePath
    pub fn new(turns: Vec<Turn>) -> Self {
        Self { turns }
    }

    pub fn new_main_path(width: f64, height: f64) -> Self {
        Self::new(vec![
            Turn::new((width * 0.22, height * 0.26), Direction::Down),
            Turn::new((width * 0.22, height * 0.72), Direction::Right),
            Turn::new((width * 0.43, height * 0.72), Direction::Up),
            Turn::new((width * 0.43, height * 0.25), Direction::Right),
            Turn::new((width * 0.62, height * 0.25), Direction::Down),
            Turn::new((width * 0.62, height * 0.72), Direction::Right),
            Turn::new((width * 0.85, height * 0.72), Direction::Up),
            Turn::new((width * 0.85, height * 0.26), Direction::Right),
        ])
    }

    /// Returns a reference to the Turns
    pub fn turns(&self) -> &[Turn] {
        &self.turns
    }
}
