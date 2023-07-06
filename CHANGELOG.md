# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

A [separate changelog is kept for rand_core](rand_core/CHANGELOG.md).

You may also find the [Upgrade Guide](https://rust-random.github.io/book/update.html) useful.

## v0.3.0 - 2023-07-05
### Added
- Game full attributes, including Enum access to enumerated attributes
- Genre full attributes, including Enum access to enumerated attributes
### Project organization
- Update README with the 0.2.0 changes

## v0.2.0 - 2023-07-04
### Added
- Character full attributes, including Enum access to enumerated attributes (Gender and Species)
- Public crate method to access the API response as JSON

## v0.1.0 - 2023-07-01
### Added
- API wrapper using [reqwest](https://docs.rs/reqwest/latest/reqwest/)
- Game abstraction to query Game related info
- Character abstraction to query Character related info
### Project organization
- Documentation markdowns for the repository
