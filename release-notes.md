# To-Do List For `v0.2.0`

Updated post release.

- [ ] Timer Page
    - [ ] Add dashboard
        - [ ] Today's focus
        - [ ] Sessions completed
        - [ ] Current streak
        - [ ] Active preset
        - [ ] Daily goal progress
    - [ ] Make it customizable
        - [ ] Allow changing font
        - [ ] Allow hiding ASCII art
        - [ ] Allow hiding dashboard
        - [ ] Allow hiding progress gauze

- [ ] Customization options
    - [ ] Font
    - [ ] Themes
    - [ ] Sounds
        - [ ] start short break
        - [ ] start long break
        - [ ] start work
    - [ ] Hooks
        - [ ] Session started
        - [ ] Session completed
        - [ ] Session paused
        - [ ] Session resumed
        - [ ] Session skipped
    - [ ] Daily goals
        - [ ] Configure daily goal
        - [ ] Track progress
        - [ ] Notify when goal is reached

- [ ] Settings Page
    - [ ] Home page
    - [ ] Edit presets
        - [ ] Create preset
        - [ ] Delete preset
        - [ ] Select active preset
        - [ ] Edit durations
    - [ ] Edit notification settings
    - [ ] Edit themes
    - [ ] Edit fonts
    - [ ] Add/remove hooks
    - [ ] Set daily goal

- [ ] Play sounds when session ends

- [/] CLI improvements
    - [X] Start work session directly
    - [X] Start short break directly
    - [X] Start long break directly
    - [ ] Start preset directly
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
