use vello::kurbo::{Point, Rect, Size, Vec2};
use winit::{event::{ElementState, Modifiers}, keyboard::Key};

pub struct EventState {
    pub window_size: Size,

    pub mouse_position: Option<Point>,
    pub previous_mouse_position: Option<Point>,
    pub mouse_down: bool,
    pub right_mouse_down: bool,
    pub was_mouse_down: bool,
    pub was_right_mouse_down: bool,
    pub scroll_delta: Vec2,

    pub modifiers: Modifiers,
    pub key_events: Vec<KeyEvent>,
}

impl EventState {
    pub fn new() -> Self {
        Self {
            window_size: Size::new(0., 0.),

            mouse_position: None,
            previous_mouse_position: None,
            mouse_down: false,
            right_mouse_down: false,
            was_mouse_down: false,
            was_right_mouse_down: false,
            scroll_delta: Vec2::new(0., 0.),

            modifiers: Modifiers::default(),
            key_events: Vec::new(),
        }
    }

    pub fn next_frame(&mut self) {
        self.was_mouse_down = self.mouse_down;
        self.was_right_mouse_down = self.right_mouse_down;
        self.previous_mouse_position = self.mouse_position;
        self.scroll_delta = Vec2::new(0., 0.);

        self.key_events.clear();
    }

    // Window Functions
    pub fn actual_window_size(&self) -> Size {
        self.window_size
    }

    pub fn actual_window_rect(&self) -> Rect {
        Rect::from_origin_size(Point::new(0., 0.), self.window_size)
    }

    // Mouse Functions
    pub fn mouse_down(&self) -> bool {
        self.mouse_down
    }

    pub fn right_mouse_down(&self) -> bool {
        self.right_mouse_down
    }

    pub fn was_mouse_down(&self) -> bool {
        self.was_mouse_down
    }

    pub fn was_right_mouse_down(&self) -> bool {
        self.was_right_mouse_down
    }

    pub fn mouse_released(&self) -> bool {
        !self.mouse_down && self.was_mouse_down
    }

    pub fn right_mouse_released(&self) -> bool {
        !self.right_mouse_down && self.was_right_mouse_down
    }

    pub fn mouse_just_down(&self) -> bool {
        self.mouse_down && !self.was_mouse_down
    }

    pub fn right_mouse_just_down(&self) -> bool {
        self.right_mouse_down && !self.was_right_mouse_down
    }

    pub fn mouse_held(&self) -> bool {
        self.mouse_down && self.was_mouse_down
    }

    pub fn right_mouse_held(&self) -> bool {
        self.right_mouse_down && self.was_right_mouse_down
    }

    pub fn actual_mouse_position(&self) -> Option<Point> {
        self.mouse_position
    }

    pub fn actual_previous_mouse_position(&self) -> Option<Point> {
        self.previous_mouse_position
    }

    pub fn actual_mouse_delta(&self) -> Option<Vec2> {
        self.mouse_position
            .zip(self.previous_mouse_position)
            .map(|(pos, prev)| pos - prev)
    }

    pub fn scroll_delta(&self) -> Vec2 {
        self.scroll_delta
    }

    // Keyboard Functions
    pub fn modifiers(&self) -> Modifiers {
        self.modifiers
    }

    pub fn key_events(&self) -> &[KeyEvent] {
        &self.key_events
    }
}

#[derive(Clone)]
pub struct KeyEvent {
    pub key: Key,
    pub state: ElementState,
}
