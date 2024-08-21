use crate::components::color::RGBA;

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    CameraUpdateForward,
    CameraUpdateBackward,
    CameraUpdateLeft,
    CameraUpdateRight,
    CameraFov(f32),
    CameraOrientation(f32, f32),
    ChangeBackgroundColor(RGBA),
    PrintFPS(),
}
