## 2024-08-18, Version v0.1.9
### Commits
- [[`70317c8b44`](https://github.com/kingsleyh/pg_filters/commit/70317c8b44af70e0dfd9af9dc45065bef3c52181)] 0.1.9 (kingsley.hendrickse)
- [[`2f2728f895`](https://github.com/kingsleyh/pg_filters/commit/2f2728f895d17b3fa8a341338dafae734920e1bb)] fix (kingsley.hendrickse)
- [[`6acb2414a6`](https://github.com/kingsleyh/pg_filters/commit/6acb2414a6585165226591de74a4cf74c7a8aa84)] fix (kingsley.hendrickse)
- [[`8ed7b525b4`](https://github.com/kingsleyh/pg_filters/commit/8ed7b525b42143ce3d8340aa277ef139baf1bd22)] try to auto convert types for operators where incorrect types were supplied for the filter value for the column type (kingsley.hendrickse)
- [[`472b9a2b4f`](https://github.com/kingsleyh/pg_filters/commit/472b9a2b4fa736a1d2ea3570ef98332fe0fa6928)] fix (kingsley.hendrickse)
- [[`3d04399080`](https://github.com/kingsleyh/pg_filters/commit/3d04399080d9c947a85bda727abfc987e81932ac)] changelog (kingsley.hendrickse)

### Stats
```diff
 CHANGELOG.md                 | 17 ++++++++-
 Cargo.toml                   |  2 +-
 src/lib/filtering.rs         | 95 ++++++++++++++++++++++++++++++++++++++++++++-
 tests/unit/filtering_test.rs | 64 ++++++++++++++++++++++++++++++-
 4 files changed, 176 insertions(+), 2 deletions(-)
```

## 2024-08-18, Version v0.1.8
### Commits
- [[`4643f01126`](https://github.com/kingsleyh/pg_filters/commit/4643f01126528f61f815ca90123aaad5aee2d743)] fix (kingsley.hendrickse)
- [[`f33cb00c40`](https://github.com/kingsleyh/pg_filters/commit/f33cb00c40ea67493af55acbac7af4a8b9b4d4c8)] update (kingsley.hendrickse)
- [[`e605c1b253`](https://github.com/kingsleyh/pg_filters/commit/e605c1b2535b9483f97583bdba00cc9843f565bb)] fix (kingsley.hendrickse)
- [[`c1e5693bca`](https://github.com/kingsleyh/pg_filters/commit/c1e5693bca3cfdd7cd7f78cee9f81909c2db0b48)] bump version (kingsley.hendrickse)
- [[`001bc1f0fd`](https://github.com/kingsleyh/pg_filters/commit/001bc1f0fde7d2d7f2a4faf4a1c8be317af4c1fb)] attempt to convert value for filter rule into the desired type (kingsley.hendrickse)

### Stats
```diff
 Cargo.lock                   |   2 +-
 Cargo.toml                   |   2 +-
 src/lib/filtering.rs         | 105 +++++++++++++++++++++++++++++---------------
 tests/unit/filtering_test.rs |  78 ++++++++++++++++++++++-----------
 4 files changed, 124 insertions(+), 63 deletions(-)
```

## 2024-08-10, Version v0.1.7
### Commits
- [[`2a89a3ed24`](https://github.com/kingsleyh/pg_filters/commit/2a89a3ed242e82caf56f7ace3b7fc4c4d6aae16d)] 0.1.7 (kingsley.hendrickse)
- [[`e68c89c9f0`](https://github.com/kingsleyh/pg_filters/commit/e68c89c9f0baa6ecfcfb49757f3de862fc38d156)] fix bug with conditional sql still being included for invalid rules (kingsley.hendrickse)
- [[`7b3f1dd9a4`](https://github.com/kingsleyh/pg_filters/commit/7b3f1dd9a4ce3737ec09b1263d1fb858e2879474)] auto release to crates (kingsley.hendrickse)
- [[`8b78708ec6`](https://github.com/kingsleyh/pg_filters/commit/8b78708ec60a3bae3caf274ad9a29ac9963fb8b3)] auto release tags (kingsley.hendrickse)
- [[`3192ce09a7`](https://github.com/kingsleyh/pg_filters/commit/3192ce09a7409093a7bb090dd57e3251b2b2b4bb)] fix (kingsley.hendrickse)
- [[`15d6e064f3`](https://github.com/kingsleyh/pg_filters/commit/15d6e064f37fbd8bd6e5ba621f50ff08a6daf33c)] fix (kingsley.hendrickse)
- [[`40716871b8`](https://github.com/kingsleyh/pg_filters/commit/40716871b8e3fbdc20c123121578e34a678a9ddd)] changelog (kingsley.hendrickse)

### Stats
```diff
 .github/workflows/release.yml         | 77 ++++++++++++++++++++++++++++++++++++-
 CHANGELOG.md                          | 19 +++++++++-
 Cargo.lock                            |  2 +-
 Cargo.toml                            |  2 +-
 src/lib/filtering.rs                  | 12 ++++--
 src/lib/mod.rs                        |  4 +--
 tests/integration/integration_test.rs |  7 ++-
 tests/unit/filtering_test.rs          | 27 ++++++++++---
 8 files changed, 134 insertions(+), 16 deletions(-)
```

## 2024-08-10, Version v0.1.6
### Commits
- [[`49f896c3b6`](https://github.com/kingsleyh/pg_filters/commit/49f896c3b6e24cdd934226ddc5382779ce013b46)] 0.1.6 (kingsley.hendrickse)
- [[`ac832a39e1`](https://github.com/kingsleyh/pg_filters/commit/ac832a39e1e17b20c4cd033b0200bfeb3fd3f89c)] readme (kingsley.hendrickse)
- [[`6c939298b3`](https://github.com/kingsleyh/pg_filters/commit/6c939298b3b163fb93ffb12040ecd14b910c2cd8)] tidy (kingsley.hendrickse)
- [[`5b339a1a3e`](https://github.com/kingsleyh/pg_filters/commit/5b339a1a3e60c7c64f78b2d1bfc9d9f3c43e3bcc)] allow sql to be accessed from filtering_options (kingsley.hendrickse)
- [[`c7f96c36fc`](https://github.com/kingsleyh/pg_filters/commit/c7f96c36fc82fb0ea20a9f152e15a8297ed4695e)] readme (kingsley.hendrickse)
- [[`1e0bbc1c21`](https://github.com/kingsleyh/pg_filters/commit/1e0bbc1c21b83f7ba77816dbaf8eecc9bf673b30)] docs (kingsley.hendrickse)

### Stats
```diff
 Cargo.toml                   |  2 +-
 README.md                    | 40 ++++++++++++++++++++++++--
 src/lib/filtering.rs         |  2 +-
 src/lib/mod.rs               | 18 +++++++-----
 tests/unit/filtering_test.rs | 70 +++++++++++++++++++++++----------------------
 5 files changed, 87 insertions(+), 45 deletions(-)
```

## 2024-08-04, Version v0.1.5
### Commits
- [[`bdbe472276`](https://github.com/kingsleyh/pg_filters/commit/bdbe472276e0e814bd2a58f7e06b6307c343e8c8)] prepare release v0.1.5 (kingsley.hendrickse)
- [[`7ef861a5f3`](https://github.com/kingsleyh/pg_filters/commit/7ef861a5f34b012261f42269c0f60868e70c216d)] fix (kingsley.hendrickse)
- [[`52eff6ea2a`](https://github.com/kingsleyh/pg_filters/commit/52eff6ea2ac1cd1c27e1cb7def1a3436150ae13d)] fix (kingsley.hendrickse)
- [[`e131beaeab`](https://github.com/kingsleyh/pg_filters/commit/e131beaeab556147be1b0ca9d7e6434053d6f450)] readme (kingsley.hendrickse)
- [[`ae08a420e1`](https://github.com/kingsleyh/pg_filters/commit/ae08a420e14906c4e906143e932ca7c444121d55)] specs (kingsley.hendrickse)
- [[`49eaa6c1a3`](https://github.com/kingsleyh/pg_filters/commit/49eaa6c1a337f2880c1719893de4a99ffc016a3f)] specs (kingsley.hendrickse)
- [[`ea89dd57c0`](https://github.com/kingsleyh/pg_filters/commit/ea89dd57c08226940be21805ae10e274248a8067)] refactor (kingsley.hendrickse)
- [[`ec7b9372e3`](https://github.com/kingsleyh/pg_filters/commit/ec7b9372e3dcf2080c05dfa4d65946cda78deddd)] fix (kingsley.hendrickse)
- [[`9d9b5a9167`](https://github.com/kingsleyh/pg_filters/commit/9d9b5a916776d473bd7b01a393864d196113a5c4)] refactor to pass a ColumnName to the FilteringRule so we can handle the sql generation better with types (kingsley.hendrickse)
- [[`a551b83930`](https://github.com/kingsleyh/pg_filters/commit/a551b83930a8e2d3bf2278694e2a6d66e053ff07)] fix bug when 0 total_records created a negative offset (kingsley.hendrickse)
- [[`df2010fc83`](https://github.com/kingsleyh/pg_filters/commit/df2010fc8389712a7db73ac295de8412ab406435)] readme (kingsley.hendrickse)
- [[`dc906a984a`](https://github.com/kingsleyh/pg_filters/commit/dc906a984a3d349272ec391ac59a4603b9182388)] update changelog (kingsley.hendrickse)

### Stats
```diff
 .gitignore                            |    1 +-
 CHANGELOG.md                          |   20 +-
 Cargo.lock                            | 2260 ++++++++++++++++++++++++++++++++++-
 Cargo.toml                            |    8 +-
 README.md                             |   12 +-
 src/lib/filtering.rs                  |  756 +++++++----
 src/lib/mod.rs                        |   63 +-
 src/lib/pagination.rs                 |   21 +-
 src/lib/sorting.rs                    |   16 +-
 tests/combined_test.rs                |  262 +----
 tests/filtering_rule_test.rs          |  200 +---
 tests/filtering_test.rs               |  567 +---------
 tests/integration/integration_test.rs |  190 +++-
 tests/integration/mod.rs              |   84 +-
 tests/mod.rs                          |    2 +-
 tests/pagination_test.rs              |  106 +--
 tests/sorting_test.rs                 |  170 +---
 tests/unit/combined_test.rs           |  293 ++++-
 tests/unit/filtering_rule_test.rs     |  403 ++++++-
 tests/unit/filtering_test.rs          |  708 +++++++++++-
 tests/unit/mod.rs                     |    5 +-
 tests/unit/pagination_test.rs         |  154 ++-
 tests/unit/sorting_test.rs            |  170 +++-
 23 files changed, 4863 insertions(+), 1608 deletions(-)
```

## 2024-07-03, Version v0.1.4
### Commits
- [[`5d9e57db54`](https://github.com/kingsleyh/pg_filters/commit/5d9e57db5427272adeb85af646dd6f5e59f38263)] bump version to 0.1.4 for release (kingsley.hendrickse)
- [[`e334df6ee4`](https://github.com/kingsleyh/pg_filters/commit/e334df6ee482797f7a0fadee0ce9f0ac30f8dc71)] add starts with and ends with (kingsley.hendrickse)
- [[`64dd0a8d80`](https://github.com/kingsleyh/pg_filters/commit/64dd0a8d809a6386f5115eab4b410b2979ead093)] fix coverage (kingsley.hendrickse)
- [[`cc9dc489ab`](https://github.com/kingsleyh/pg_filters/commit/cc9dc489abb06e5029f03fe92813d1b925813ec4)] update changelog (kingsley.hendrickse)

### Stats
```diff
 CHANGELOG.md                 | 15 +++++++++++-
 Cargo.lock                   |  2 +-
 Cargo.toml                   |  2 +-
 src/lib/filtering.rs         | 24 +++++++++++++++++-
 tests/filtering_rule_test.rs | 33 +++++++++++++++++++++++-
 tests/filtering_test.rs      | 64 +++++++++++++++++++++++++++++++++++++++++++++-
 tests/pagination_test.rs     | 11 +++++++-
 tests/sorting_test.rs        | 21 +++++++++++++++-
 8 files changed, 169 insertions(+), 3 deletions(-)
```

## 2024-06-30, Version v0.1.3
### Commits
- [[`4315637af3`](https://github.com/kingsleyh/pg_filters/commit/4315637af35545f83ab2af2c10c94c59dc7c9921)] bump version for release to crates.io (kingsley.hendrickse)
- [[`079bf6bf54`](https://github.com/kingsleyh/pg_filters/commit/079bf6bf544fdd3b94f9427b572587f3859fe724)] reformat readme and add changelog (kingsley.hendrickse)
- [[`c48123389b`](https://github.com/kingsleyh/pg_filters/commit/c48123389b6e4232c8d1acaf80581f5629a3f3d1)] readme (kingsley.hendrickse)

### Stats
```diff
 CHANGELOG.md | 52 ++++++++++++++++++++++++++++++++++++++++++++++++++++
 Cargo.lock   |  2 +-
 Cargo.toml   |  2 +-
 README.md    | 11 ++++++++---
 4 files changed, 62 insertions(+), 5 deletions(-)
```

## 2024-06-30, Version v0.1.2
### Commits
- [[`1ffae66cdf`](https://github.com/kingsleyh/pg_filters/commit/1ffae66cdf10a5b18a7e76b004c3912c554a17f2)] bump version (kingsley.hendrickse)
- [[`761eaffb6e`](https://github.com/kingsleyh/pg_filters/commit/761eaffb6ed28723911c9adb9984a24d1aa7b0c1)] revert version (kingsley.hendrickse)
- [[`f0ff8c6c14`](https://github.com/kingsleyh/pg_filters/commit/f0ff8c6c14a54bcd1f95505cb5344f5854bd3a34)] bump version (kingsley.hendrickse)
- [[`3dd5c4f1f0`](https://github.com/kingsleyh/pg_filters/commit/3dd5c4f1f056502e8fa021aaa525c107208310c6)] breaking change: re-order FilterRule::new parameters to make more sense when reading the code (kingsley.hendrickse)
- [[`67b9b9346d`](https://github.com/kingsleyh/pg_filters/commit/67b9b9346dfd5cfed1b51acf7dc89254fa4cf83c)] readme (kingsley.hendrickse)
- [[`6301389a4b`](https://github.com/kingsleyh/pg_filters/commit/6301389a4ba5f6af8b130b033ec52e2bdb864469)] ci (kingsley.hendrickse)
- [[`50508c4a2e`](https://github.com/kingsleyh/pg_filters/commit/50508c4a2ebebf9019082e92ff36fd8af5cbdc24)] ci (kingsley.hendrickse)
- [[`4d47b7a1b1`](https://github.com/kingsleyh/pg_filters/commit/4d47b7a1b1e8fa5593750c78739761a5b45d072e)] try add code coverage (kingsley.hendrickse)
- [[`610d1ab9ea`](https://github.com/kingsleyh/pg_filters/commit/610d1ab9ea33c74d13b6fd773073d0ca0e77da47)] try add code coverage (kingsley.hendrickse)
- [[`3ae8dd5977`](https://github.com/kingsleyh/pg_filters/commit/3ae8dd5977ee5e39956b0643b79f8354b9ea96c9)] tidy (kingsley.hendrickse)
- [[`433703322f`](https://github.com/kingsleyh/pg_filters/commit/433703322fa70fe2188f10c5dd84371b19f3ae4c)] readme (kingsley.hendrickse)
- [[`4443007c44`](https://github.com/kingsleyh/pg_filters/commit/4443007c4460a0b756f1b422c7c9d007cdce37ae)] fix (kingsley.hendrickse)
- [[`dc231931e1`](https://github.com/kingsleyh/pg_filters/commit/dc231931e1035084a4048ec085b028dd3e58efe6)] cargo (kingsley.hendrickse)
- [[`6aad8efdfd`](https://github.com/kingsleyh/pg_filters/commit/6aad8efdfd4582bc7d2c96dd1b8241852d8fd0d0)] update (kingsley.hendrickse)
- [[`4c70de559e`](https://github.com/kingsleyh/pg_filters/commit/4c70de559ed6de695fa294e0cd6731c941725386)] fix (kingsley.hendrickse)
- [[`391b33bb8c`](https://github.com/kingsleyh/pg_filters/commit/391b33bb8c23a82a589893d4b6fd6d9b58682304)] fix (kingsley.hendrickse)
- [[`dff059a2e9`](https://github.com/kingsleyh/pg_filters/commit/dff059a2e95f5c96c7a8801971eecda41e5de68e)] fix (kingsley.hendrickse)
- [[`30b23d3004`](https://github.com/kingsleyh/pg_filters/commit/30b23d3004ef3deff3ddfb75c25c4c6b62836a52)] fix (kingsley.hendrickse)
- [[`0f36f26142`](https://github.com/kingsleyh/pg_filters/commit/0f36f261424cd63466ce54f5a580bc1aaa2aa13f)] add ci (kingsley.hendrickse)
- [[`6f41ab65f6`](https://github.com/kingsleyh/pg_filters/commit/6f41ab65f66eb1479abbd66fb8e99f291a14c4e7)] readme (kingsley.hendrickse)
- [[`c250ef6fba`](https://github.com/kingsleyh/pg_filters/commit/c250ef6fba352c58e4b63181be497a5fa7d19286)] updates (kingsley.hendrickse)
- [[`14d4c005d0`](https://github.com/kingsleyh/pg_filters/commit/14d4c005d0f0399b8174d8fab28f0d91e891f052)] updaes (kingsley.hendrickse)
- [[`77a1f2797d`](https://github.com/kingsleyh/pg_filters/commit/77a1f2797d0a4ff74a4a6ca1957fa5e07bdaf28c)] readme (kingsley.hendrickse)
- [[`e2313a37c3`](https://github.com/kingsleyh/pg_filters/commit/e2313a37c38d6d5b5a2ee0ac1588527af9e79c1e)] readme (kingsley.hendrickse)
- [[`0cbdcf6a52`](https://github.com/kingsleyh/pg_filters/commit/0cbdcf6a523b04b208fce704e49f69b762ac8de9)] readme (kingsley.hendrickse)
- [[`6e880d571a`](https://github.com/kingsleyh/pg_filters/commit/6e880d571a7d1a5dc0c82d351188ed622e919310)] initial (kingsley.hendrickse)

### Stats
```diff
 .github/workflows/ci.yml     |  73 ++++++-
 .gitignore                   |   1 +-
 Cargo.lock                   |   7 +-
 Cargo.toml                   |  19 ++-
 LICENSE-APACHE               | 201 ++++++++++++++++++-
 LICENSE-MIT                  |  21 ++-
 README.md                    |  98 +++++++++-
 fix.sh                       |   1 +-
 src/lib/filtering.rs         | 368 ++++++++++++++++++++++++++++++++-
 src/lib/mod.rs               | 160 ++++++++++++++-
 src/lib/pagination.rs        | 144 +++++++++++++-
 src/lib/sorting.rs           | 157 ++++++++++++++-
 tests/combined_test.rs       | 262 +++++++++++++++++++++++-
 tests/filtering_rule_test.rs | 167 +++++++++++++++-
 tests/filtering_test.rs      | 503 ++++++++++++++++++++++++++++++++++++++++++++-
 tests/pagination_test.rs     |  97 ++++++++-
 tests/sorting_test.rs        | 149 +++++++++++++-
 17 files changed, 2428 insertions(+)
```


