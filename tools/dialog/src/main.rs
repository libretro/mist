const DPI_SCALE_FACTOR: f32 = 1.75;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    title: String,

    #[clap(long)]
    message: String,

    #[clap(long)]
    option: Vec<String>,

    #[clap(long, default_value_t = 1.75)]
    dpi_scale_factor: f32,
}

fn create_display(
    event_loop: &glutin::event_loop::EventLoop<()>,
    title: &str,
    fullscreen: bool,
) -> (
    glutin::WindowedContext<glutin::PossiblyCurrent>,
    glow::Context,
) {
    let mut window_builder = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize {
            width: 650.0,
            height: 230.0,
        })
        .with_title(title);

    if fullscreen {
        window_builder =
            window_builder.with_fullscreen(Some(glutin::window::Fullscreen::Borderless(None)));
    } else {
        window_builder = window_builder.with_resizable(false);
    }

    let gl_window = unsafe {
        glutin::ContextBuilder::new()
            .with_depth_buffer(0)
            .with_srgb(true)
            .with_stencil_buffer(0)
            .with_vsync(true)
            .build_windowed(window_builder, event_loop)
            .unwrap()
            .make_current()
            .unwrap()
    };

    let gl = unsafe { glow::Context::from_loader_function(|s| gl_window.get_proc_address(s)) };

    unsafe {
        use glow::HasContext as _;
        gl.enable(glow::FRAMEBUFFER_SRGB);
    }

    (gl_window, gl)
}

struct Option {
    key: String,
    color: egui::Color32,
    label: String,
}

fn main() {
    let args = Args::parse();
    let fullscreen = true;

    let mut options = Vec::new();

    for option in args.option {
        let split = option.split(':').collect::<Vec<_>>();

        if split.len() != 3 {
            eprintln!("Invalid option \"{}\"", option);
            continue;
        }

        let color_len = split[1].len();
        let color = if let Ok(color_value) = u32::from_str_radix(&split[1], 16) {
            if color_len == 6 {
                egui::Color32::from_rgb(
                    (color_value >> 16) as u8,
                    (color_value >> 8) as u8,
                    color_value as u8,
                )
            } else if color_len == 8 {
                egui::Color32::from_rgba_unmultiplied(
                    (color_value >> 24) as u8,
                    (color_value >> 16) as u8,
                    (color_value >> 8) as u8,
                    color_value as u8,
                )
            } else {
                egui::Color32::TEMPORARY_COLOR
            }
        } else {
            egui::Color32::TEMPORARY_COLOR
        };

        options.push(Option {
            key: split[0].into(),
            color,
            label: split[2].into(),
        });
    }

    let event_loop = glutin::event_loop::EventLoop::with_user_event();
    let (gl_window, gl) = create_display(&event_loop, &args.title, fullscreen);

    let mut egui_glow = egui_glow::EguiGlow::new(&gl_window, &gl);

    {
        let egui_ctx = &egui_glow.egui_ctx;
        // Change style
        let mut style: egui::Style = (*egui_ctx.style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.override_text_color = Some(egui::Color32::from_rgb(220, 220, 220)); // Increase text brightness
        style.spacing.item_spacing = egui::vec2(10.0, 20.0);
        style.spacing.button_padding = egui::vec2(8.0, 4.0);

        if !fullscreen {
            style.visuals.window_corner_radius = 0.0;
        } else {
            style.visuals.window_corner_radius = 2.0;
            style.visuals.window_shadow.extrusion = 0.0;
        }

        egui_ctx.set_style(style);
    }

    let mut first = true;
    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {
            let mut quit = false;

            let (needs_repaint, shapes) = egui_glow.run(gl_window.window(), |egui_ctx| {
                let mut window = egui::Window::new("")
                    .collapsible(false)
                    .title_bar(false)
                    .resizable(false);

                let screen_rect = egui_ctx.input().screen_rect;

                if fullscreen {
                    window = window.anchor(egui::Align2::CENTER_CENTER, (0.0, 0.0));
                } else {
                    window = window
                        .anchor(egui::Align2::LEFT_TOP, (-1.0, -1.0))
                        .fixed_rect(screen_rect);
                }

                window.show(egui_ctx, |ui| {
                    ui.heading(&args.title);
                    ui.label(&args.message);

                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        for option in &options {
                            if ui
                                .add(egui::Button::new(&option.label).fill(option.color))
                                .clicked()
                            {
                                print!("{}", option.key);
                                quit = true;
                            }
                        }
                        if ui
                            .add(egui::Button::new("Exit").fill(egui::Color32::from_rgb(122, 0, 0)))
                            .clicked()
                        {
                            quit = true;
                        }
                    });

                    if !fullscreen {
                        ui.set_width(screen_rect.width());
                        ui.set_height(screen_rect.height());
                    }
                });
            });

            *control_flow = if quit {
                glutin::event_loop::ControlFlow::Exit
            } else if needs_repaint {
                gl_window.window().request_redraw();
                glutin::event_loop::ControlFlow::Poll
            } else {
                glutin::event_loop::ControlFlow::Wait
            };

            {
                unsafe {
                    use glow::HasContext as _;
                    gl.clear_color(0.0, 0.0, 0.0, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT);
                }

                egui_glow.paint(&gl_window, &gl, shapes);

                if first {
                    egui_glow.egui_ctx.set_pixels_per_point(
                        egui_glow.egui_ctx.pixels_per_point() * DPI_SCALE_FACTOR,
                    );
                    first = false;
                }

                gl_window.swap_buffers().unwrap();
            }
        };

        match event {
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

            glutin::event::Event::WindowEvent { event, .. } => {
                use glutin::event::WindowEvent;
                if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }

                if let glutin::event::WindowEvent::Resized(physical_size) = event {
                    gl_window.resize(physical_size);
                }

                egui_glow.on_event(&event);

                gl_window.window().request_redraw();
            }
            glutin::event::Event::LoopDestroyed => {
                egui_glow.destroy(&gl);
            }

            _ => (),
        }
    });
}
