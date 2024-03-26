#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Data {
    pos: (usize, usize),
    attr: [f64; 2],
    label: u8,
}

impl Data {
    pub fn new(pos: (usize, usize), attr: [f64; 2], label: u8) -> Self {
        Self { pos, attr, label }
    }

    pub fn pos(self) -> (usize, usize) {
        self.pos
    }

    pub fn set_pos(&mut self, new_pos: (usize, usize)) {
        self.pos = new_pos
    }

    pub fn label(self) -> u8 {
        self.label
    }

    pub fn euclidian_distance(self, other: Data) -> f64 {
        let dist: f64 =
            (self.attr[0] - other.attr[0]).powi(2) + (self.attr[1] - other.attr[1]).powi(2);

        dist.sqrt()
    }
}
