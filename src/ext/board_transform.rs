
pub trait BoardTransform {
    fn by_slide(&mut self, other: &Self);
    fn by_rotation(&mut self, other: &Self);
    fn remove(&mut self);
}