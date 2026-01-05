To view .rs files with full linting (via rust-analyzer) on secondary Linux monitors
while keeping your main editor focused, you should use a lightweight LSP-capable
terminal pager or editor.

Since most dedicated "viewers" lack full LSP integration for diagnostics,
the best approach is to use a "read-only" instance of a terminal-based editor.

1. Helix (Read-Only Mode)
Helix is a modern terminal editor written in Rust with native LSP support out of the box.
It is perfect for this because it requires zero configuration to show Rust diagnostics.

    The Command: hx --read-only path/to/file.rs

    Why it works: It will immediately connect to rust-analyzer and display inline lints,
    types, and errors, but it prevents accidental edits.

    Setup: On most distros: sudo apt install helix or pacman -S helix.


Automate using a custom task.

Zed Task that interacts with your Linux window manager. While Zed itself doesn't have a
"launch on monitor X" setting, Linux window managers (WMs) allow you to force windows to
specific monitors based on their window title or app ID.

In Pop!_OS COSMIC (the new Rust-based desktop), window management differs from the older
GNOME-based Pop Shell. As of late 2025, COSMIC is designed for native tiling and provides
specific shortcuts to manage windows across multiple monitors.

1. Use "Displays Have Separate Workspaces"
For the most predictable multi-monitor behavior, ensure your workspace settings are
configured to treat each monitor independently.

    Go to Settings > Desktop > Workspaces.
    Ensure Multi-monitor Behavior is set to "Displays Have Separate Workspaces".
    This allows you to launch the terminal on one monitor without affecting the
    layout of your primary monitor where Zed is open.

2. Automated Window Placement (Task Configuration)
COSMIC handles window placement via its compositor, cosmic-comp. While a full "Window Rules"
engine is a highly requested feature in 2025, you can achieve your goal by combining a
Zed Task with COSMIC's keyboard-driven workflow or standard terminal flags.

Update your Zed Task (tasks.json):
Use a terminal that supports the Wayland protocol natively for the best COSMIC experience
(like cosmic-term or foot).

```json
[
  {
    "label": "Open File in Secondary Monitor",
    "command": "cosmic-term --title 'ZedViewer' -e hx --read-only $ZED_FILE",
    "use_new_terminal": true
  }
]
```

3. Moving the Window to Your Target Monitor
If COSMIC launches the window on your primary monitor, use the following native 2025
shortcuts to move it instantly without using your mouse:

    Super + Shift + Arrow Keys: Moves the focused window (the new terminal) to the adjacent monitor.

    Super + X (Window Swapping): If you already have a window open on your secondary monitor,
    press Super + X, use the arrow keys to "hover" over that monitor, and release to swap the
    new terminal into that exact position.

4. "Sticky" Windows for Constant Viewing
If you want your .rs viewer to stay visible even when you switch workspaces on your main monitor:

    Right-click the terminal's title bar (or use the window menu).
    Select "Always on Visible Workspace" (Sticky).

    This ensures that as you code in Zed and move between different virtual desktops,
    your linting/viewing window remains fixed on the second monitor.

Alternative: Use the "Stacking" Feature
If you have limited space on your secondary monitor, COSMIC 2025 supports Window Stacking,
which functions like browser tabs for apps. You can "stack" multiple files you are viewing
by dragging one terminal window on top of another. They will then share the same space on
your second monitor, accessible via tabs at the top.

Hey - let's teleport the file to the second monitor

To achieve a "teleport" effect—where hitting a key spawns the file in a viewer on your
second monitor and simultaneously closes it in Zed—you can use a Zed Keymap that triggers
two actions sequentially.

1. The Zed Logic
Zed allows you to bind multiple actions to a single keyboard shortcut.
You will create a binding that first runs your custom "Viewer" task and then immediately
closes the active tab in Zed.

Update your ~/.config/zed/keymap.json:

```json
[
  {
    "context": "Editor",
    "bindings": {
      "ctrl-alt-v": [
        "task:spawn", { "task_name": "Open File in Secondary Monitor" },
        "pane::CloseActiveItem"
      ]
    }
  }
]
```

    task:spawn: Launches your terminal viewer script.
    pane::CloseActiveItem: The internal Zed command to close the current tab.

2. The Task Configuration
Ensure your ~/.config/zed/tasks.json is set up to handle the terminal properly.
For Pop!_OS COSMIC, using cosmic-term or alacritty is recommended for native Wayland support.

```json
[
  {
    "label": "Open File in Secondary Monitor",
    "command": "cosmic-term --title 'ZedViewer' -e hx --read-only \"$ZED_FILE\"",
    "use_new_terminal": true,
    "allow_concurrent_runs": true
  }
]
```

3. Forcing the "Teleport" to Monitor 2
Since Zed cannot natively tell the terminal where to open, you must use COSMIC’s Window Rules
(available in the 2025 COSMIC Settings) to "catch" any window titled ZedViewer.

    Open Settings > Desktop > Window Management.
    Look for Window Rules (or Auto-Placement).

    Add a new rule:
        Description: Zed Viewer Monitor 2
        Window Title: ZedViewer
        Action: Move to Monitor [Select your 2nd Monitor].
        Focus: You can also set it to not take focus if you want your cursor to stay in Zed.

Summary of Workflow

    Open file in Zed.
    Press Ctrl+Alt+V.
    Zed spawns the terminal (which COSMIC immediately moves to Monitor 2).
    Zed closes the tab on Monitor 1 instantly.
