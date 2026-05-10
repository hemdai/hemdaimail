# Contributing to Hemdaimail

First off, thank you for considering contributing to Hemdaimail! It's people like you that make Hemdaimail such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by the [Hemdaimail Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

- **Check if the bug has already been reported** by searching on GitHub under [Issues](https://github.com/hemdai/hemdaimail/issues).
- If you can't find an open issue addressing the problem, [open a new one](https://github.com/hemdai/hemdaimail/issues/new).

### Suggesting Enhancements

- Open a new issue with a clear title and description.
- Provide a step-by-step description of the suggested enhancement.

### Your First Code Contribution

1. Fork the repo.
2. Clone your fork.
3. Create a new branch: `git checkout -b my-feature-branch`.
4. Make your changes.
5. Run tests: `cargo test --workspace` and `npm run test` (in `apps/web`).
6. Push to your fork and submit a pull request.

## Styleguides

### Rust Styleguide

- Use `cargo fmt` to format your code.
- Follow the patterns established in the existing modules (Axum, SQLx).

### JavaScript Styleguide

- Use Prettier for formatting.
- Use ESLint to check for common mistakes.

## Technical Documentation

Refer to the `docs/` directory for detailed architectural designs and production readiness strategies.
