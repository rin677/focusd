# To-Do List For `v0.2.0`

Updated post release.

- [X] Timer Page
    - [X] Add dashboard
        - [X] Today's focus
        - [X] Sessions completed
        - [X] Current streak
        - [X] Active preset
        - [X] Daily goal progress
    - [X] Make it customizable
        - [X] Allow changing font
        - [X] Allow hiding ASCII art
        - [X] Allow hiding dashboard
        - [X] Allow hiding progress gauze

- [/] Customization options
    - [X] Font
    - [ ] Themes
    - [ ] Sounds
        - [ ] start short break
        - [ ] start long break
        - [ ] start work
    - [X] Hooks
        - [X] Session started
        - [X] Session completed
        - [X] Session paused
        - [X] Session resumed
        - [X] Session skipped
    - [X] Daily goals
        - [X] Configure daily goal
        - [X] Track progress
        - [X] Notify when goal is reached

- [/] Settings Page
    - [X] Home page:
        - [X] Hide/show stats
        - [X] Hide/show ASCII Art
        - [X] Hide/show progress
        - [X] Edit fonts
    - [ ] Edit presets
        - [ ] Create preset
        - [ ] Delete preset
        - [ ] Select active preset
        - [ ] Edit durations
    - [X] Edit notification settings
    - [ ] Edit themes
    - [ ] Add/remove hooks
    - [ ] Set daily goal

- [ ] Play sounds when session ends

- [/] CLI improvements
    - [X] Start work session directly
    - [X] Start short break directly
    - [X] Start long break directly
    - [X] Start preset directly
    - [ ] Start custom duration timer

# Release Notes

## Breaking Changes
- Removed some duplicate commands line arguments (just use `pause` instead of `pause-session`):
    - `pause-session`
    - `resume-session`
    - `reset-session`
    - `stop-session`
    - `skip-session`
    - `next-session`
- Changed `stop-daemon` to `--stop-daemon`

## New Features
- CLI Arguments:
    - Directly start specific mode with arguments `work`, `short-break` and `long-break`
    - Directly select specific preset with `--preset`.
- TUI:
    - New layout which don't have multiple borders.
    - You can hide progress with `tui_show_progress` in config file.
    - You can hide ASCII art with `tui_show_ascii_art` in config file.
    - You can now customize fonts with `font` in config file.
    - Up and down key can scroll in history page.
- Hooks, any shell commands can be executed when timer is paused/resumed and when session starts. It might be useful to send notifications or to toggle music. These are all the hooks available:
    - hook_pause
    - hook_resume
    - hook_resume_short_break
    - hook_pause_short_break
    - hook_resume_work
    - hook_pause_work
    - hook_pause_long_break
    - hook_resume_long_break
    - hook_start_short_break
    - hook_start_long_break
    - hook_start_work
