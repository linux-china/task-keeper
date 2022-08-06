<!-- Keep a Changelog guide -> https://keepachangelog.com -->

# Task Keeper Changelog

## [Unreleased]

### Changed

- Command line with pipes supported in Markdown: `curl --silent https://httpbin.org/ip | jq '.origin'`
- Yarn support: if `"packageManager"` in `package.json` contains `yarn`, then use `yarn run` instead of `npm run` 
- Ignore runner whe no tasks found

## [0.1.0] - 2022-08-05

### Added

- Task Runner support: make, npm, deno, just, fleet, Rakefile, invoke, task, cargo-make, proc, markdown
- .env support by default: `tk --no-dotenv` to disable
