use crate::args_parser::{Args, GraphicApi};
use crate::utils::TestCollector;
use crate::workspace::prepare_working_directory;
use crate::{
    image::{are_images_equal, read_image_from_file, save_screen_as_img_png},
    workspace::{TEST_FILE_EXTENSION, TEST_RESULTS_DIR, TEST_TEMPLATE_DIR},
};
use colored::Colorize;
use phoenix::renderer::vulkan::Vulkan;
use phoenix::renderer::Api;
use phoenix::{
    renderer::{opengl::OpenGL, Render},
    window::{GlfwConfig, Resolution, Window},
};
use std::collections::HashMap;
use std::rc::Rc;

pub mod basic_2d_geometries;
pub mod basic_2d_textures;
pub mod basic_2d_transformations;
pub mod basic_3d_geometries;
pub mod basic_3d_lights;

pub type TestFunction = fn(Rc<Window>, Box<dyn Render>);
#[allow(clippy::module_name_repetitions)]
pub type TestsList = HashMap<String, TestFunction>;

pub enum TestResult {
    Passed,
    Failed,
}

impl From<bool> for TestResult {
    fn from(value: bool) -> Self {
        if value {
            Self::Passed
        } else {
            Self::Failed
        }
    }
}

pub fn run(args: &Args) {
    prepare_working_directory();
    let config = create_config();
    let window = Rc::new(create_window(&config));
    dispatch_tests(&window, args);
}

fn dispatch_tests(window: &Rc<Window>, args: &Args) {
    let tests = if args.test_name == "All" {
        TestCollector::new()
    } else {
        TestCollector::from_test_name(&args.test_name)
    };

    if args.graphic_api == GraphicApi::All {
        run_tests(&tests, window, Api::OpenGL);
        run_tests(&tests, window, Api::Vulkan);
    } else if args.graphic_api == GraphicApi::Opengl {
        run_tests(&tests, window, Api::OpenGL);
    } else if args.graphic_api == GraphicApi::Vulkan {
        run_tests(&tests, window, Api::Vulkan);
    }
}

//Add support for just one test

fn run_tests(tests: &TestCollector, window: &Rc<Window>, api: Api) {
    let mut failed_tests: Vec<String> = Vec::new();
    let mut passed_tests: Vec<String> = Vec::new();

    let message = format!("Running tests for API: {api:?}").blue();
    println!("{message}");
    for (test_name, test_func) in tests.get_api_tests(api) {
        let renderer = create_renderer(window, api);
        let result = run_specific_test(window.clone(), renderer, *test_func, test_name);
        match result {
            TestResult::Failed => failed_tests.push(test_name.clone()),
            TestResult::Passed => passed_tests.push(test_name.clone()),
        };
    }

    print_tests_status(
        &failed_tests,
        &passed_tests,
        tests.get_api_not_supported(api),
    );
}

fn create_renderer(window: &Window, api: Api) -> Box<dyn Render> {
    match api {
        Api::OpenGL => Box::new(OpenGL::new(window).unwrap()),
        Api::Vulkan => Box::new(Vulkan::new()),
    }
}

pub fn run_specific_test(
    window: Rc<Window>,
    renderer: Box<dyn Render>,
    run_test: fn(Rc<Window>, Box<dyn Render>),
    test_name: &str,
) -> TestResult {
    let resolution = window.get_resolution();
    run_test(window, renderer);
    let result_path = TEST_RESULTS_DIR.to_owned() + test_name + TEST_FILE_EXTENSION;
    let template_path = TEST_TEMPLATE_DIR.to_owned() + test_name + TEST_FILE_EXTENSION;
    save_screen_as_img_png(&resolution, &result_path).unwrap();

    read_image_from_file(&result_path).map_or_else(
        |_| {
            println!("Failed to read test result image from path: {result_path}");
            TestResult::Failed
        },
        |result_image| {
            read_image_from_file(&template_path).map_or_else(
                |_| {
                    println!("Failed to read test template image from path: {template_path}");
                    TestResult::Failed
                },
                |template_image| are_images_equal(&result_image, &template_image).into(),
            )
        },
    )
}

fn print_tests_status(
    failed_tests: &Vec<String>,
    passed_tests: &Vec<String>,
    not_supported: &Vec<String>,
) {
    for test in failed_tests {
        println!("{} {}", test, "FAILED".red());
    }
    for test in passed_tests {
        println!("{} {}", test, "PASSED".green());
    }
    for test in not_supported {
        println!("{} {}", test, "NOT SUPPORTED".yellow());
    }
}

fn create_config() -> GlfwConfig {
    GlfwConfig::create().unwrap()
}

fn create_window(config: &GlfwConfig) -> Window {
    config
        .create_window(
            "Test",
            Resolution {
                width: 800,
                height: 600,
            },
        )
        .unwrap()
}
