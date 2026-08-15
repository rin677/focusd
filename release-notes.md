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
  - Dashboard style layout for timer page.
  - Fonts and theme can now be customized.
  - Up and down key can scroll in history page.
  - Popup to show all keymaps you can see it by pressing `?`.
  - Settings page so that you don't have to open the toml file.
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
- Omarchy integration:
  - Focusd now has omarchy shell plugin. Check the [omarchy-focusd](https://github.com/BibekBhusal0/omarchy-focusd) for more info.

## Screenshots

![Timer page](https://raw.githubusercontent.com/BibekBhusal0/focusd/master/media/timer-demo.gif)

![History page](https://raw.githubusercontent.com/BibekBhusal0/focusd/master/media/history-page.png)

![Stats page](https://raw.githubusercontent.com/BibekBhusal0/focusd/master/media/stats-page.png)

![Waybar integration](https://raw.githubusercontent.com/BibekBhusal0/focusd/master/media/waybar-demo.gif)

![Omarchy plugin](https://raw.githubusercontent.com/BibekBhusal0/focusd/master/media/omarchy-plugin.gif)
