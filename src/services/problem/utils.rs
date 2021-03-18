use crate::errors::{ServiceError, ServiceResult};
use crate::models::problems;
use digest::Digest;
use hex::ToHex;
use md5::Md5;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

fn read_settings(path: &str) -> std::io::Result<problems::ProblemSettings> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let settings: problems::ProblemSettings = toml::from_str(&contents)?;

    Ok(settings)
}

fn read_info(path: &str) -> std::io::Result<problems::ProblemInfo> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let info: problems::ProblemInfo = toml::from_str(&contents)?;

    Ok(info)
}

fn read_description(path: &str) -> std::io::Result<Option<String>> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            return Ok(None);
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(Some(contents))
}

fn read_examples(path: &str) -> std::io::Result<(Vec<problems::Example>, i32)> {
    let mut examples = Vec::new();

    let mut count = 0;
    loop {
        let id = count + 1;
        let input_name = id.to_string() + ".in";
        let mut input_file = match File::open(path.to_string() + "/" + &input_name) {
            Ok(file) => file,
            Err(_) => {
                break;
            }
        };
        let mut input_content = String::new();
        input_file.read_to_string(&mut input_content)?;

        let output_name = id.to_string() + ".out";
        let mut output_file = match File::open(path.to_string() + "/" + &output_name) {
            Ok(file) => file,
            Err(_) => {
                break;
            }
        };
        let mut output_content = String::new();
        output_file.read_to_string(&mut output_content)?;

        examples.push(problems::Example {
            input: input_content,
            output: output_content,
        });

        count += 1;
    }

    Ok((examples, count))
}

pub fn read_insertable_problem(path: &str) -> ServiceResult<problems::InsertableProblem> {
    let info_path = path.to_string() + "/Info.toml";
    let description_path = path.to_string() + "/Description.md";
    let examples_path = path.to_string() + "/Examples";
    let settings_path = path.to_string() + "/Settings.toml";

    let info = read_info(&info_path)?;
    let description = read_description(&description_path)?;
    let (examples, example_count) = read_examples(&examples_path)?;
    let settings = read_settings(&settings_path)?;

    let contents = problems::ProblemContents {
        description: description,
        example_count: example_count,
        examples: examples,
    };

    Ok(problems::InsertableProblem {
        title: info.title,
        tags: info.tags,
        difficulty: info.difficulty,
        contents: serde_json::to_string(&contents).unwrap(),
        settings: serde_json::to_string(&settings).unwrap(),
        is_released: false,
    })
}

fn hash_token<D: Digest>(key: &str, output: &mut [u8]) {
    let mut hasher = D::new();
    hasher.update(key.as_bytes());
    output.copy_from_slice(&hasher.finalize())
}

fn get_stripped_md5_output(output: &str) -> String {
    let mut buf = [0u8; 16];
    hash_token::<Md5>(output.trim_end(), &mut buf);
    (&buf[..]).to_vec().encode_hex::<String>()
}

#[derive(Debug, Clone, Serialize)]
struct NormalTestCaseInfo {
    input_name: String,
    input_size: i32,
    output_name: String,
    output_size: i32,
    stripped_output_md5: String,
}

#[derive(Debug, Clone, Serialize)]
struct SpjTestCaseInfo {
    input_name: String,
    input_size: i32,
}

fn prepare_normal_test_cases(path: &str) -> ServiceResult<i32> {
    let mut count = 0;
    let mut test_cases: BTreeMap<String, NormalTestCaseInfo> = BTreeMap::new();

    loop {
        let id = count + 1;
        let input_name = id.to_string() + ".in";
        let mut input_file = match File::open(path.to_string() + "/" + &input_name) {
            Ok(file) => file,
            Err(_) => {
                info!("Can't find file {}", path.to_string() + "/" + &input_name);
                break;
            }
        };
        let mut input_content = String::new();
        input_file.read_to_string(&mut input_content)?;

        let output_name = id.to_string() + ".out";
        let mut output_file = match File::open(path.to_string() + "/" + &output_name) {
            Ok(file) => file,
            Err(_) => {
                info!("Can't find file {}", path.to_string() + "/" + &output_name);
                break;
            }
        };
        let mut output_content = String::new();
        output_file.read_to_string(&mut output_content)?;

        test_cases.insert(
            id.to_string(),
            NormalTestCaseInfo {
                input_name: input_name,
                input_size: input_content.len() as i32,
                output_name: output_name,
                output_size: output_content.len() as i32,
                stripped_output_md5: get_stripped_md5_output(&output_content),
            },
        );

        count += 1;
    }

    if count == 0 {
        let hint = String::from("Need at least one test case.");
        return Err(ServiceError::BadRequest(hint));
    }

    let info = serde_json::json!({
        "test_case_number": count,
        "spj": false,
        "test_cases": test_cases,
    });

    let mut file = File::create(&(path.to_string() + "/info"))?;
    file.write_all(info.to_string().as_bytes())?;

    Ok(count)
}

fn prepare_spj_test_case(path: &str) -> ServiceResult<i32> {
    let mut count = 0;
    let mut test_cases: BTreeMap<String, SpjTestCaseInfo> = BTreeMap::new();

    loop {
        // check if spj_src.c exists
        if count == 0 {
            File::open(path.to_string() + "/spj_src.c")?;
        }
        let name = (count + 1).to_string() + ".in";
        let mut file = match File::open(path.to_string() + "/" + &name) {
            Ok(file) => file,
            Err(_) => {
                info!("Can't find file {}", path.to_string() + "/" + &name);
                break;
            }
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        test_cases.insert(
            count.to_string(),
            SpjTestCaseInfo {
                input_name: name,
                input_size: content.len() as i32,
            },
        );

        count += 1;
    }

    if count == 0 {
        let hint = String::from("Need at least one test case.");
        return Err(ServiceError::BadRequest(hint));
    }

    let info = serde_json::json!({
        "test_case_number": count,
        "spj": true,
        "test_cases": test_cases,
    });

    let mut file = File::create(path.to_string() + "/" + "info").expect("Error creating info");
    file.write_all(info.to_string().as_bytes())
        .expect("Error writing info");

    Ok(count)
}

pub fn prepare_test_cases(path: &str, is_spj: bool) -> ServiceResult<i32> {
    if is_spj {
        Ok(prepare_spj_test_case(path)?)
    } else {
        Ok(prepare_normal_test_cases(path)?)
    }
}
