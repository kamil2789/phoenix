macro_rules! setup_opengl {
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
        window.set_current();
        let _renderer = OpenGL::new(&window).unwrap();
    };
}

pub(crate) use setup_opengl;
