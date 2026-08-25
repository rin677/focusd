## New Features
- You can add minutes to the running timer, its keymaps is `a`, or use CLI argument `--add-minutes` or `-a`

## Fix
- Hooks works properly when starting session.

## Breaking Changes
- 4 commands have been removed into one `work`, `long-break`, `short-break` and `start` are now single command
  - `focusd --start` start current session
  - `focusd --start work` starts work session
  - `focusd --start short-break` starts short break
  - `focusd --start long-break` starts long break
