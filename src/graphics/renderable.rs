use data::Rect;

pub type RenderList = Vec<Rect>;

pub trait Renderable {
  fn render(&self) -> RenderList;
}

