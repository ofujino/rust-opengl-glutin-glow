pub trait MatrixState {
    fn new(width: u32, height: u32) -> Self;
    fn update(&mut self, step: f32);
    fn get_world(&self) -> &[f32];
    fn get_view(&self) -> &[f32];
    fn get_projection(&self) -> &[f32];
}
