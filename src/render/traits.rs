use super::RenderContext;

pub trait Render {
    fn render(&self, ctx: &RenderContext);
}
