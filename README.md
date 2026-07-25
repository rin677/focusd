# FocusD

A beautiful terminal pomodoro timer with daemon, waybar integration and interactive TUI.

<img width="640" height="656" alt="Image" src="https://github.com/user-attachments/assets/bcdf5f8d-bc9d-4934-ad22-055b4bf09b98" />

## Why Build Another Pomodoro Timer??

I know lots of pomodoro timer exists for terminal and for waybar. But none of them have all those features: waybar integration, nice TUI, history, statistics, streak. I decided to build my one to learn rust.

## Features

- Beautiful TUI.
- Lightning fast (written in rust)
- Long running daemon so it continues in background even if TUI is closed
- Configuration and custom presets
- Full history of all sessions
- Detailed statistics and streak to keep you motivated
- Notification in timer completion
- Synced state between different TUIs.

## Screenshots

### Timer Page

<img width="640" height="656" alt="Image" src="https://github.com/user-attachments/assets/bcdf5f8d-bc9d-4934-ad22-055b4bf09b98" />

### History Page

<img width="882" height="890" alt="Image" src="https://github.com/user-attachments/assets/98a80a17-f7b2-449e-a465-21e8b6612b2b" />

### Stats Page

<img width="1912" height="1156" alt="Image" src="https://github.com/user-attachments/assets/cec26933-ab86-4387-aadf-bab4453df044" />

### Waybar Integration

<img width="640" height="846" alt="Image" src="https://github.com/user-attachments/assets/26c65dbf-3394-4be3-8076-10587b21bab1" />

## Installation

### Mac-Os

```bash
brew tap bibekbhusal0/packages
brew trust bibekbhusal0/packages
brew install focusd
```

### Arch

```bash
yay -S focusd
```

### Windows

Not supported yet.

### Other

First make sure cargo is installed.

```bash
cargo install focusd
```

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

### Shell Aliases

You can setup alias for bash/zsh like this to use it quickly.

```bash
alias pomo='focusd'

```

### Setting Hyprland Keybinds

If you are on hyprland keymap can be setup like this:

```lua
hl.bind("SUPER + P", hl.dsp.exec_cmd("focusd toggle"), {descriptatio = "Pomodoro toggle"})
hl.bind("SUPER + N", hl.dsp.exec_cmd("focusd next"), {descriptatio = "Pomodoro next"})
```

If old `.conf` file

```conf
bindd = SUPER, P, Pomodoro toggle, exec, focusd next
```

### Building Custom Menu with Walker

If you are using walker as launcher and menu, I recommend creating a picker menu to quickly execute different commands instead of setting different keybinds.

```bash
#!/usr/bin/env bash

choice=$(printf '%s\n' \
  ' Start' \
  ' Pause' \
  ' Resume' \
  ' Toggle' \
  ' Reset' \
  '󰒭 Next' \
  ' Stats' \
  ' History' \
  '󰧨 TUI' \
  | walker --dmenu -p "focusd:")

case "$choice" in
  ' Start') focusd start ;;
  ' Pause') focusd pause ;;
  ' Resume') focusd resume ;;
  ' Toggle') focusd toggle ;;
  ' Reset') focusd reset ;;
  '󰒭 Next') focusd next ;;
  ' Stats') xdg-terminal-exec focusd stats ;;
  ' History') xdg-terminal-exec focusd history ;;
  '󰧨 TUI') xdg-terminal-exec focusd ;;
esac
```

## Configuration

The config file is stored at `~/.config/focusd/config.toml`
This is the default config file.

```toml
active_preset = "pomodoro"
show_notifications = true # Show notification when session ends

[presets.pomodoro]
work_minutes = 25
short_break_minutes = 5
long_break_minutes = 15
sessions_before_long_break = 4

[presets.deep_work]
work_minutes = 50
short_break_minutes = 10
long_break_minutes = 30
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

## Roadmap

This project is far from perfect and I will keep improving this. Here are some of the planned features.

- [ ] Include hooks (to be triggered when session ends/starts)
- [ ] Make timer page customizable, maybe show dashboard like interface
- [ ] Including settings page so that settings can be changed interactively
- [ ] Session tags and daily goals

## Known Issues

This is project is recently made and will be under heavy development so there might be some issues/bugs. Those are knows issues:

- If config file don't contain correct data-type in single field, entire config file will not load (falling back to default config)

## Contributing

Feel free to open issues if you encounter any issues or have some feature ideas. But pull requests are not accepted currently. That's because as stated above main goal for building this project is for me to learn rust. But even if you descide to make a Pull request make it to [v-2 branch](https://github.com/BibekBhusal0/focusd/tree/v-2) unless it's a hotfix.

## License

This is licensed under MIT license. Check [License](LICENSE) for more details.
