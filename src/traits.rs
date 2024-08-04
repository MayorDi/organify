pub trait Render {
    fn render_init(&mut self) {
        unimplemented!()
    }
    fn render(&self);
}

pub trait Behavior {
    fn update(&mut self);
}
