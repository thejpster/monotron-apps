use sdl2;
use std::collections::VecDeque;
use vga_framebuffer;

pub struct Context<'a> {
    pub fb: vga_framebuffer::FrameBuffer<'a, FakeHardware>,
    pub keypresses: VecDeque<u8>,
}

pub struct FakeHardware {
    pub event_pump: sdl2::EventPump,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    point: sdl2::rect::Point,
}

// We need to do this to put a Context in a static. But is this a good idea?
// Certainly I don't recommend creating threads in Monotron programs (not that
// Monotron can do that anyway?!)
unsafe impl<'a> Send for Context<'a> {}

impl vga_framebuffer::Hardware for FakeHardware {
    fn configure(&mut self, width: u32, sync_end: u32, line_start: u32, clock_rate: u32) {
        println!(
            "width={}, sync_end={}, line_start={}, clock_rate={}",
            width, sync_end, line_start, clock_rate
        );
    }

    fn vsync_on(&mut self) {}

    fn vsync_off(&mut self) {}

    fn write_pixels(&mut self, red: u32, green: u32, blue: u32) {
        for bit in (0..8).rev() {
            let red_bit = red & (1 << bit) != 0;
            let green_bit = green & (1 << bit) != 0;
            let blue_bit = blue & (1 << bit) != 0;
            self.canvas.set_draw_color(sdl2::pixels::Color::RGB(
                if red_bit { 0xFF } else { 0x00 },
                if green_bit { 0xFF } else { 0x00 },
                if blue_bit { 0xFF } else { 0x00 },
            ));
            self.canvas
                .draw_point(self.point.offset(self.point.x, 0))
                .unwrap();
            self.canvas
                .draw_point(self.point.offset(self.point.x + 1, 0))
                .unwrap();
            let mut new_point = self.point.offset(1, 0);
            if new_point.x == 400 {
                new_point = self.point.offset(-self.point.x, 1);
                if new_point.y == 600 {
                    new_point = (0, 0).into();
                }
            }
            self.point = new_point;
        }
    }
}

impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Monotron", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_logical_size(800, 600).unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let d = FakeHardware {
            canvas,
            event_pump,
            point: (0, 0).into(),
        };
        let mut fb = vga_framebuffer::FrameBuffer::new();
        fb.init(d);
        Context {
            fb,
            keypresses: VecDeque::new(),
        }
    }

    pub fn toggle_fullscreen(&mut self) {
        use sdl2::video::FullscreenType;
        let ref mut canvas = self.fb.borrow_hw_mut().unwrap().canvas;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        if canvas.window_mut().fullscreen_state() == FullscreenType::Off {
            canvas
                .window_mut()
                .set_fullscreen(FullscreenType::Desktop)
                .unwrap();
        } else {
            canvas
                .window_mut()
                .set_fullscreen(FullscreenType::Off)
                .unwrap();
        };
    }

    pub fn draw(&mut self) {
        for _r in 0..628 {
            self.fb.isr_sol();
        }
        self.fb.borrow_hw_mut().unwrap().canvas.present();
    }

    pub fn pump(&mut self) {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        let mut need_toggle = false;
        for event in self.fb.borrow_hw_mut().unwrap().event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => std::process::exit(1),
                Event::KeyDown {
                    keycode: Some(Keycode::F11),
                    ..
                } => {
                    need_toggle = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    self.keypresses.push_back(b'\r');
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    self.keypresses.push_back(8);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.keypresses.push_back(27);
                }
                Event::TextInput { text, .. } => {
                    println!("Got {:?}", text);
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
