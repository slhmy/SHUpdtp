#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileConfig {
    pub src_name: String,
    pub exe_name: String,
    pub max_cpu_time: i32,
    pub max_real_time: i32,
    pub max_memory: i32,
    pub compile_command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunConfig {
    pub command: String,
    pub seccomp_rule: Option<String>,
    pub env: Vec<String>,
    pub memory_limit_check_only: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub compile: CompileConfig,
    pub run: RunConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpjConfig {
    pub exe_name: String,
    pub command: String,
    pub seccomp_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpjCompileConfig {
    pub src_name: String,
    pub exe_name: String,
    pub max_cpu_time: i32,
    pub max_real_time: i32,
    pub max_memory: i32,
    pub compile_command: String,
}

fn default_env() -> Vec<String> {
    vec![
        "LANG=en_US.UTF-8".to_owned(),
        "LANGUAGE=en_US:en".to_owned(),
        "LC_ALL=en_US.UTF-8".to_owned(),
    ]
}

fn c_lang_config() -> LanguageConfig {
    LanguageConfig {
        compile: CompileConfig {
            src_name: "main.c".to_owned(),
            exe_name: "main".to_owned(),
            max_cpu_time: 3000,
            max_real_time: 5000,
            max_memory: 128 * 1024 * 1024,
            compile_command: "/usr/bin/gcc -DONLINE_JUDGE -O2 -w -fmax-errors=3 -std=c99 {src_path} -lm -o {exe_path}".to_owned(),
        },
        run: RunConfig {
            command: "{exe_path}".to_owned(),
            seccomp_rule: Some("c_cpp".to_owned()),
            env: default_env(),
            memory_limit_check_only: 0,
        }
    }
}

pub fn spj_compile_config() -> SpjCompileConfig {
    SpjCompileConfig {
        src_name: "spj-{spj_version}.cpp".to_owned(),
        exe_name: "spj-{spj_version}".to_owned(),
        max_cpu_time: 3000,
        max_real_time: 5000,
        max_memory: 1024 * 1024 * 1024,
        compile_command: "/usr/bin/g++ -DONLINE_JUDGE -O2 -w -fmax-errors=3 -std=c99 {src_path} -lm -L /test_case/include -o {exe_path}".to_owned(),
    }
}

pub fn spj_config() -> SpjConfig {
    SpjConfig {
        exe_name: "spj-{spj_version}".to_owned(),
        command: "{exe_path} {in_file_path} {user_out_file_path}".to_owned(),
        seccomp_rule: "c_cpp".to_owned(),
    }
}

fn cpp_lang_config() -> LanguageConfig {
    LanguageConfig {
        compile: CompileConfig {
            src_name: "main.cpp".to_owned(),
            exe_name: "main".to_owned(),
            max_cpu_time: 3000,
            max_real_time: 5000,
            max_memory: 128 * 1024 * 1024,
            compile_command: "/usr/bin/g++ -DONLINE_JUDGE -O2 -w -fmax-errors=3 -std=c++11 {src_path} -lm -o {exe_path}".to_owned(),
        },
        run: RunConfig {
            command: "{exe_path}".to_owned(),
            seccomp_rule: Some("c_cpp".to_owned()),
            env: default_env(),
            memory_limit_check_only: 0,
        }
    }
}

fn java_lang_config() -> LanguageConfig {
    LanguageConfig {
        compile: CompileConfig {
            src_name: "Main.java".to_owned(),
            exe_name: "Main".to_owned(),
            max_cpu_time: 5000,
            max_real_time: 10000,
            max_memory: -1,
            compile_command: "/usr/bin/javac {src_path} -d {exe_dir} -encoding UTF8".to_owned(),
        },
        run: RunConfig {
            command: "/usr/bin/java -cp {exe_dir} -XX:MaxRAM={max_memory}k -Djava.security.manager -Dfile.encoding=UTF-8 -Djava.security.policy==/etc/java_policy -Djava.awt.headless=true Main".to_owned(),
            seccomp_rule: None,
            env: default_env(),
            memory_limit_check_only: 1,
        }
    }
}

fn py2_lang_config() -> LanguageConfig {
    LanguageConfig {
        compile: CompileConfig {
            src_name: "solution.py".to_owned(),
            exe_name: "solution.pyc".to_owned(),
            max_cpu_time: 3000,
            max_real_time: 5000,
            max_memory: 128 * 1024 * 1024,
            compile_command: "/usr/bin/python -m py_compile {src_path}".to_owned(),
        },
        run: RunConfig {
            command: "/usr/bin/python {exe_path}".to_owned(),
            seccomp_rule: Some("general".to_owned()),
            env: default_env(),
            memory_limit_check_only: 0,
        },
    }
}

fn py3_lang_config() -> LanguageConfig {
    LanguageConfig {
        compile: CompileConfig {
            src_name: "solution.py".to_owned(),
            exe_name: "__pycache__/solution.cpython-36.pyc".to_owned(),
            max_cpu_time: 3000,
            max_real_time: 5000,
            max_memory: 128 * 1024 * 1024,
            compile_command: "/usr/bin/python3 -m py_compile {src_path}".to_owned(),
        },
        run: RunConfig {
            command: "/usr/bin/python3 {exe_path}".to_owned(),
            seccomp_rule: Some("general".to_owned()),
            env: {
                let mut default_env = default_env();
                default_env.push("PYTHONIOENCODING=UTF-8".to_owned());
                default_env
            },
            memory_limit_check_only: 0,
        },
    }
}

pub fn get_lang_config(language: &str) -> LanguageConfig {
    match language {
        "c" => c_lang_config(),
        "cpp" => cpp_lang_config(),
        "java" => java_lang_config(),
        "py2" => py2_lang_config(),
        "py3" => py3_lang_config(),
        _ => c_lang_config(),
    }
}
