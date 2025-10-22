# Socio AI Agent - Repository Guidelines

## Project Structure & Module Organization
- Backend (Rust): `crates/socio-backend/` with `src/`, `tests/`, and `Cargo.toml`. Key entry points: `crates/socio-backend/src/main.rs`, `crates/socio-backend/src/lib.rs`, OpenAPI in `crates/socio-backend/src/openapi.rs`.
- Frontend (TypeScript): `crates/socio/frontend/` with `src/`, `package.json`, Vite config, tests under `src/**/*.test.ts`.
- Agent Tools: `crates/socio-backend/src/agent_tools/` with tool implementations and handlers.
- Data & Docs: `pgdata/` (local DB volume), `test-data/`, `docs/`, and environment files (`.env`, `.env-example`).

## Build, Test, and Development Commands
- Local dev (backend + frontend): `make dev` (use `EMULATOR=true` to enable fake GCS).
- Backend run only: `make app-run` or `make app-run-o11y` (with New Relic feature).
- Tests (fast path): `make test-fast` (frontend + backend), or full: `make test`.
- Backend tests: `cd crates/socio-backend && DATABASE_URL=... APP_ENV=test cargo test`.
- Frontend tests: `cd crates/socio/frontend && npm ci && npm run test:run`.
- DB: `make db-start`, `make db-migrate`, `make db-connect`, schema dump via `make dump-db-schema`.
 - See API Client Regeneration below for SDK/client generation.

## API Client Regeneration
- TypeScript client: `make generate-api-client`. Starts the backend on a free port (8081–8083), waits for `/health`, then overwrites `crates/socio/frontend/src/api/*` via `openapi-typescript-codegen` from `.../api-docs/openapi.json`. Prereqs: Node, backend compiles. Optional: `EMULATOR=true make generate-api-client`.
- Swift (iOS) SDK: `make generate-swift-sdk`. Uses Docker OpenAPI Generator to write to `mobile/ios/SDK/` (contents replaced), applying patches from `mobile/ios/patches/` if present. Fetches spec from the running backend `.../api-docs/openapi.json` and cleans up.
- Ports: the Makefile probes 8081–8083. If blocked, free them (e.g., `make kill-8081`). Commit regenerated clients with your PR.

## Coding Style & Naming Conventions
- Rust: follow `rust_coding_guidelines.md`. Format with `cargo fmt --all`. Lint with `cargo clippy -- -D warnings` (Makefile enforces additional disallowed methods).
- TypeScript: follow `TYPESCRIPT_CODING_GUIDELINES.md`. Format with `npm run format`; lint with `npm run lint`.
- Naming: modules and files are `snake_case` in Rust, `camelCase` for TS identifiers, `PascalCase` for React components.

## Testing Guidelines
- Rust: unit tests inline (`mod tests`) and integration tests in `crates/socio-backend/tests/*.rs`. Use `serial_test` where required for DB coupling.
- Frontend: Vitest under `crates/socio/frontend/src/**/*.test.ts`; run `npm run test:coverage` for coverage.
- iOS: `make test-ios-unit` (fast) or `make test-ios` (adds UI tests). Aim to cover new logic and edge cases.

## Commit & Pull Request Guidelines
- Commits: concise, imperative subject; optional scope (e.g., "backend: add auth middleware"). Group related changes.
- PRs: include summary, linked issues, and steps to validate. Add screenshots for UI changes and note API/DB migrations. Ensure `make test-fast` passes and code is formatted/linted.

## Security & Configuration Tips
- Copy `.env-example` to `.env` and set `DATABASE_URL`. Use `EMULATOR=true` for local storage via the fake GCS server. Never commit secrets.

## Backend Architecture (for Agents)
- agent_tools module (`crates/socio-backend/src/agent_tools/`)
  - Purpose: AI agent tool implementations for content generation and social media posting.
  - Conventions: one tool per file; tool parameters in `tool_params/`, handlers in `handlers/`; use `AgentToolResponse` for standardized responses.
  - Examples: `crates/socio-backend/src/agent_tools/tool_params/generate_content_tool_params.rs`, `crates/socio-backend/src/agent_tools/handlers/handle_generate_content_tool.rs`.

