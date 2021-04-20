use std::sync::{Arc, Mutex};

pub trait Notify<T> {
    fn notify(&self, arg: T);
}

pub trait Observer<T> {
    fn on_notify(&mut self, event: &T);
}


#[derive(Debug, Clone)]
pub enum Error {
    ErrorA, ErrorB
}

pub type GameResult<T = ()> = Result<T, Error>;

#[derive(Default)]
pub struct EventSystem {
    wrapped_observers: Vec<Arc<Mutex<dyn Observer>>>,
}



impl Notify<MouseEventArgs> for EventSystem {
    fn notify(&self, arg: MouseEventArgs) {

    }
}

impl EventSystem {
    pub fn new() -> EventSystem {
        EventSystem {
            wrapped_observers: vec![],
        }
    }

    pub fn notify(&self, event: Event) {
        for wrapped_observer in self.wrapped_observers.clone() {
            let mut observer = wrapped_observer.lock().unwrap();
            observer.on_notify(&event);
        }
    }

    pub fn add_observer(&mut self, observer: Arc<Mutex<dyn Observer>>) {
        self.wrapped_observers.push(observer);
    }
}





pub enum Event {
    MouseEvent,
    KeyEvent,
    ResizeEvent,
    DropEvent
}

#[derive(Default)]
pub struct MouseEventArgs {
    mouse_button: u8,
    mouse_position: glam::Vec2
}

impl MouseEventArgs {
    pub fn new() -> GameResult<Arc<Mutex<MouseEventArgs>>> {
        Ok(Arc::new(Mutex::new(MouseEventArgs{ mouse_button:0, mouse_position: glam::Vec2::new(0.0, 0.0) })))
    }
}

impl Observer for MouseEventArgs {
    fn on_notify(&mut self, event: &Event) {
        match event {
            Event::MouseEvent => {
                println!("MouseEvent");
                // self.player += 1;
            },
            Event::KeyEvent => {
                println!("KeyEvent");
                // self.ai += 1;
            }
            Event::ResizeEvent => {
                println!("ResizeEvent");
                // self.ai += 1;
            },
            Event::DropEvent => {
                println!("DropEvent");
                // self.ai += 1;
            }
        }
    }
}