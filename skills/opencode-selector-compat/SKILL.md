---
name: opencode-selector-compat
description: Validate and maintain compatibility when opencode's database schema changes.
---

# opencode-selector-compat

Use this skill when opencode updates may have changed the SQLite schema, or
when making the selector more resilient to schema drift.

## Trigger

- A query fails because a column is missing.
- User reports `opencode-selector` broke after an opencode update.
- Adding new fields from opencode's DB.

## Workflow

1. **Inspect the schema**
   - Run `sqlite3 ~/.local/share/opencode/opencode.db ".schema"` or use Python.
   - Compare against the queries in `src/db/repository.rs`.

2. **Make columns optional**
   - Treat missing columns as `Option<T>`.
   - Provide sensible defaults.

3. **Test gracefully**
   - Add in-memory DB tests with and without the new column.
   - Ensure the app starts even when columns are missing.

4. **Document**
   - Update `AGENTS.md` if compatibility rules change.

## Constraints

- Never modify opencode's DB schema.
- Never assume columns exist without fallback.
- Keep read-only operations safe by default.
