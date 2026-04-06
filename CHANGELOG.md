# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- YAML parser implementation
- Code generator implementation
- Scheduler and queue system
- CLI interface
- Backend abstraction layer
- Tool implementations (file, web, shell)
- Agent orchestration system

---

## [0.1.0] - 2026-04-06

### Added
- Initial project structure and repository setup
- Dual-licensing (MIT OR Apache-2.0)
- Error handling module (`error.rs`) with comprehensive error types
- Cargo.toml with production-ready dependencies
- Comprehensive README with framework overview and examples
- 53 example workflows across 19 categories
- 8 Architecture Decision Records (ADRs) documenting key design decisions
- Research documentation and implementation plans
- Complete documentation structure in `opencode/docs/`

### Documentation
- Added LICENSE file (MIT and Apache-2.0 dual license)
- Added INSTALL.md with platform-specific installation instructions
- Added ENVIRONMENT_VARIABLES.md with complete configuration reference
- Added CONTRIBUTING.md with development workflow and coding standards
- Added .env.example with all supported environment variables
- Added inline documentation to Cargo.toml explaining dependency choices
- Added comprehensive README (650+ lines) covering framework features and usage

### Infrastructure
- Set up project repository structure
- Configured release build profile with LTO and optimizations
- Configured dev build profile for faster iteration
- Created .gitignore for Rust builds, models, logs, and workspace directories

### Project Status
- **Phase 1 (Foundation)**: Complete
  - Repository structure established
  - YAML schema specification complete
  - Documentation structure implemented
  - Error handling module implemented

- **Phase 2 (MVP Queue & Scheduler)**: Pending
  - Queue state machine
  - Scheduler implementation
  - CLI control surface

- **Phase 3 (CLI & Backends)**: Pending
  - Backend abstraction layer
  - Provider implementations (Ollama, LM Studio, llama.cpp)
  - Networking boundary

### Notes
- This is the initial release of the framework
- Core functionality is under active development
- Example workflows and schema documentation are complete and ready for use
- Focus is on YAML-based declarative workflow definitions with direct execution and code generation modes

---

## [Version Reference]

| Version | Date | Status | Key Features |
|----------|--------|---------|--------------|
| 0.1.0 | 2026-04-06 | Initial release | Documentation, error handling, project structure |

---

## Change Types

- **Added**: New features
- **Changed**: Changes in existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Vulnerability fixes

---

## How to Read This Changelog

### [Unreleased]
Features and changes planned for future releases but not yet released.

### [X.Y.Z] - YYYY-MM-DD
Released version with date.

### Added
New features added to the project.

### Changed
Changes to existing functionality that maintain backward compatibility.

### Deprecated
Features that will be removed in future versions.

### Removed
Features removed from the project.

### Fixed
Bug fixes and error corrections.

### Security
Security vulnerability fixes and patches.

---

## Semantic Versioning

Given a version number MAJOR.MINOR.PATCH:
- **MAJOR**: Incompatible API changes
- **MINOR**: Backwards-compatible functionality additions
- **PATCH**: Backwards-compatible bug fixes

Example: 1.2.3 → 2.0.0 (breaking changes), 1.2.3 → 1.3.0 (new features), 1.2.3 → 1.2.4 (bug fixes)

---

## Contribution

To add an entry to this changelog:

1. Add entry under `[Unreleased]` or new version section
2. Use appropriate category (Added, Changed, Fixed, etc.)
3. Format as: `- [Category] Description of change`
4. Reference related issues if applicable
5. Update version number when releasing

---

## Links

- [Full Documentation](README.md)
- [Installation Guide](INSTALL.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Example Workflows](opencode/docs/reports/requirements/example-workflows/)
