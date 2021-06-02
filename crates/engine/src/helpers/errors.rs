pub enum InitError {
    Window,
    Gpu
}

impl ToString for InitError {
    fn to_string(&self) -> String {
        match self {
        InitError::Window => return "Error trying to create the Window".to_string(),
        InitError::Gpu => return "Error trying to generate the GPU aftraction".to_string()
        }
    }
}
