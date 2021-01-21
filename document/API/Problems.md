# Problems

## Get `Get /problems/{id}`
### Return `Json`
Problem
- `id` int
- `info` ProblemInfo
  - `title` string
  - `tags` [string]
  - `difficulty` double
- `contents` ProblemContents
  - `description` nullable string
  - `example_count` int
  - `examples` [Example]
    - `input` string
    - `output` string
- `settings` ProblemSettings
  - `is_spj` bool
  - `high_performance_max_cpu_time` int
  - `high_performance_max_memory` int
  - `other_max_cpu_time` int
  - `other_max_memory` other_max_memory
  - `opaque_output` bool
  - `test_case_count` nullable int
- `is_released` bool
### Explain
WIP