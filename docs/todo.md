# Brainstormed TODOs

- Pane management: toggle single/split view; explore dynamic pane count (up to four) with intuitive layout controls.
- File handling: keep .txt/.md support; remember last open directory; add recent-file history; auto-reload when a file changes on disk.
- Editor polish: basic Markdown syntax highlighting; optional word wrap; configurable font and size; toggle whitespace visibility.
- Split layouts: support 3-4 pane arrangements and add Ctrl+Tab focus cycling so documents can be switched without touching the mouse.
- Keyboard shortcuts: ensure common navigation works (Home/End, Shift+arrows selection, copy/paste, etc.) and expose them via the palette docs.
- Saving workflow: autosave interval; confirm-on-overwrite; surface quick-save location/history; warn before closing dirty panes.
- UX niceties: keyboard shortcuts for pane switching/toggling; status indicators for active pane/file type; per-pane rename option.
- Future integrations: clipboard history; export to PDF/HTML for Markdown; optional spell-check or linting plug-ins.
- Next session: finish wiring more commands into the palette (manual save, layout variants) and chase down a full `cargo +nightly check` run once deps are cached.
