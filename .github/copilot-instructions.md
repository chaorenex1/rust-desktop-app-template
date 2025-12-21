# Copilot instructions (code-ai-assistant)

## Big picture
- Monorepo: Vue 3 + Vite frontend in `frontend/`, Tauri v2 Rust backend in `src-tauri/`.
- Frontend talks to Rust via Tauri `invoke()` commands (see `frontend/src/services/tauri/commands.ts`), and via Tauri events for streaming (`ai-response`).

## Day-to-day workflows
- Install deps: `pnpm install`
- Run full desktop app (recommended): `pnpm tauri:dev` (runs Vite via `beforeDevCommand` in `src-tauri/tauri.conf.json`).
- Frontend only: `pnpm dev` (delegates to `pnpm --dir frontend dev`).
- Build desktop app: `pnpm tauri:build`
- Frontend formatting: `pnpm --dir frontend format` / `pnpm --dir frontend format:check`
- Rust tests (backend crate): `cd src-tauri && cargo test`

## Tauri/Rust backend conventions
- App startup is orchestrated in `src-tauri/src/main.rs`:
  - `core::app::init()` loads config + initializes data dir.
  - `utils::logging::init_tracing()` sets up file + console tracing.
  - `database::connection::init()` sets up the pool and runs migrations in a background task.
- IPC commands live under `src-tauri/src/tauri/` and are registered in the single `invoke_handler![]` list in `src-tauri/src/main.rs`.
  - When adding a new command: implement `#[tauri::command]` in the appropriate module, export it from `src-tauri/src/tauri/mod.rs`, then add it to `invoke_handler![]`.

## Frontend ↔ backend integration patterns
- Prefer the typed wrappers in `frontend/src/services/tauri/commands.ts` instead of calling `invoke()` inline.
  - Keep the backend command name in snake_case (e.g. `send_chat_message_streaming`) and map payload fields explicitly (e.g. `context_files`).
  - Convert backend snake_case response fields into camelCase on the frontend (example: `line_count` → `lineCount` in `readFile()`).
- Streaming chat pattern:
  - Rust: `send_chat_message_streaming` returns a `request_id` immediately and emits `ai-response` events with JSON payload `{ request_id, delta, done }`.
  - Vue: `frontend/src/components/chat/ChatPanel.vue` listens to `ai-response` and appends `delta` to the last assistant message.

## Data, settings, and migrations
- Config is loaded/merged in `src-tauri/src/config/loader.rs` (defaults + optional `config.toml` + env). Defaults are defined in `src-tauri/src/config/schema.rs`.
- Default data dir is under the user home: `~/code-ai-assistant` (see `get_default_data_dir()`), and logs default to `${data_dir}/logs`.
- SQLite + SeaORM:
  - Connection/pool: `src-tauri/src/database/connection.rs`
  - Migrations: `src-tauri/src/migration/` (registered in `src-tauri/src/migration/mod.rs`) and auto-run at app start.
- Settings persistence:
  - `get_settings`/`save_settings` in `src-tauri/src/tauri/settings_commands.rs` store a single JSON blob under key `user_config`.
  - Frontend merges settings during bootstrap in `frontend/src/stores/appStore.ts` and saves via `JSON.stringify(settings)`.

## Path handling (project-specific)
- Treat `/` as the canonical separator everywhere (even on Windows).
  - Frontend helpers: `frontend/src/utils/pathUtils.ts` (use `normalizePath()` + `joinPath()`; avoid manual string concatenation).
  - Backend helper: `src-tauri/src/utils/fs.rs::normalize_path()` for paths returned to the frontend.
- See `docs/PATH_HANDLING.md` for rationale and migration examples.
