use glfw_sys::glfw_bindings::glfwGetTime;

pub trait Timer {
    fn get_time(&self) -> f32;
}

#[derive(Default)]
pub struct GlfwTimer {}

pub struct FpsCounter {
    timer: Box<dyn Timer>,
    delta_time: f32,
    last_time: f32,
}

impl Timer for GlfwTimer {
    #[allow(clippy::cast_possible_truncation)]
    fn get_time(&self) -> f32 {
        unsafe { glfwGetTime() as f32 }
    }
}

impl FpsCounter {
    #[must_use]
    pub fn new(timer: Box<dyn Timer>) -> Self {
        Self {
            timer,
            delta_time: 0.0,
            last_time: 0.0,
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn update(&mut self) {
        let current_time = self.timer.get_time();
        self.delta_time = current_time - self.last_time;
        self.last_time = current_time;
    }

    #[must_use]
    pub fn get_fps(&self) -> f32 {
        1.0 / self.delta_time.abs()
    }

    #[must_use]
    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::Timer;
    use crate::systems::performance::FpsCounter;

    #[derive(Default)]
    struct TestTimer {
        tick: Cell<u16>,
    }

    //The clock for each update moves one second i.e. one frame lasts one second
    impl Timer for TestTimer {
        fn get_time(&self) -> f32 {
            let num = self.tick.get();
            self.tick.replace(num + 1);

            f32::from(self.tick.get()) * 1.0
        }
    }

    #[test]
    fn test_fps_counter_get_delta_time() {
        let timer = Box::new(TestTimer::default());

        let mut fps = FpsCounter::new(timer);
        assert_eq!(fps.get_delta_time(), 0.0);

        fps.update();
        assert!(fps.get_delta_time() > 0.99);

        fps.update();
        assert!(fps.get_delta_time() > 0.99);
    }

    #[test]
    fn test_fps_counter_get_fps() {
        let timer = Box::new(TestTimer::default());

        let mut fps = FpsCounter::new(timer);
        fps.update();
        assert_eq!(fps.get_fps(), 1.0);

        fps.update();
        assert_eq!(fps.get_fps(), 1.0);
    }
}
