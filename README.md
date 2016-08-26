# i3-workspace-scroll

This is a small wrapper around the i3 commands 'workspace prev_on_output' and
'workspace next_on_output' that I created to make better use of the horizontal
scroll wheel on my mouse. It does the same as those i3 commands, except when we
tell it to go to the next workspace of the last one on the focused output, or
the previous workspace of the first one. In those cases, it simply does
nothing, instead of wrapping around like the i3 commands do.

`.xbindkeysrc` configuration for Logitech MX Master:

```
# thumb wheel up => prev workspace
"i3-workspace-scroll prev"
    b:6

# thumb wheel down => next workspace
"i3-workspace-scroll next"
    b:7
```

You can also configure the normal scroll wheel of any mouse to scroll through
your workspaces when some other key is pressed, e.g. the Super ("Windows") key:

```
# Super+ScrUp => prev workspace
"i3-workspace-scroll prev"
  Mod4 + b:4

# Super+ScrDown => next workspace
"i3-workspace-scroll next"
  Mod4 + b:5
```
