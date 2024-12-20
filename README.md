✨ Overview
---

`shfl` (short for "shuffle") lets you easily rearrange lines in a file with
simple keymaps.

![demo](https://github.com/user-attachments/assets/07bd4b71-f78b-4b82-8080-3b973258bf55)

🤔 Motivation
---

I like switching my tmux sessions quickly (using `tmux switch-client -t
"$session"`, triggered via a shortcut). The session names are stored in a local
file, and I map a key to a session on a specific line number. To easily change
which session is assigned to which line number, I needed a tool that would start
up quickly and have easy keymaps to reorder lines in a file.

💾 Installation
---

**cargo**:

```sh
cargo install --git https://github.com/dhth/shfl.git
```

⌨️ Keymaps
---

```
K                    move item one position above
Enter                move item/selection to the start of the list
j / Down             go down
k / Up               go up
[1-9]                move current item to index in list
g                    go to the start of the list
G                    go to the end of the list
w                    write to file
space / s            select/unselect item
?                    show/hide help view
Esc / q              go back/reset selection/exit
```

Acknowledgements
---

`shfl` is built using [ratatui][1].

[1]: https://github.com/ratatui/ratatui
