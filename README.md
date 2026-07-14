<!-- :TODO: other segments later -->

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

## Waybar Configuration

Add a custom module to your Waybar configuration:

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
    "short-break": "", // other options 🍪, 🥤, 
    "short-break-paused": "󰏤",
    "long-break": "󰒲", // Other options 🏖️,⏳,🌴
    "long-break-paused": "󰏤"
  },

  "on-click": "focusd toggle",
  "on-click-right": "focusd next",
  "on-click-middle": "focusd reset",
}
```

or if you want icons based on percentage:

```jsonc
"format-icons": [ "󰪞", "󰪟", "󰪠", "󰪡", "󰪢" ], // Make sure to give different colors to recognize session type/state
```

Then add the module to your bar:

```jsonc
"modules-right": [
  "custom/pomodoro"
]
```

## Available States

The following values may be emitted in the `alt` field (can be used to show icons):

| State                |
| -------------------- |
| `work`               |
| `work-paused`        |
| `short-break`        |
| `short-break-paused` |
| `long-break`         |
| `long-break-paused`  |

## Styling

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

Depending on the current state, one or more classes may be applied to the module.
