use std::collections::HashMap;

use phoenix::renderer::Api;

use crate::tests::{basic_2d_geometries, TestsList};
pub struct TestCollector {
    opengl: ApiTestSuite,
    vulkan: ApiTestSuite,
}

struct ApiTestSuite {
    pub tests: TestsList,
    pub not_supported: Vec<String>,
}

impl TestCollector {
    pub fn new() -> Self {
        let opengl = create_api_test_suite(Api::OpenGL);
        let vulkan = create_api_test_suite(Api::Vulkan);
        TestCollector { opengl, vulkan }
    }

    pub fn from_test_name(test_name: &str) -> Self {
        let opengl = create_suite_for_specific_test(test_name, Api::OpenGL);
        let vulkan = create_suite_for_specific_test(test_name, Api::Vulkan);
        TestCollector { opengl, vulkan }
    }

    pub fn get_api_tests(&self, api: Api) -> &TestsList {
        match api {
            Api::OpenGL => &self.opengl.tests,
            Api::Vulkan => &self.vulkan.tests,
        }
    }

    pub fn get_api_not_supported(&self, api: Api) -> &Vec<String> {
        match api {
            Api::OpenGL => &self.opengl.not_supported,
            Api::Vulkan => &self.vulkan.not_supported,
        }
    }
}

fn create_api_test_suite(api: Api) -> ApiTestSuite {
    match api {
        Api::OpenGL => {
            let tests = create_opengl_tests();
            let not_supported = create_opengl_not_supported_tests();
            ApiTestSuite {
                tests,
                not_supported,
            }
        }
        Api::Vulkan => {
            let tests = create_vulkan_tests();
            let not_supported = create_vulkan_not_supported_tests();
            ApiTestSuite {
                tests,
                not_supported,
            }
        }
    }
}

fn create_suite_for_specific_test(test_name: &str, api: Api) -> ApiTestSuite {
    let all_tests = create_api_test_suite(api);

    let mut not_supported = vec![];
    let mut tests: TestsList = HashMap::new();

    if let Some(func_test) = all_tests.tests.get(test_name) {
        tests.insert(test_name.to_string(), *func_test);
    }

    if all_tests.not_supported.contains(&test_name.to_string()) {
        not_supported.push(test_name.to_string());
    }

    ApiTestSuite{
        tests,
        not_supported,
    }
}

fn create_opengl_tests() -> TestsList {
    let mut tests = basic_2d_geometries::TEST_LIST.clone();
    basic_2d_geometries::OPENGL_NOT_SUPPORTED
        .iter()
        .for_each(|test| {
            tests.remove(test);
        });
    tests
}

fn create_opengl_not_supported_tests() -> Vec<String> {
    const ARBITRARY_SIZE: usize = 100;
    let mut tests = Vec::with_capacity(ARBITRARY_SIZE);

    tests.extend_from_slice(&basic_2d_geometries::OPENGL_NOT_SUPPORTED);

    tests
}

fn create_vulkan_tests() -> TestsList {
    let mut tests = basic_2d_geometries::TEST_LIST.clone();
    basic_2d_geometries::VULKAN_NOT_SUPPORTED
        .iter()
        .for_each(|test| {
            tests.remove(test);
        });
    tests
}

fn create_vulkan_not_supported_tests() -> Vec<String> {
    const ARBITRARY_SIZE: usize = 100;
    let mut tests = Vec::with_capacity(ARBITRARY_SIZE);

    tests.extend_from_slice(&basic_2d_geometries::VULKAN_NOT_SUPPORTED);

    tests
}
