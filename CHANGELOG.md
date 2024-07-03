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


