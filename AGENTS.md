# Repository Guidelines

## Project Structure & Module Organization
- `frontend/`: Vue 3 + Vite UI with Pinia stores, Element Plus components, and chat/terminal widgets. Entry points live in `src/`, build artifacts in `dist/`.
- `src-tauri/`: Rust backend (Tauri 2) exposing commands, services, and configuration (`src/tauri`, `src/services`). Cargo tasks run from this folder.
- `scripts/`: Utility scripts (e.g., `fetch:codeagent-wrapper`) invoked through `pnpm`.
- Assets such as icons live under `frontend/public/` or `src-tauri/icons/`. Keep new modules consistent with this layout.

## Build, Test, and Development Commands
- `pnpm --dir frontend dev`: Launches the Vue dev server with hot reload.
- `pnpm --dir frontend build`: Type-checks via `vue-tsc` and emits production assets through Vite.
- `pnpm tauri dev` / `pnpm tauri build`: Builds and runs the Tauri shell (requires Rust toolchain).
- `cargo check` in `src-tauri/`: Validates Rust sources quickly; run before submitting backend changes.

## Coding Style & Naming Conventions
- TypeScript/Vue: 2-space indentation, script setup syntax, kebab-case component files. Use camelCase for variables and PascalCase for components & Pinia stores.
- Rust: Follow rustfmt defaults (`cargo fmt`) and keep modules snake_case. Prefer descriptive logging via `tracing`.
- Formatting: run `pnpm --dir frontend format` for Prettier; rely on `cargo fmt` for Rust.

## Testing Guidelines
- Frontend relies on type-checking (`vue-tsc`) and manual QA; add Vitest suites under `frontend/src/__tests__` when feasible.
- Backend tests should use Rust’s builtin `#[cfg(test)]` modules or integration tests in `src-tauri/tests/`.
- Name tests after behavior (e.g., `chat_session_persists_messages`). Ensure commands touching AI mocks include deterministic fixtures.

## Commit & Pull Request Guidelines
- Commits should use imperative subjects (e.g., “Add chat session normalization”) and be scoped to a logical change.
- PRs must describe motivation, key changes, test evidence (`pnpm build`, `cargo check`), and link to relevant issues. Include screenshots/GIFs for UI-visible modifications.
- Keep PRs small and focused; note any follow-up tasks in the description.

## Security & Configuration Tips
- Secrets (API keys, CLI tokens) belong in `.env` / system keyrings; never commit them.
- Review `src-tauri/src/config` when adding new settings. Validate file paths through `normalizePath` before invoking filesystem commands.
