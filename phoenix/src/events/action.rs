use crate::components::color::RGBA;

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    ChangeBackgroundColor(RGBA),
    CameraUpdateForward,
    CameraUpdateBackward,
    CameraUpdateLeft,
    CameraUpdateRight,
    CameraFov(f32),
    CameraOrientation(f32, f32),
}
