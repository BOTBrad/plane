extern crate sdl2;

pub struct Graphics {
  renderer: sdl2::render::Renderer<sdl2::video::Window>
}

impl Graphics {
  pub fn new(width: int, height: int) -> Graphics {
    sdl2::init(sdl2::InitVideo);

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::PosCentered, sdl2::video::PosCentered, width, height, sdl2::video::OpenGL) {
      Ok(newWindow) => newWindow,
      Err(err) => fail!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::Accelerated) {
      Ok(renderer) => renderer,
      Err(err) => fail!(format!("failed to create renderer: {}", err))
    };
    Graphics {
      renderer: renderer
    }
  }
  pub fn update<T: Renderable>(&self, renderable: &T, color: &sdl2::pixels::Color) {
    let rect = renderable.render();
    let _ = self.renderer.set_draw_color(*color);
    let _ = self.renderer.clear();
    let _ = self.renderer.set_draw_color(sdl2::pixels::RGB(255, 255, 255));
    let _ = self.renderer.fill_rect(&sdl2::rect::Rect{x: rect.x, y: rect.y, w: rect.w, h: rect.h});
    self.renderer.present();
  }
  pub fn quit(&self) {
    sdl2::quit();
  }
}

pub struct Rect {
  pub x: i32,
  pub y: i32,
  pub w: i32,
  pub h: i32
}

// renderable
pub trait Renderable {
  fn render(&self) -> Rect;
}
