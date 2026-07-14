# FocusD

A beautiful terminal pomodoro timer with daemon, waybar integration and interactive TUI.

<!-- TODO: one screenshot here -->

## Why Build Another Pomodoro Timer??

I know lots of pomodoro timer exists for terminal and for waybar. But I wanted to build my own just to learn rust. And this project has been made while learning rust.

## Features

- Beautiful TUI.
- Lightning fast (written in rust)
- Long running daemon so it continues in background even if TUI is closed
- Configuration and custom presets
- Full history of all sessions
- Detailed statistics and streak to keep you motivated
- Notification in timer completion
- Synced state between different TUIs.

<!-- TODO: More screenshots and GIFs here -->

## Installation

<!-- TODO: Will try to release this in cargo, homebrew and aur -->

### Building from Source

```bash
git clone https://github.com/bibekbhusal0/focusd
cd focusd
cargo build --release
```

## Usage

After focusd is installed it can be launched with command `focusd`, which opens the TUI showing timer page. Pressing `Space` will start the timer.

### Commands

Those commands can also be seen by running `focusd --help`

- `stats` - Launch TUI in stats page.
- `history` - Launch TUI in history page.
- `pause` - Pause the timer if running.
- `toggle` - Toggle the timer.
- `reset` - Reset current session.
- `next`/`skip` - Skip to next session.
- `start` - Start the session.
- `prints-stats` - Print stats.
- `print-history` - Print all the history.
- `status` - Shows timer status in JSON format, this is mainly for waybar (see the section for [waybar](#waybar-configuration))
- `stop-daemon` - Stop daemon if running.
- `--daemon` - Start daemon (force stops currently running daemon).

### Keymap

For TUI those are the keymaps:

- `space` toggle pomodoro timer.
- `n` skip to next session.
- `r` reset current session.
- `q` to exit the TUI.
- `[` to go to next page.
- `]` to go to previous page.
- `j`/`k` to scroll in history page.

## Configuration

The config file is stored at `~/.config/focusd/config.toml`
This is the default config file.

```toml
active_preset = "pomodoro"
show_notifications = true # Show notification when session ends

[presets.pomodoro]
work_minutes = 25
short_break_minutes = 5
long_breka_minutes = 15
sessions_before_long_break = 4

[presets.deep_work]
work_minutes = 50
short_break_minutes = 10
long_breka_minutes = 30
sessions_before_long_break = 4

# ... More presets can be defined here
```

### Waybar Configuration

The command `focusd status` gives output in JSON format which can be used for Waybar.

```json
{
  "text": "37:18",
  "alt": "work",
  "class": ["work"],
  "percentage": 25,
  "tooltip": "Work Session\nCycle 2/4\n\nNext: Short Break (10m)\n\nCompleted Today: 4\nFocused Today: 1h 38m\nCurrent Streak: 16 days"
}
```

Add a custom module in your Waybar:
change `~/.config/waybar/config.jsonc` to include pomodoro module.

```jsonc
"custom/pomodoro": {
  "return-type": "json",
  "exec": "focusd status",
  "interval": 1,
  "format": "{icon} {text}",
  "tooltip":true,
  "format-icons": {
    "work": "", // Other options , 🍅
    "work-paused": "󰏤", //  other options 󰏤 ,,, , or same as above and change opacity from styles.css
    "short-break": "", // other options ☕, 🍪, 🥤, 
    "short-break-paused": "󰏤",
    "long-break": "󰒲", // Other options 🏖️,⏳,🌴
    "long-break-paused": "󰏤"
  },

  "on-click": "focusd toggle",
  "on-click-right": "focusd next",
  "on-click-middle": "focusd reset",
},

...

"modules-right": [
  "custom/pomodoro"
]
```

or if you want icons based on progress:

```jsonc
"format-icons": [ "󰪞", "󰪟", "󰪠", "󰪡", "󰪢" ], // Make sure to give different colors to recognize session type/state
```

#### Styling

The module exposes state through CSS classes.

Example:

```css
#custom-pomodoro.work {
  color: #a6e3a1;
}

#custom-pomodoro.short-break {
  color: #89b4fa;
}

#custom-pomodoro.long-break {
  color: #cba6f7;
}

#custom-pomodoro.paused {
  opacity: 0.6;
}
```

## Road Map

This project is far from perfect and I will keep improving this. Here are some of the planned features.

- [ ] Include hooks (to be triggered when session ends/starts)
- [ ] Make timer page customizable, maybe show dashboard like interface
- [ ] Including settings page so that settings can be changed interactively
- [ ] Session tags and daily goals

## Known Issues

This is project is recently made and will be under heavy development so there might be some issues/bugs. Those are knows issues:

- If config file don't contain correct data-type in single field, entire config file will not load (falling back to default config)

## Contributing

Feel free to open issues if you encounter any issues or have some feature ideas. But pull requests are not accepted currently. That's because as stated above main goal for building this project is for me to learn rust.

## License

This is licensed under MIT license. Check [License](LICENSE) for more details.
