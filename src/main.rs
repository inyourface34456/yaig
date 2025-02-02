mod support;
mod ui;
mod game;
mod settings;
mod componets;

use glium::glutin::surface::WindowSurface;
use glium::{Display, Surface};
use imgui::Context;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::winit::dpi::LogicalSize;
use imgui_winit_support::winit::event::{Event, WindowEvent};
use imgui_winit_support::winit::event_loop::EventLoop;
use imgui_winit_support::winit::window::WindowAttributes;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::path::Path;
use std::time::Instant;
use support::{DPI, FONT_SIZE, clipboard, create_context};
use ui::main_ui;
use game::{Game, Tab};
use settings::{Settings, Screen};
use componets::*;

const TITLE: &'static str = file!();

fn init(_ctx: &mut Context, _render: &mut Renderer, _display: &Display<WindowSurface>) {}

fn main() {
    let mut game_state = Game::default();
    let mut imgui = create_context(FONT_SIZE);

    let title = match Path::new(&TITLE).file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => TITLE,
    };
    let event_loop = EventLoop::new().expect("Failed to create EventLoop");

    let window_attributes = WindowAttributes::default()
        .with_title(title)
        .with_inner_size(LogicalSize::new(1024, 768));
    //.with_resizable(false);

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_attributes)
        .build(&event_loop);

    let mut renderer = Renderer::new(&mut imgui, &display).expect("Failed to initialize renderer");

    if let Some(backend) = clipboard::init() {
        imgui.set_clipboard_backend(backend);
    } else {
        eprintln!("Failed to initialize clipboard");
    }

    let mut platform = WinitPlatform::new(&mut imgui);
    {
        let dpi_mode = if let Ok(factor) = Ok::<String, ()>(format!("{}", DPI)) {
            // Allow forcing of HiDPI factor for debugging purposes
            match factor.parse::<f64>() {
                Ok(f) => HiDpiMode::Locked(f),
                Err(e) => panic!("Invalid scaling factor: {}", e),
            }
        } else {
            HiDpiMode::Default
        };

        platform.attach_window(imgui.io_mut(), &window, dpi_mode);
    }

    let mut last_frame = Instant::now();

    init(&mut imgui, &mut renderer, &display);

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::AboutToWait => {
                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");

                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let ui = imgui.frame();

                let (width, height) = display.get_framebuffer_dimensions();
                let run = true;

                game_state.width = width as f32;
                game_state.height = height as f32;

                match game_state.config.fullscreen {
                    Screen::Windowed => window.set_fullscreen(None),
                    Screen::Fullscreen => window.set_fullscreen(Some(glium::winit::window::Fullscreen::Exclusive(window_target.primary_monitor().unwrap().video_modes().next().unwrap()))),
                    Screen::Borderless => window.set_fullscreen(Some(glium::winit::window::Fullscreen::Borderless(None))),
                }

                main_ui(ui, &mut game_state);

                if !run {
                    window_target.exit();
                }

                let mut target = display.draw();
                target.clear_color_srgb(0., 0., 0., 0.);
                platform.prepare_render(ui, &window);
                let draw_data = imgui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                if new_size.width > 0 && new_size.height > 0 {
                    display.resize((new_size.width, new_size.height));
                }
                platform.handle_event(imgui.io_mut(), &window, &event);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => window_target.exit(),
            event => {
                platform.handle_event(imgui.io_mut(), &window, &event);
            }
        })
        .expect("EventLoop error");
}
