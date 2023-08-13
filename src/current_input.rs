use std::collections::HashMap;
use winit::{
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{Key, KeyCode},
};

/// Stores a character or a backspace.
///
/// TODO: Either:
///  *   remove this struct and just use backspace character instead
///  *   move keypresses like Home, End, Left, Right, Up, Down, Return to this enum
///  (advantage of using this struct is it retains sub-frame keypress ordering)
#[derive(Clone)]
pub enum TextChar {
    Char(char),
    Back,
}

#[derive(Clone)]
pub struct CurrentInput {
    pub mouse_actions: Vec<MouseAction>,
    pub key_actions: Vec<KeyAction>,
    pub scancode_actions: Vec<ScanCodeAction>,
    pub key_held: HashMap<Key, bool>,
    pub scancode_held: HashMap<KeyCode, bool>,
    pub mouse_held: HashMap<MouseButton, bool>,
    pub mouse_point: Option<(f32, f32)>,
    pub mouse_point_prev: Option<(f32, f32)>,
    pub y_scroll_diff: f32,
    pub x_scroll_diff: f32,
    pub text: Vec<TextChar>,
}

impl CurrentInput {
    pub fn new() -> CurrentInput {
        CurrentInput {
            mouse_actions: vec![],
            key_actions: vec![],
            scancode_actions: vec![],
            key_held: HashMap::new(),
            scancode_held: HashMap::new(),
            mouse_held: HashMap::new(),
            mouse_point: None,
            mouse_point_prev: None,
            y_scroll_diff: 0.0,
            x_scroll_diff: 0.0,
            text: vec![],
        }
    }

    pub fn step(&mut self) {
        self.mouse_actions.clear();
        self.key_actions.clear();
        self.scancode_actions.clear();
        self.y_scroll_diff = 0.0;
        self.x_scroll_diff = 0.0;
        self.mouse_point_prev = self.mouse_point;
        self.text.clear();
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => match event.state {
                ElementState::Pressed => {
                    let key = &event.logical_key;

                    if !self.key_held.get(key).copied().unwrap_or_default() {
                        self.key_actions.push(KeyAction::Pressed(key.clone()));
                    }

                    self.key_held.insert(key.clone(), true);
                    self.key_actions.push(KeyAction::PressedOs(key.clone()));

                    if let Key::Backspace = key {
                        self.text.push(TextChar::Back);
                    }

                    let scancode = event.physical_key;

                    if !self
                        .scancode_held
                        .get(&scancode)
                        .copied()
                        .unwrap_or_default()
                    {
                        self.scancode_actions
                            .push(ScanCodeAction::Pressed(scancode));
                        self.scancode_held.insert(scancode, true);
                    }

                    self.scancode_actions
                        .push(ScanCodeAction::PressedOs(scancode));
                }
                ElementState::Released => {
                    let key = &event.logical_key;

                    self.key_held.insert(key.clone(), false);
                    self.key_actions.push(KeyAction::Released(key.clone()));

                    let scancode = event.physical_key;

                    self.scancode_held.insert(scancode, false);

                    self.scancode_actions
                        .push(ScanCodeAction::Released(scancode));
                }
            },
            // WindowEvent::ReceivedCharacter(c) => {
            //     let c = *c;
            //     if c != '\x08' && c != '\r' && c != '\n' {
            //         self.text.push(TextChar::Char(c));
            //     }
            // }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_point = Some((position.x as f32, position.y as f32));
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => {
                self.mouse_held.insert(*button, true);
                self.mouse_actions.push(MouseAction::Pressed(*button));
            }
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button,
                ..
            } => {
                self.mouse_held.insert(*button, false);
                self.mouse_actions.push(MouseAction::Released(*button));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                // I just took this from three-rs, no idea why this magic number was chosen ¯\_(ツ)_/¯
                const PIXELS_PER_LINE: f64 = 38.0;

                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.x_scroll_diff += x;
                        self.y_scroll_diff += y;
                    }
                    MouseScrollDelta::PixelDelta(delta) => {
                        self.y_scroll_diff += (delta.y / PIXELS_PER_LINE) as f32;
                        self.x_scroll_diff += (delta.x / PIXELS_PER_LINE) as f32;
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone)]
pub enum KeyAction {
    Pressed(Key),
    PressedOs(Key),
    Released(Key),
}

#[derive(Clone, PartialEq)]
pub enum ScanCodeAction {
    Pressed(KeyCode),
    PressedOs(KeyCode),
    Released(KeyCode),
}

#[derive(Clone)]
pub enum MouseAction {
    Pressed(MouseButton),
    Released(MouseButton),
}
