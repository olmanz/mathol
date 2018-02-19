pub trait Cotangent {
    fn cot(self) -> f64;
}

impl Cotangent for f64 {
    fn cot(self) -> f64 {
        println!("self: {}", self);
        self.cos() / self.sin()
    }
}