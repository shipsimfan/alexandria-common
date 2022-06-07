pub trait Input {
    fn new() -> Self;

    fn key_down(&mut self, key: u8);
    fn key_up(&mut self, key: u8);
    fn mouse_down(&mut self, key: u8);
    fn mouse_up(&mut self, key: u8);
    fn update_mouse_position(&mut self, position: (isize, isize));
    fn set_mouse_lock(&mut self, state: bool);
    fn frame_reset(&mut self);

    fn is_mouse_locked(&self) -> bool;
}
