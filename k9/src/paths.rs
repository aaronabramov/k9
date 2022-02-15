use anyhow::Result;

use std::path::{Component, Path, PathBuf};

// Project root is the root of the entire project. The project might contain multiple crate and it should not
// be used together with whatever `file!()` macro will return.
pub fn get_project_root_path() -> PathBuf {
    // If there's a buck build id we'll grab the `pwd`, because we're probably running `buck test` from the root
    if crate::config::CONFIG.built_with_buck {
        let pwd = std::env::var("PWD").expect(
            "
`BUCK_BUILD_ID` environment variable was present, which means this project is being built with
buck and relies on `PWD` env variable to contain the project root, but `PWD` wasn't there",
        );
        return PathBuf::from(pwd);
    }

    // otherwise ask cargo for project root
    let project_root =
        std::env::var("CARGO_MANIFEST_DIR").expect("Can't get project root directory");
    PathBuf::from(project_root)
}

// Crate root will be the root of the project + directory of one of the workspace crates (if exists)
// To find this we'll need to use any `file!()` macro value to test if the file exist using
// an absolute path.
pub fn find_crate_root(result_of_file_macro: &str) -> Result<PathBuf> {
    let project_root = get_project_root_path();

    let mut without_overlap = project_root.clone();
    without_overlap.push(result_of_file_macro);

    if without_overlap.exists() {
        return Ok(project_root);
    }

    let root_with_overlap_removed = remove_overlap(&project_root, result_of_file_macro)?;

    let mut with_overlap_removed = root_with_overlap_removed.clone();
    with_overlap_removed.push(result_of_file_macro);

    if !with_overlap_removed.exists() {
        anyhow::bail!(format!(
            "
            Failed to locate the path of the source file.
            Project root was determined to be `{cargo_manifest_dir}`
            and the relative source file path given `{result_of_file_macro}

            Tried paths:
            `{without_overlap}`
            `{with_overlap_removed}`
            ",
            cargo_manifest_dir = project_root.display(),
            result_of_file_macro = result_of_file_macro,
            without_overlap = without_overlap.display(),
            with_overlap_removed = with_overlap_removed.display(),
        ))
    }

    Ok(root_with_overlap_removed)
}

// This is a hack to work around the issue with project root path resolution when
// using workspaces.
//
// When using a `file!()` macro in a crate that is a part of a workspace it will return
// a relative path from the workspace root.
// At the same time CARGO_MANIFEST env variable with hold the value of the crate's Cargo.toml
// path and not the workspace Cargo.toml.
//
// e.g.
//    /home
//        my_project/
//          Cargo.toml    <---- workspace definition
//          nested_crate/
//              Cargo.toml    <---------- other project's manifest
//              lib.rs      <------- file!() macro used here
//
//
// `file()` macro will return "my_project/nested_crate/lib.rs"
// and CARGO_MANIFEST will contain "/home/my_project"
//
// In the end we want to find the absolute path to the file, which is
//      `/home/my_project/nested_crate/lib.rs`
//
//
// There's probably a better solution for this problem, but after 20 min of research
// the sketchy workaround i found is to just join two paths while also removing the
// overlapping part.
//
// Technically this can be a bit dangerous, since the joining part may resolve in
// another existing file that is not the file we're looking for (esp if trying to
// resolve some generic file names like `lib.rs`) but the risk should be fairly minimal.
fn remove_overlap(left: &Path, right: &str) -> Result<PathBuf> {
    let right = PathBuf::from(right);

    let left_comps = left.components().collect::<Vec<_>>();

    let mut r_prefix = vec![];

    for (i, r_comp) in right.components().enumerate() {
        match r_comp {
            Component::Normal(_n) => {
                r_prefix.push(r_comp);

                let l_suffix = &left_comps[(left_comps.len() - i - 1)..];

                if l_suffix == &r_prefix[..] {
                    let result = left_comps[..(left_comps.len() - i - 1)]
                        .iter()
                        .collect::<PathBuf>();
                    return Ok(result);
                }
            }
            _ => anyhow::bail!(format!(
                "Invalid path component. Expected to only have normals: `{:?}`",
                r_comp
            )),
        }
    }

    let result = left.to_owned();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn remove_overlap_helper(l: &str, r: &str) -> Result<String> {
        Ok(remove_overlap(&PathBuf::from(l), r)?.display().to_string())
    }
    #[test]
    fn remove_path_overlap_test() -> Result<()> {
        k9_stable::assert_matches_inline_snapshot!(
            remove_overlap_helper("hello/world", "world/hello")?,
            r##"hello"##
        );

        k9_stable::assert_matches_inline_snapshot!(
            remove_overlap_helper("a/b/c/d/e/f/g", "c/d/e/f/g/h/i")?,
            r##"a/b"##
        );

        k9_stable::assert_matches_inline_snapshot!(
            remove_overlap_helper("a/b/c", "a/b/c")?,
            r##""##
        );

        // no overlap, similar directories
        k9_stable::assert_matches_inline_snapshot!(
            remove_overlap_helper("a/b/c/d", "a/b/c")?,
            r##"a/b/c/d"##
        );

        k9_stable::assert_matches_inline_snapshot!(
            remove_overlap_helper("/home/workspace/my_crate", "my_crate/my_file")?,
            r##"/home/workspace"##
        );

        k9_stable::assert_matches_inline_snapshot!(
            remove_overlap_helper("/Users/me/p/gull/gull", "gull/e2e/flow_codegen_test.rs")?,
            r##"/Users/me/p/gull"##
        );
        Ok(())
    }
}
