extern crate sdl2;

fn main() {
  sdl2::init(sdl2::InitVideo);

  let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::PosCentered, sdl2::video::PosCentered, 800, 600, sdl2::video::OpenGL) {
    Ok(window) => window,
    Err(err) => fail!(format!("failed to create window: {}", err))
  };

  let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::Accelerated) {
    Ok(renderer) => renderer,
    Err(err) => fail!(format!("failed to create renderer: {}", err))
  };

  let _ = renderer.set_draw_color(sdl2::pixels::RGB(255, 0, 0));
  let _ = renderer.clear();
  renderer.present();

  sdl2::timer::delay(2000);
  sdl2::quit();

  println!("Hello, world!")
}
