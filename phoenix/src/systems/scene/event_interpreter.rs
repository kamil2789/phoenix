use crate::{events::action::Action, systems::scene::Scene};

pub fn process_actions(mut actions: Vec<Action>, scene: &mut Scene) -> bool {
    while let Some(action) = actions.pop() {
        let time = scene.get_delta_time();
        match action {
            Action::CameraUpdateForward => scene.camera.as_mut().unwrap().move_forward(time),
            Action::CameraUpdateBackward => scene.camera.as_mut().unwrap().move_backward(time),
            Action::CameraUpdateLeft => scene.camera.as_mut().unwrap().move_left(time),
            Action::CameraUpdateRight => scene.camera.as_mut().unwrap().move_right(time),
            Action::CameraFov(yoffset) => scene.camera.as_mut().unwrap().change_fov(yoffset),
            Action::CameraOrientation(xpos, ypos) => scene.camera.as_mut().unwrap().change_orientation(xpos, ypos)
        }
    }

    true
}