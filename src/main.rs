use macroquad::{
    input::utils as input_utils,
    miniquad,
    prelude::*,
    ui::{hash, root_ui, widgets},
};

#[cfg(target_os = "android")]
mod bluetooth;

#[cfg(target_os = "android")]
mod fileopen;

fn window_conf() -> Conf {
    Conf {
        window_title: "Quad!".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

fn screen_keyboard(id: usize, characters: &mut Vec<char>) {
    if widgets::Button::new("Show keyboard")
        .size(vec2(400., 50.))
        .ui(&mut *root_ui())
    {
        let mut gl = unsafe { get_internal_gl() };
        gl.quad_context.show_keyboard(true);
    }

    if widgets::Button::new("Hide keyboard")
        .size(vec2(400., 50.))
        .ui(&mut *root_ui())
    {
        let mut gl = unsafe { get_internal_gl() };
        gl.quad_context.show_keyboard(false);
    }

    struct MiniquadInput<'a>(&'a mut Vec<char>);

    impl<'a> miniquad::EventHandler for MiniquadInput<'a> {
        fn update(&mut self, _ctx: &mut miniquad::Context) {}
        fn draw(&mut self, _ctx: &mut miniquad::Context) {}
        fn char_event(
            &mut self,
            _ctx: &mut miniquad::Context,
            character: char,
            _keymods: miniquad::KeyMods,
            _repeat: bool,
        ) {
            self.0.push(character);
        }
    }

    let mut input = MiniquadInput(characters);
    input_utils::repeat_all_miniquad_input(&mut input, id);
}

#[macroquad::main(window_conf)]
async fn main() {
    #[cfg(target_os = "android")]
    let mut bluetooth = bluetooth::Bluetooth::new();
    #[cfg(target_os = "android")]
    let data = std::sync::Arc::new(std::sync::Mutex::new(None));
    let mut characters = Vec::new();
    let id = input_utils::register_input_subscriber();

    loop {
        clear_background(WHITE);

        let tab = root_ui().tabbar(
            hash!(),
            vec2(screen_width(), 70.),
            &["keyboard", "bluetooth", "open file"],
        );
        match tab {
            0 => {
                screen_keyboard(id, &mut characters);
                for character in &characters {
                    root_ui().label(None, &format!("input: {}", character));
                }
            }
            1 => {
                #[cfg(target_os = "android")]
                bluetooth.ui();
            }
            2 => {
                #[cfg(target_os = "android")]
                {
                    if widgets::Button::new("Load file")
                        .size(vec2(400., 50.))
                        .ui(&mut *root_ui())
                    {
                        fileopen::find_file(data.clone());
                    }
                    if let Some(data) = &*data.lock().unwrap() {
                        root_ui().label(
                            None,
                            &format!("File open, content byte length: {}", data.len()),
                        );
                    }
                }
            }
            _ => unreachable!(),
        }
        next_frame().await;
    }
}
