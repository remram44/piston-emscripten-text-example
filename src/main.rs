//! Entrypoint and eventloop for SDL client.

extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;

mod render;

use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::input::*;
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window;

/// The application context, passed through the `event_loop` module.
struct App {
    gl: GlGraphics,
    glyph_cache: GlyphCache<'static>,
}

#[cfg(not(target_os = "emscripten"))]
const OPENGL: OpenGL = OpenGL::V3_2;
#[cfg(target_os = "emscripten")]
const OPENGL: OpenGL = OpenGL::V2_1;

/// Entrypoint. Sets up SDL and the event loop.
fn main() {
    let width = 800;
    let height = 600;

    // Create an SDL2 window.
    let window: Sdl2Window =
        WindowSettings::new("vigilant-engine", [width, height])
            .opengl(OPENGL)
            .srgb(false)
            .exit_on_esc(true)
            .build()
            .expect("Couldn't create an OpenGL window");

    let gl = GlGraphics::new(OPENGL);

    let glyph_cache = GlyphCache::new(
        "assets/FiraSans-Regular.ttf",
        (),
        TextureSettings::new(),
    ).unwrap();

    let app = App {
        gl: gl,
        glyph_cache: glyph_cache,
    };

    // Use the event_loop module to handle SDL/Emscripten differences
    event_loop::run(window, handle_event, app);
}

/// Handles a Piston event fed from the `event_loop` module.
fn handle_event(
    _window: &mut Sdl2Window,
    event: Event,
    app: &mut App,
) -> bool {
    // Draw
    if let Some(r) = event.render_args() {
        let glyph_cache = &mut app.glyph_cache;
        app.gl.draw(r.viewport(), |c, g| {
            render::render(c, g, glyph_cache);
        });
    }
    true
}

/// Event loop, factored out for SDL and Emscripten support.
#[cfg(not(target_os = "emscripten"))]
mod event_loop {
    use piston::event_loop::{EventSettings, Events};
    use piston::input::Event;
    use sdl2_window::Sdl2Window;

    pub fn run<T>(
        mut window: Sdl2Window,
        handler: fn(&mut Sdl2Window, Event, &mut T) -> bool,
        mut arg: T,
    ) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            if !handler(&mut window, e, &mut arg) {
                break;
            }
        }
    }
}

/// Event loop, factored out for SDL and Emscripten support.
#[cfg(target_os = "emscripten")]
mod event_loop {
    extern crate emscripten_sys;

    use piston::input::{AfterRenderArgs, Event, Loop, RenderArgs, UpdateArgs};
    use piston::window::Window;
    use sdl2_window::Sdl2Window;
    use std::mem;
    use std::os::raw::c_void;

    struct EventLoop<T> {
        last_updated: f64,
        window: Sdl2Window,
        handler: fn(&mut Sdl2Window, Event, &mut T) -> bool,
        arg: T,
    }

    pub fn run<T>(
        window: Sdl2Window,
        handler: fn(&mut Sdl2Window, Event, &mut T) -> bool,
        arg: T,
    ) {
        unsafe {
            let mut events = Box::new(EventLoop {
                last_updated: emscripten_sys::emscripten_get_now() as f64,
                window: window,
                handler: handler,
                arg: arg,
            });
            let events_ptr = &mut *events as *mut EventLoop<_> as *mut c_void;
            emscripten_sys::emscripten_set_main_loop_arg(
                Some(main_loop_c::<T>),
                events_ptr,
                0,
                1,
            );
            mem::forget(events);
        }
    }

    extern "C" fn main_loop_c<T>(arg: *mut c_void) {
        unsafe {
            let events: &mut EventLoop<T> = mem::transmute(arg);
            let window = &mut events.window;
            let handler = events.handler;
            let arg = &mut events.arg;
            window.swap_buffers();

            let e = Event::Loop(Loop::AfterRender(AfterRenderArgs));
            handler(window, e, arg);

            while let Some(e) = window.poll_event() {
                handler(window, Event::Input(e), arg);
            }

            if window.should_close() {
                emscripten_sys::emscripten_cancel_main_loop();
                return;
            }

            let now = emscripten_sys::emscripten_get_now() as f64;
            let dt = (now - events.last_updated) / 1000.0;
            events.last_updated = now;

            let e = Event::Loop(Loop::Update(UpdateArgs { dt: dt }));
            handler(window, e, arg);

            let size = window.size();
            let draw_size = window.draw_size();
            let e = Event::Loop(Loop::Render(RenderArgs {
                ext_dt: dt,
                width: size.width,
                height: size.height,
                draw_width: draw_size.width,
                draw_height: draw_size.height,
            }));
            handler(window, e, arg);
        }
    }
}
