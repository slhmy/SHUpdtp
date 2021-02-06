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
  - `result` Option<JudgeResult>
  - `submit_time` NaiveDateTime
  - `is_accepted` Option<bool>
  - `finish_time` Option<NaiveDateTime>
