use image::Rgb;

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    attr: [f64; 2],
    label: u8,
    color: Rgb<u8>
}

impl Data {
    pub fn new(attr: [f64; 2], label: u8, color: Rgb<u8>) -> Self {
        Self {
            attr,
            label,
            color
        }
    }

    pub fn euclidian_distance(self, other: Data) -> f64 {
        let mut dist = 0.0;
        for (i, num) in self.attr.into_iter().enumerate() {
            dist += (num - other.attr[i]).powi(2); 
        } 
        dist.sqrt()
    }
}