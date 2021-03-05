pub struct WindowSettings {
    pub gl_version: (u32, u32),
    pub window_size: (u32, u32),
    pub title: String
}

impl WindowSettings {
    pub fn new() -> Self {
        WindowSettings { 
            gl_version: (4, 1),
            window_size: (640, 480),
            title: String::new()
         }
    }
}
