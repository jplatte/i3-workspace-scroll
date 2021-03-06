# i3-workspace-scroll

This is a small wrapper around the i3 / sway commands 'workspace prev_on_output'
and 'workspace next_on_output' that I created to make better use of the
horizontal scroll wheel on my mouse. It does the same as those i3 / sway
commands, except when the next next workspace is requested on the last one on
the focused output, or the previous workspace is requested on the first one. In
those cases, it simply does nothing, instead of wrapping around like the i3 /
sway commands do.

## Installation

```
cargo install i3-workspace-scroll
```

## Usage

Note: You might want to swap button6 and button7 depending on your mouse.

### sway

Note: In sway, it won't work when the cursor is on top of a non-window surface (e.g. the bar).

**Horizontal scroll wheel:**

```
bindsym --whole-window --border --input-device=<device> +button6 exec ~/.cargo/bin/i3-workspace-scroll prev
bindsym --whole-window --border --input-device=<device> +button7 exec ~/.cargo/bin/i3-workspace-scroll next
```

**Regular scroll wheel + modifier:**

```
bindsym --whole-window --border --input-device=type:pointer $mod+button4 exec ~/.cargo/bin/i3-workspace-scroll prev
bindsym --whole-window --border --input-device=type:pointer $mod+button5 exec ~/.cargo/bin/i3-workspace-scroll next
```

### i3

Note: The code blocks below are configuration for `xbindkeys`, which needs to be started with i3 for
this to work. The configuration has to be placed in `~/.xbindkeysrc`.

**Horizontal scroll wheel:**

```
# thumb wheel up => prev workspace
"~/.cargo/bin/i3-workspace-scroll prev"
    b:6

# thumb wheel down => next workspace
"~/.cargo/bin/i3-workspace-scroll next"
    b:7
```

**Regular scroll wheel + modifier:**

```
# Super+ScrUp => prev workspace
"~/.cargo/bin/i3-workspace-scroll prev"
  Mod4 + b:4

# Super+ScrDown => next workspace
"~/.cargo/bin/i3-workspace-scroll next"
  Mod4 + b:5
```

## License

This application is licensed under the GNU General Public License v3.0 or any
later version. The full license text can be found in the file <LICENSE> next to
this README.

