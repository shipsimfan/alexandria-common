use crate::Vector2;

pub trait ViewportUpdater {
    // Returns (top_left, size)
    fn update_viewport(&mut self, new_window_size: Vector2) -> (Vector2, Vector2);
}

pub trait Viewport {
    fn set_active(&mut self);
    fn update(&mut self, top_left: Vector2, size: Vector2);
}

pub struct FitScreenUpdater;

pub struct FixedAspectUpdater {
    aspect: f32,
}

impl FitScreenUpdater {
    pub fn new() -> Box<dyn ViewportUpdater> {
        Box::new(FitScreenUpdater)
    }
}

impl ViewportUpdater for FitScreenUpdater {
    fn update_viewport(&mut self, new_window_size: Vector2) -> (Vector2, Vector2) {
        (Vector2::ZERO, new_window_size)
    }
}

impl FixedAspectUpdater {
    pub fn new(aspect: f32) -> Box<dyn ViewportUpdater> {
        Box::new(FixedAspectUpdater { aspect })
    }
}

impl ViewportUpdater for FixedAspectUpdater {
    fn update_viewport(&mut self, new_window_size: Vector2) -> (Vector2, Vector2) {
        let window_aspect = new_window_size.x() / new_window_size.y();

        if window_aspect > self.aspect {
            let size = Vector2::new(self.aspect * new_window_size.y(), new_window_size.y());
            let top_left = Vector2::new((new_window_size.x() - size.x()) / 2.0, 0.0);
            (top_left, size)
        } else {
            let size = Vector2::new(new_window_size.x(), new_window_size.x() / self.aspect);
            let top_left = Vector2::new(0.0, (new_window_size.y() - size.y()) / 2.0);
            (top_left, size)
        }
    }
}
