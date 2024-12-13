use std::fs;
/**
 * Copyright 2020 Garrit Franke
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::io::Error;
use std::process::Command;

fn test_directory(dir_in: &str) -> Result<(), Error> {
    let dir_out = format!("{}_out", dir_in);
    let dir = std::env::current_dir().unwrap();

    let examples = std::fs::read_dir(dir.join(dir_in))?;

    let _ = fs::create_dir(&dir_out);

    let out_file_suffix = ".js";

    for ex in examples {
        let example = ex?;
        let in_file = dir.join(dir_in).join(example.file_name());

        // We don't want to build submodules, since they don't run without a main function
        if in_file.is_dir() {
            continue;
        }
        let out_file = dir.join(&dir_out).join(
            example
                .file_name()
                .into_string()
                .unwrap()
                .replace(".sb", out_file_suffix),
        );
        let success = Command::new("cargo")
            .arg("run")
            .arg("build")
            .arg(&in_file)
            .arg("-o")
            .arg(&out_file)
            .spawn()?
            .wait()?
            .success();
        assert!(success, "{:?}", &in_file);

        let node_installed = Command::new("node").arg("-v").spawn()?.wait()?.success();
        if node_installed {
            let execution = Command::new("node")
                .arg(out_file)
                .spawn()?
                .wait()?
                .success();
            assert!(execution, "{:?}", &in_file)
        }
    }
    Ok(())
}

#[test]
fn test_examples() -> Result<(), Error> {
    test_directory("examples")?;
    Ok(())
}

#[test]
fn test_testcases() -> Result<(), Error> {
    let dir = std::env::current_dir().unwrap();

    let in_file = dir.join("tests/main.sb");
    let success = Command::new("cargo")
        .arg("run")
        .arg("run")
        .arg(&in_file)
        .spawn()?
        .wait()?
        .success();
    assert!(success, "{:?}", &in_file);
    Ok(())
}


#[test]
fn test_struct_decl_error() -> Result<(), Error> {
    let dir = std::env::current_dir().unwrap();

    let in_file = dir.join("tests/struct_decl_err.sb");
    let success = Command::new("cargo")
        .arg("run")
        .arg("run")
        .arg(&in_file)
        .spawn()?
        .wait()?
        .success();

    assert!(!success);
    Ok(())
}

#[test]
fn test_struct_instance_error() -> Result<(), Error> {
    let dir = std::env::current_dir().unwrap();

    let in_file = dir.join("tests/struct_instance_err.sb");
    let success = Command::new("cargo")
        .arg("run")
        .arg("run")
        .arg(&in_file)
        .spawn()?
        .wait()?
        .success();

    assert!(!success);
    Ok(())
}


#[test]
fn test_inline_function_error() -> Result<(), Error> {
    let dir = std::env::current_dir().unwrap();

    let in_file = dir.join("tests/inline_function_err.sb");
    let success = Command::new("cargo")
        .arg("run")
        .arg("run")
        .arg(&in_file)
        .spawn()?
        .wait()?
        .success();

    assert!(!success);
    Ok(())
}

