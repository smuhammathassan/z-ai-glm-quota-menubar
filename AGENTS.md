# Agent Guidelines

Behavioral guidelines to reduce common LLM coding mistakes. Merge with project-specific instructions as needed.

Tradeoff: These guidelines bias toward caution over speed. For trivial tasks, use judgment.

## 1. Think Before Coding

Do not assume. Do not hide confusion. Surface tradeoffs.

Before implementing:

- State assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them; do not pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what is confusing. Ask.

## 2. Simplicity First

Minimum code that solves the problem. Nothing speculative.

- No features beyond what was asked.
- No abstractions for single-use code.
- No flexibility or configurability that was not requested.
- No error handling for impossible scenarios.
- If 200 lines could be 50, rewrite it.
- Ask: would a senior engineer say this is overcomplicated? If yes, simplify.

## 3. Surgical Changes

Touch only what is required. Clean up only your own mess.

When editing existing code:

- Do not improve adjacent code, comments, or formatting.
- Do not refactor things that are not broken.
- Match existing style, even if you would do it differently.
- If you notice unrelated dead code, mention it; do not delete it.
- Remove imports, variables, and functions that your changes made unused.
- Do not remove pre-existing dead code unless asked.

Every changed line should trace directly to the user request.

## 4. Goal-Driven Execution

Define success criteria. Loop until verified.

- Add validation: write tests for invalid inputs, then make them pass.
- Fix a bug: write a test that reproduces it, then make it pass.
- Refactor: ensure tests pass before and after.

For multi-step tasks, state a brief plan:

1. Step -> verify: check.
2. Step -> verify: check.
3. Step -> verify: check.

## Project Rules

- Keep this app Rust-native and small.
- Do not use Electron, Tauri, SwiftUI, a webview, or background services.
- Store the Z.ai API key only in macOS Keychain.
- Use the unofficial Z.ai quota endpoint cautiously and fail gracefully.
- Mention GLM in public docs for discoverability, but do not add a separate provider unless requested.
- Keep v1 to the requested menu bar text and dropdown actions.
- Target under 10 MB idle RSS; measure before claiming the target is met.
