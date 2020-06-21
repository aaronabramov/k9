use std::path::PathBuf;

pub fn get_project_root_path() -> PathBuf {
    let buck_build_id_present = std::env::var("BUCK_BUILD_ID").is_ok();

    // If there's a buck build id we'll grab the `pwd`, because we're probably running `buck test` from the root
    if buck_build_id_present {
        let pwd = std::env::var("PWD").expect(
            "
`BUCK_BUILD_ID` enviroment variable was present, which means this project is being built with
buck and relies on `PWD` env variable to contain the project root, but `PWD` wasn't there",
        );
        return PathBuf::from(pwd);
    }

    // otherwise ask cargo for project root
    let project_root =
        std::env::var("CARGO_MANIFEST_DIR").expect("Can't get project root directory");
    PathBuf::from(project_root)
}

pub fn get_absolute_path(relative_file_path: &str) -> PathBuf {
    let mut project_root = get_project_root_path();
    project_root.push(relative_file_path);
    project_root
}
