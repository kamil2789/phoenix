use glfw_sys::glfw_bindings::glfwGetTime;

#[derive(Default)]
pub struct FpsCounter {
    delta_time: f32,
    last_time: f32,
}

impl FpsCounter {
    #[allow(clippy::cast_possible_truncation)]
    pub fn update(&mut self) {
        let current_time = unsafe { glfwGetTime() as f32 };
        self.delta_time = current_time - self.last_time;
        self.last_time = current_time;
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn get_fps(&self) -> u16 {
        if self.delta_time > 0.0 {
            (1.0 / self.delta_time) as u16
        } else {
            0
        }
    }

    #[must_use]
    pub fn get_precise_fps(&self) -> f32 {
        if self.delta_time > 0.0 {
            1.0 / self.delta_time
        } else {
            0.0
        }
    }

    #[must_use]
    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }
}
