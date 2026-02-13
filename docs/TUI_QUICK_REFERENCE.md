# TUI Quick Reference

## Launch TUI

```bash
clicky tui
```

## Global Shortcuts

| Key | Action |
|-----|--------|
| `q` | Quit (from Board view) |
| `?` | Toggle help overlay |

## Board View

| Key | Action |
|-----|--------|
| `←/h` | Previous column |
| `→/l` | Next column |
| `↑/k` | Previous card |
| `↓/j` | Next card |
| `Enter` | Select card (view details) |
| `c` | Create new card |
| `Esc` | Exit card selection |

## Create Card Form

| Key | Action |
|-----|--------|
| `↑/k` | Previous field |
| `↓/j` | Next field |
| `i` | Enter edit mode |
| `Enter` | Save field / Submit card |
| `Esc` | Cancel / Return to board |

**Fields:**
1. Title (required)
2. Description (optional)
3. Assignee (optional)
4. Column (read-only, defaults to current)

## Card Detail View

| Key | Action |
|-----|--------|
| `e` | Edit card (future) |
| `d` | Delete card |
| `m` | Move card (future) |
| `Esc/q` | Return to board |

## Delete Confirmation

| Key | Action |
|-----|--------|
| `y` | Confirm deletion |
| `n` | Cancel / Return to card details |

## Tips

1. Create cards in the column you want them to appear (column is read-only during creation)
2. Press `i` before typing in a field
3. Press `Enter` to save each field and move to the next
4. The final `Enter` submits the card
5. Use `Esc` at any time to cancel and return to the board
6. Title is the only required field

## Workflow Example

**Create a card:**
```
c → i → Type title → Enter → i → Type description → Enter → i → Type assignee → Enter → Enter
```

**Delete a card:**
```
↓ → Select card → Enter → d → y
```
