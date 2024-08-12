#[derive(Clone)]
pub enum Action {
    CameraUpdateForward,
    CameraUpdateBackward,
    CameraUpdateLeft,
    CameraUpdateRight,
    CameraFov(f32),
    CameraOrientation(f32, f32)
}
