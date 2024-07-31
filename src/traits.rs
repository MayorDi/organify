pub trait Render {
    fn render_init(&mut self) {}
    fn render(&self);
}
