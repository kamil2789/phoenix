use std::collections::HashMap;

use phoenix::renderer::Api;

use crate::tests::{
    basic_2d_geometries, basic_2d_textures, basic_2d_transformations, basic_3d_geometries,
    basic_3d_lights, TestsList,
};
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
        Self { opengl, vulkan }
    }

    pub fn from_test_name(test_name: &str) -> Self {
        let opengl = create_suite_for_specific_test(test_name, Api::OpenGL);
        let vulkan = create_suite_for_specific_test(test_name, Api::Vulkan);
        Self { opengl, vulkan }
    }

    pub fn get_api_tests(&self, api: Api) -> &TestsList {
        match api {
            Api::OpenGL => &self.opengl.tests,
            Api::Vulkan => &self.vulkan.tests,
        }
    }

    pub const fn get_api_not_supported(&self, api: Api) -> &Vec<String> {
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
        tests.insert(test_name.to_owned(), *func_test);
    }

    if all_tests.not_supported.contains(&test_name.to_owned()) {
        not_supported.push(test_name.to_owned());
    }

    ApiTestSuite {
        tests,
        not_supported,
    }
}

fn create_opengl_tests() -> TestsList {
    let mut tests = basic_2d_geometries::TEST_LIST.clone();
    tests.extend(basic_2d_textures::TEST_LIST.clone());
    tests.extend(basic_2d_transformations::TEST_LIST.clone());
    tests.extend(basic_3d_geometries::TEST_LIST.clone());
    tests.extend(basic_3d_lights::TEST_LIST.clone());

    for test in &create_opengl_not_supported_tests() {
        tests.remove(test);
    }
    tests
}

fn create_opengl_not_supported_tests() -> Vec<String> {
    const ARBITRARY_SIZE: usize = 100;
    let mut result = Vec::with_capacity(ARBITRARY_SIZE);

    result.extend_from_slice(&basic_2d_geometries::OPENGL_NOT_SUPPORTED);
    result.extend_from_slice(&basic_2d_textures::OPENGL_NOT_SUPPORTED);
    result.extend_from_slice(&basic_2d_transformations::OPENGL_NOT_SUPPORTED);
    result.extend_from_slice(&basic_3d_geometries::OPENGL_NOT_SUPPORTED);
    result.extend_from_slice(&basic_3d_lights::OPENGL_NOT_SUPPORTED);
    result
}

fn create_vulkan_tests() -> TestsList {
    let mut tests = basic_2d_geometries::TEST_LIST.clone();
    tests.extend(basic_2d_textures::TEST_LIST.clone());
    tests.extend(basic_2d_transformations::TEST_LIST.clone());
    tests.extend(basic_3d_geometries::TEST_LIST.clone());
    tests.extend(basic_3d_lights::TEST_LIST.clone());

    for test in &create_vulkan_not_supported_tests() {
        tests.remove(test);
    }

    tests
}

fn create_vulkan_not_supported_tests() -> Vec<String> {
    const ARBITRARY_SIZE: usize = 100;
    let mut result = Vec::with_capacity(ARBITRARY_SIZE);

    result.extend_from_slice(&basic_2d_geometries::VULKAN_NOT_SUPPORTED);
    result.extend_from_slice(&basic_2d_textures::VULKAN_NOT_SUPPORTED);
    result.extend_from_slice(&basic_2d_transformations::VULKAN_NOT_SUPPORTED);
    result.extend_from_slice(&basic_3d_geometries::VULKAN_NOT_SUPPORTED);
    result.extend_from_slice(&basic_3d_lights::VULKAN_NOT_SUPPORTED);

    result
}
