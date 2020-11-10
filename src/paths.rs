use anyhow::Result;

use std::path::{Component, Path, PathBuf};

pub fn get_project_root_path() -> PathBuf {
    // If there's a buck build id we'll grab the `pwd`, because we're probably running `buck test` from the root
    if crate::config::CONFIG.built_with_buck {
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

pub fn get_absolute_path(relative_file_path: &str) -> Result<PathBuf> {
    let project_root = get_project_root_path();

    let mut left = project_root.clone();
    left.push(relative_file_path);
    let result = left;

    if !result.exists() {
        let result_with_overlap_removed =
            join_and_remove_overlap(&project_root, relative_file_path)?;
        dbg!(&result_with_overlap_removed);

        if !result_with_overlap_removed.exists() {
            anyhow::bail!(format!(
                "
                Failed to locate the path of the source file.
                Project root was determined to be `{project_root}`
                and the relative source file path given `{relative_file_path}

                Tried paths:
                `{result}`
                `{result_with_overlap_removed}`
                ",
                project_root = project_root.display(),
                relative_file_path = relative_file_path,
                result = result.display(),
                result_with_overlap_removed = result_with_overlap_removed.display(),
            ))
        }

        return Ok(result_with_overlap_removed);
    }

    Ok(result)
}

// This is a hack to work around the issue with project root path resolution when
// using workspaces.
//
// When using a `file!()` macro in a crate that is a part of a workspace it will return
// a relative path from the worspace root.
// At the same time CARGO_MANIFEST env variable with hold the value of the crate's Cargo.toml
// path and not the workspace Cargo.toml.
//
// e.g.
//    /home
//        my_project/
//          Cargo.toml    <---- worspace definition
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
fn join_and_remove_overlap(left: &Path, right: &str) -> Result<PathBuf> {
    let right = PathBuf::from(right);

    let left_comps = left.components().collect::<Vec<_>>();

    let mut r_prefix = vec![];

    for (i, r_comp) in right.components().enumerate() {
        match r_comp {
            Component::Normal(_n) => {
                r_prefix.push(r_comp);

                let l_suffix = &left_comps[(left_comps.len() - i - 1)..];

                if l_suffix == &r_prefix[..] {
                    let mut result = left_comps[..(left_comps.len() - i - 1)]
                        .iter()
                        .collect::<PathBuf>();
                    result.push(right);
                    return Ok(result);
                }
            }
            _ => anyhow::bail!(format!(
                "Invalid path component. Expected to only have normals: `{:?}`",
                r_comp
            )),
        }
    }

    let mut result = left.to_owned();
    result.push(right);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::super::assert_matches_inline_snapshot;
    use super::*;

    fn join_overlap_helper(l: &str, r: &str) -> Result<String> {
        Ok(join_and_remove_overlap(&PathBuf::from(l), r)?
            .display()
            .to_string())
    }
    #[test]
    fn remove_path_overlap_test() -> Result<()> {
        assert_matches_inline_snapshot!(
            join_overlap_helper("hello/world", "world/hello")?,
            "hello/world/hello"
        );

        assert_matches_inline_snapshot!(
            join_overlap_helper("a/b/c/d/e/f/g", "c/d/e/f/g/h/i")?,
            "a/b/c/d/e/f/g/h/i"
        );

        assert_matches_inline_snapshot!(join_overlap_helper("a/b/c", "a/b/c")?, "a/b/c");

        // no overlap, similar directories
        assert_matches_inline_snapshot!(join_overlap_helper("a/b/c/d", "a/b/c")?, "a/b/c/d/a/b/c");

        assert_matches_inline_snapshot!(
            join_overlap_helper("/home/workspace/my_crate", "my_crate/my_file")?,
            "/home/workspace/my_crate/my_file"
        );
        Ok(())
    }
}
