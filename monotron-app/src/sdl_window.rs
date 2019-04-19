use sdl2;
use std::collections::VecDeque;
use vga_framebuffer;

pub struct Context<'a> {
    pub fb: vga_framebuffer::FrameBuffer<'a, Dummy>,
    pub event_pump: sdl2::EventPump,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub keypresses: VecDeque<u8>,
}

pub struct Dummy();

impl vga_framebuffer::Hardware for Dummy {
    fn configure(&mut self, width: u32, sync_end: u32, line_start: u32, clock_rate: u32) {
        println!(
            "width={}, sync_end={}, line_start={}, clock_rate={}",
            width, sync_end, line_start, clock_rate
        );
    }

    fn vsync_on(&self) {}

    fn vsync_off(&self) {}
}

unsafe impl<'a> Send for Context<'a> {}

impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let d = Dummy();
        let mut fb = vga_framebuffer::FrameBuffer::new();
        fb.init(d);
        let window = video_subsystem
            .window("Monotron", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_logical_size(800, 600).unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        Context {
            event_pump,
            canvas,
            fb,
            keypresses: VecDeque::new(),
        }
    }

    pub fn toggle_fullscreen(&mut self) {
        use sdl2::video::FullscreenType;
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
        if self.canvas.window_mut().fullscreen_state() == FullscreenType::Off {
            self.canvas
                .window_mut()
                .set_fullscreen(FullscreenType::Desktop)
                .unwrap();
        } else {
            self.canvas
                .window_mut()
                .set_fullscreen(FullscreenType::Off)
                .unwrap();
        };
    }

    pub fn draw(&mut self) {
        for _r in 0..29 {
            self.fb.isr_sol();
        }
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        let mut old_triple = (false, false, false);
        for y in 0..600 {
            self.fb.isr_sol();
            for (x_byte, (red, green, blue)) in self.fb.iter_u8().enumerate() {
                for bit in (0..8).rev() {
                    let red_bit = red & (1 << bit) != 0;
                    let green_bit = green & (1 << bit) != 0;
                    let blue_bit = blue & (1 << bit) != 0;
                    let x = (x_byte as i32 * 8) + (7 - bit);
                    let new_triple = (red_bit, green_bit, blue_bit);
                    if new_triple != old_triple {
                        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(
                            if red_bit { 0xFF } else { 0x00 },
                            if green_bit { 0xFF } else { 0x00 },
                            if blue_bit { 0xFF } else { 0x00 },
                        ));
                        old_triple = new_triple;
                    }
                    self.canvas.draw_point((2 * x, y)).unwrap();
                    self.canvas.draw_point(((2 * x) + 1, y)).unwrap();
                }
            }
        }
        self.canvas.present();
    }

    pub fn pump(&mut self) {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        let mut need_toggle = false;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => std::process::exit(1),
                Event::KeyDown {
                    keycode: Some(Keycode::F11),
                    ..
                } => {
                    need_toggle = true;
                }
                Event::TextInput { text, .. } => {
                    for b in text.bytes() {
                        self.keypresses.push_back(b);
                    }
                }
                _ => {}
            }
        }
        if need_toggle {
            self.toggle_fullscreen();
        }
    }
}
