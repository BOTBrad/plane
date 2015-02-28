extern crate sdl2;

use sdl2::video::{
  self,
  WindowPos,
  Window,
};
use sdl2::render::{
  self,
  Renderer,
  RenderDriverIndex,
};
use sdl2::pixels::Color;

use data;
use std::vec::Vec;

pub struct Graphics {
  renderer: sdl2::render::Renderer
}

impl Graphics {
  pub fn new((width, height): (i32, i32)) -> Graphics {
    sdl2::init(sdl2::INIT_VIDEO);

    let window = match Window::new("plane", WindowPos::PosCentered, WindowPos::PosCentered, width, height, video::OPENGL) {
      Ok(new_window) => new_window,
      Err(err) => panic!(format!("failed to create window: {}", err))
    };

    let renderer = match Renderer::from_window(window, RenderDriverIndex::Auto, render::ACCELERATED) {
      Ok(renderer) => renderer,
      Err(err) => panic!(format!("failed to create renderer: {}", err))
    };
    Graphics {
      renderer: renderer
    }
  }
  pub fn update(&self, render_list: Vec<data::Rect>) {
    let mut drawer = self.renderer.drawer();

    let _ = drawer.set_draw_color(Color::RGB(0, 0, 0));
    let _ = drawer.clear();
    let _ = drawer.set_draw_color(Color::RGB(255, 255, 255));

    for rect in render_list.iter() {
      let _ = drawer.fill_rect(sdl2::rect::Rect{x: rect.x, y: rect.y, w: rect.w, h: rect.h});
    }

    drawer.present();
  }
  pub fn quit(&self) {
    sdl2::quit();
  }
}

// renderable
pub trait Renderable {
  fn render(&self) -> data::Rect;
}

