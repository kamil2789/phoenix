use crate::{events::action::Action, systems::scene::Scene};

pub fn process_actions(mut actions: Vec<Action>, scene: &mut Scene) -> bool {
    while let Some(action) = actions.pop() {
        let time = scene.get_delta_time();
        match action {
            Action::PrintFPS() => println!("FPS: {}", scene.fps_counter.get_fps()),
            Action::ChangeBackgroundColor(color) => scene.set_background_color(color),
            Action::CameraUpdateDown => scene.camera.as_mut().unwrap().move_down(time),
            Action::CameraUpdateUp => scene.camera.as_mut().unwrap().move_up(time),
            Action::CameraUpdateForward => scene.camera.as_mut().unwrap().move_forward(time),
            Action::CameraUpdateBackward => scene.camera.as_mut().unwrap().move_backward(time),
            Action::CameraUpdateLeft => scene.camera.as_mut().unwrap().move_left(time),
            Action::CameraUpdateRight => scene.camera.as_mut().unwrap().move_right(time),
            Action::CameraFov(yoffset) => scene.camera.as_mut().unwrap().change_fov(yoffset),
            Action::CameraOrientation(xpos, ypos) => scene
                .camera
                .as_mut()
                .unwrap()
                .change_orientation(xpos, ypos),
        }
    }

    true
}
