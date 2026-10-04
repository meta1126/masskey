# Graph Report - masskey  (2026-10-03)

## Corpus Check
- 32 files · ~25,413 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 13 file(s) not represented in the graph (top: (none) 3, .rs~ 3, .svelte~ 2)

## Summary
- 316 nodes · 302 edges · 28 communities (17 shown, 11 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- package.json
- compilerOptions
- tauri.conf.json
- devDependencies
- mastodon.rs
- scripts
- Agent Shell Transcript
- default.json
- Agent Shell Transcript
- Agent Shell Transcript
- +layout.ts
- masskey
- Agent Shell Transcript
- END oh-my-opencode-slim clonedeps
- Agent Shell Transcript
- Agent Shell Transcript
- Agent Shell Transcript
- Agent Shell Transcript
- Agent Shell Transcript
- Agent Shell Transcript
- Agent Shell Transcript
- AGENTS.md
- Tauri + SvelteKit + TypeScript

## God Nodes (most connected - your core abstractions)
1. `Agent Shell Transcript` - 14 edges
2. `Agent Shell Transcript` - 14 edges
3. `Agent Shell Transcript` - 14 edges
4. `Agent Shell Transcript` - 12 edges
5. `Agent Shell Transcript` - 11 edges
6. `compilerOptions` - 10 edges
7. `END oh-my-opencode-slim clonedeps` - 9 edges
8. `scripts` - 8 edges
9. `Agent Shell Transcript` - 8 edges
10. `Agent Shell Transcript` - 7 edges

## Surprising Connections (you probably didn't know these)
- `complete_login()` --references--> `Account`  [EXTRACTED]
  src-tauri/src/lib.rs → src-tauri/src/api/mastodon.rs

## Import Cycles
- None detected.

## Communities (28 total, 11 thin omitted)

### Community 0 - "package.json"
Cohesion: 0.08
Nodes (23): dependencies, @tauri-apps/api, @tauri-apps/plugin-opener, @tauri-apps/plugin-shell, description, license, name, type (+15 more)

### Community 1 - "compilerOptions"
Cohesion: 0.15
Nodes (12): ./.svelte-kit/tsconfig.json, compilerOptions, allowJs, checkJs, esModuleInterop, forceConsistentCasingInFileNames, moduleResolution, resolveJsonModule (+4 more)

### Community 2 - "tauri.conf.json"
Cohesion: 0.11
Nodes (17): app, security, windows, build, beforeBuildCommand, beforeDevCommand, devUrl, frontendDist (+9 more)

### Community 3 - "devDependencies"
Cohesion: 0.18
Nodes (11): devDependencies, prettier, prettier-plugin-svelte, svelte, svelte-check, @sveltejs/adapter-static, @sveltejs/kit, @sveltejs/vite-plugin-svelte (+3 more)

### Community 4 - "mastodon.rs"
Cohesion: 0.21
Nodes (6): Account, complete_login(), start_login(), AppState, complete_login(), start_login()

### Community 5 - "scripts"
Cohesion: 0.25
Nodes (8): scripts, build, check, check:watch, dev, prepare, preview, tauri

### Community 6 - "Agent Shell Transcript"
Cohesion: 0.07
Nodes (28): Agent (2026-10-03 11:42:31), Agent (2026-10-03 11:42:42), Agent (2026-10-03 11:43:05), Agent (2026-10-03 11:43:57), Agent's Thoughts (2026-10-03 11:42:26), Agent's Thoughts (2026-10-03 11:42:38), Agent's Thoughts (2026-10-03 11:43:01), Agent's Thoughts (2026-10-03 11:43:15) (+20 more)

### Community 7 - "default.json"
Cohesion: 0.33
Nodes (5): description, identifier, permissions, $schema, windows

### Community 8 - "Agent Shell Transcript"
Cohesion: 0.07
Nodes (28): 1. `api::mastodon::start_login`（`src-tauri/src/api/mastodon.rs:11`）— 本体ロジック, 2. `start_login`（`src-tauri/src/lib.rs:11`）— Tauri コマンドラッパー, Agent (2026-10-03 16:57:57), Agent (2026-10-03 17:00:07), Agent's Thoughts (2026-10-03 16:57:10), Agent's Thoughts (2026-10-03 16:57:27), Agent's Thoughts (2026-10-03 16:59:44), Agent's Thoughts (2026-10-03 16:59:50) (+20 more)

### Community 10 - "Agent Shell Transcript"
Cohesion: 0.08
Nodes (23): Agent (2026-09-27 18:59:51), Agent (2026-09-27 19:00:04), Agent (2026-09-27 19:00:12), Agent (2026-09-27 19:00:18), Agent (2026-09-27 19:00:24), Agent (2026-09-27 19:00:29), Agent (2026-09-27 19:00:58), Agent (2026-09-27 19:20:04) (+15 more)

### Community 16 - "Agent Shell Transcript"
Cohesion: 0.08
Nodes (23): Agent (2026-10-03 16:10:36), Agent (2026-10-03 16:10:59), Agent (2026-10-03 16:12:13), Agent (2026-10-03 16:12:46), Agent's Thoughts (2026-10-03 16:10:28), Agent's Thoughts (2026-10-03 16:10:52), Agent's Thoughts (2026-10-03 16:11:10), Agent's Thoughts (2026-10-03 16:11:51) (+15 more)

### Community 17 - "END oh-my-opencode-slim clonedeps"
Cohesion: 0.09
Nodes (22): Agent (2026-09-27 19:34:55), Agent (2026-09-27 19:35:14), Agent (2026-09-27 19:35:18), Agent (2026-09-27 19:35:25), Agent (2026-09-27 19:37:01), Agent (2026-09-27 19:37:07), Agent (2026-09-27 19:40:50), Cloned Dependency Source (+14 more)

### Community 18 - "Agent Shell Transcript"
Cohesion: 0.10
Nodes (19): Agent (2026-10-03 16:27:20), Agent (2026-10-03 16:27:41), Agent (2026-10-03 16:27:58), Agent (2026-10-03 16:28:30), Agent (2026-10-03 16:28:55), Agent's Thoughts (2026-10-03 16:26:49), Agent's Thoughts (2026-10-03 16:27:28), Agent's Thoughts (2026-10-03 16:27:54) (+11 more)

### Community 19 - "Agent Shell Transcript"
Cohesion: 0.12
Nodes (16): Agent (2026-09-27 19:30:40), Agent (2026-09-27 19:30:45), Agent (2026-09-27 19:30:49), Agent (2026-09-27 19:30:52), Agent (2026-09-27 19:30:55), Agent Shell Transcript, BEGIN oh-my-opencode-slim clonedeps, BEGIN oh-my-opencode-slim clonedeps (+8 more)

### Community 20 - "Agent Shell Transcript"
Cohesion: 0.15
Nodes (12): Agent (2026-09-27 17:30:54), Agent (2026-09-27 17:31:09), Agent (2026-09-27 17:31:25), Agent (2026-09-27 17:33:29), Agent Shell Transcript, Tool Call [completed]: execute, Tool Call [completed]: execute, Tool Call [completed]: execute (+4 more)

### Community 21 - "Agent Shell Transcript"
Cohesion: 0.33
Nodes (5): Agent (2026-09-27 17:48:28), Agent (2026-09-27 17:48:36), Agent Shell Transcript, Tool Call [completed]: read, User (2026-09-27 17:47:47)

### Community 23 - "Agent Shell Transcript"
Cohesion: 0.50
Nodes (3): Agent Shell Transcript, User (2026-10-03 10:46:58), User (2026-10-03 10:50:20)

## Knowledge Gaps
- **192 isolated node(s):** `name`, `version`, `description`, `type`, `dev` (+187 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 220 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **11 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `END oh-my-opencode-slim clonedeps` connect `END oh-my-opencode-slim clonedeps` to `Agent Shell Transcript`?**
  _High betweenness centrality (0.011) - this node is a cross-community bridge._
- **Why does `devDependencies` connect `devDependencies` to `package.json`?**
  _High betweenness centrality (0.008) - this node is a cross-community bridge._
- **What connects `name`, `version`, `description` to the rest of the system?**
  _192 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `package.json` be split into smaller, more focused modules?**
  _Cohesion score 0.07671957671957672 - nodes in this community are weakly interconnected._
- **Should `tauri.conf.json` be split into smaller, more focused modules?**
  _Cohesion score 0.1111111111111111 - nodes in this community are weakly interconnected._
- **Should `Agent Shell Transcript` be split into smaller, more focused modules?**
  _Cohesion score 0.06896551724137931 - nodes in this community are weakly interconnected._
- **Should `Agent Shell Transcript` be split into smaller, more focused modules?**
  _Cohesion score 0.06896551724137931 - nodes in this community are weakly interconnected._