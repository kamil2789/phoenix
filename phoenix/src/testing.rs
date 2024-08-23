macro_rules! setup_window {
    () => {
        let config = GlfwConfig::create().unwrap();
        let window = Rc::new(
            config
                .create_window(
                    "OpenGL",
                    Resolution {
                        width: 1600,
                        height: 900,
                    },
                )
                .unwrap(),
        );
        window.set_current().unwrap();
    };
}

pub(crate) use setup_window;