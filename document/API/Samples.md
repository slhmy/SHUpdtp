# Samples

## Create `POST /samples`
### Body `Json`
- `problem_id`!
- `src`! code string
- `language`! content of this string can be `c` `cpp` `java` `py2` `py3`
- `description` describe what is the sample be used for
### Return `Json`
Uuid
### Explain
Create a sample, sample code will be judged.
The Uuid return refers to sample's id.

## Get Sample Details `GET /samples/{id}`
### Return `Json`
Sample
- `submission_id` Uuid
- `description` Option<String>
- `submission` Submission
  - `id` Uuid
  - `problem_id` i32
  - `user_id` i32
  - `region` Option<String>
  - `state` String
  - `settings` JudgeSettings
    - `language_config` LanguageConfig
      - `compile` CompileConfig
        - `src_name` String
        - `exe_name` String
        - `max_cpu_time` i32
        - `max_real_time` i32
        - `max_memory` i32
        - `compile_command` String
      - `run` RunConfig
        - `command` String
        - `seccomp_rule` Option<String>
        - `env` Vec<String>
        - `memory_limit_check_only` i32
    - `src` String
    - `max_cpu_time` i32
    - `max_memory` i32
    - `test_case_id` Option<String>
    - `test_case` Option<Vec<TestCase>> Additional TestCases, we don't expect to have this. So it's always `null`.
      - `input` string
      - `output` string
    - `spj_version` Option<String>
    - `spj_config` Option<SpjConfig>
      - `exe_name` String
      - `command` String
      - `seccomp_rule` String
    - `spj_compile_config` Option<SpjCompileConfig>
      - `src_name` String
      - `exe_name` String
      - `max_cpu_time` i32
      - `max_real_time` i32
      - `max_memory` i32
      - `compile_command` String
    - `spj_src` Option<String>
    - `output` bool
  - `result` Option<JudgeResult> This where the result saved.
    - `err` Option<String> If it is `null`, then the result is effective.
    - `err_reason` Option<String> If err is not `null`, then this column is used to describe the err.
    - `is_accepted` Option<bool>
    - `details` Option<Vec<JudgeResultData>>
      - `cpu_time` i32
      - `real_time` i32
      - `memory` i32
      - `signal` i32
      - `exit_code` i32
      - `error` String See more in Explain
      - `result` String See more in Explain
      - `test_case` String
      - `output_md5` Option<String>
      - `output` Option<String>
  - `submit_time` NaiveDateTime
  - `is_accepted` Option<bool>
  - `finish_time` Option<NaiveDateTime>
### Explain
`result`
- WRONG_ANSWER = -1 (this means the process exited normally, but the answer is wrong)
- SUCCESS = 0 (this means the answer is accepted)
- CPU_TIME_LIMIT_EXCEEDED = 1
- REAL_TIME_LIMIT_EXCEEDED = 2
- MEMORY_LIMIT_EXCEEDED = 3
- RUNTIME_ERROR = 4
- SYSTEM_ERROR = 5

`error`
- SUCCESS = 0
- INVALID_CONFIG = -1
- CLONE_FAILED = -2
- PTHREAD_FAILED = -3
- WAIT_FAILED = -4
- ROOT_REQUIRED = -5
- LOAD_SECCOMP_FAILED = -6
- SETRLIMIT_FAILED = -7
- DUP2_FAILED = -8
- SETUID_FAILED = -9
- EXECVE_FAILED = -10
- SPJ_ERROR = -11

## Get List `GET /samples`
### Params
- `description_filter` Option<String>
- `limit` i32
- `offset` i32
### Return `Json`
[SlimSample]
- `submission_id` Uuid
- `description` Option<String>
- `submission_state` String
- `is_accepted` Option<bool>
- `submit_time` NaiveDateTime

## Delete a Sample `DELETE /samples/{id}`
### Return `null`
200 means success