- routes module (`crates/socio-backend/src/routes/`)
  - Purpose: API routes and HTTP transport. Small handlers that validate/authorize, then delegate to services.
  - Conventions: one handler per file; group by domain in subfolders; annotate every handler with `#[utoipa::path(...)]`; register in the domain `configure_*` and in `routes::config`.
  - Examples: `crates/socio-backend/src/routes/chat.rs`, `crates/socio-backend/src/routes/social.rs`, aggregator: `crates/socio-backend/src/routes/mod.rs`.

- services module (`crates/socio-backend/src/services/`)
  - Purpose: business logic and external integrations, e.g., social media APIs, WebSocket management.
  - Conventions: encapsulate external integrations; inject via `web::Data` in `main.rs`; keep IO boundaries here; support testability by traits where appropriate.
  - Examples: `crates/socio-backend/src/services/social_media.rs`, `crates/socio-backend/src/services/websocket.rs`.

- OpenAPI (`crates/socio-backend/src/openapi.rs`)
  - Purpose: aggregate all documented routes and schemas and export a single spec.
  - Conventions: add each new handler to the `paths(...)` list; ensure all response/request structs derive `ToSchema` so they appear in `components(schemas(...))`.
  - Client export: used by `make generate-api-client` (TypeScript) and `make generate-swift-sdk` (iOS).

- App entry points
  - `crates/socio-backend/src/main.rs`: wiring (pools/clients/services), middleware, Swagger UI, and route configuration via `routes::config`.
  - `crates/socio-backend/src/lib.rs`: app scaffolding types; keep this minimal.

## Frontend Architecture (for Agents)
- Generated API client lives in `crates/socio/frontend/src/api/` (from backend OpenAPI): `core/`, `models/`, `services/`.
  - Strict typing: always import types from `crates/socio/frontend/src/api/models/*` and call generated methods from `crates/socio/frontend/src/api/services/*` instead of hand-rolling request DTOs.
  - Regeneration: after backend OpenAPI changes, run `make generate-api-client` and commit changes to `crates/socio/frontend/src/api/*`.
- App code uses a "code-as-graph" and one-file-per-item approach
  - Prefer small, focused files for hooks, components, and service wrappers.
  - Examples reference: `crates/socio/frontend/src/context/dub-context.tsx`, `crates/socio/frontend/src/hooks/use-dub-route-tracking.ts`.

## One-File-Per-Item Pattern
- Back end
  - One agent tool → one file in `agent_tools/tool_params/` with its parameter struct(s).
  - One tool handler → one file in `agent_tools/handlers/` (keep it small and focused).
  - One HTTP handler (endpoint) → one file in `routes/<domain>/` with `#[utoipa::path]`.
  - One service/integration → one file or tight folder in `services/`.
- Front end
  - One hook/component/utility → one file; typed strictly with backend-generated types.
  - Generated API: do not edit files under `crates/socio/frontend/src/api/*`; instead, wrap in your own thin modules if needed.

## Typical Flow For A New Agent Tool
1. Tool Parameters: add a file in `crates/socio-backend/src/agent_tools/tool_params/<tool_name>_tool_params.rs` with request/response structs and `ToSchema`.
2. Handler: add a file in `crates/socio-backend/src/agent_tools/handlers/handle_<tool_name>_tool.rs` with the tool implementation.
3. Enum: add the new tool to `SocioToolParameters` enum in `crates/socio-backend/src/agent_tools/socio_tool_parameters.rs`.
4. Dispatch: update the dispatch function in `crates/socio-backend/src/agent_tools/dispatch_socio_agent_tool.rs` to handle the new tool.
5. Tests: add backend integration tests under `crates/socio-backend/tests/<domain>/` and frontend tests under `crates/socio/frontend/src/**/*.test.ts` as needed.

## Practical Tips For Agents
- Keep route handlers thin; move IO and business logic to `services/`.
- Use `web::Data` for shared clients (social media, WebSocket) and pass pools explicitly to queries.
- Prefer explicit return types using structs from `types/` or dedicated response DTOs that also derive `ToSchema`.
- Error handling: return consistent `SocioError` from routes; surface typed errors in `services/` and map at the boundary.
- When adding fields to structs used in APIs, remember to:
  - update `ToSchema` annotations (examples, formats),
  - update handlers and tests,
  - regenerate clients and adjust frontend call sites.