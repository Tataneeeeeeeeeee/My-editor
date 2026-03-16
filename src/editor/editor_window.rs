use super::key;
use super::menu_bar;
use super::menu_bar::menu_bar::{MenuAction, MenuBar};
use super::tab_bar;
use super::text_buffer::TextBuffer;
use super::tool_bar::search_file::{render_search_files, search_in_files};
use super::tool_bar::text_input::{TextInputState, TextInputType};
use super::tool_bar::tree_file::{FileTree, render_file_tree};
use crate::settings::settings::{SettingsGlobal, load_settings};
use gpui::prelude::FluentBuilder;
use gpui::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Represents an editor tab with its content
pub struct EditorTab {
    pub buffer: TextBuffer,
    pub title: String,
    pub focus_handle: FocusHandle,
    pub is_modified: bool,
}

impl EditorTab {
    pub fn new(_id: usize, title: String, focus_handle: FocusHandle) -> Self {
        Self {
            buffer: TextBuffer::new(),
            title,
            focus_handle,
            is_modified: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PendingCreationKind {
    File,
    Folder,
}

#[derive(Clone, Debug)]
pub struct PendingCreation {
    pub kind: PendingCreationKind,
    pub parent_dir: PathBuf,
    pub input: String,
}

#[allow(dead_code)]
/// Main window containing multiple editor tabs
pub struct EditorWindow {
    pub tabs: Vec<EditorTab>,
    pub active_tab_index: usize,
    pub next_tab_id: usize,
    pub menu_bar: MenuBar,
    pub file_tree: FileTree,
    pub root_dir: PathBuf,
    pub explorer_open: bool,
    pub search_open: bool,
    pub search_input_state: TextInputState,
    pub menu_icon: std::collections::HashMap<String, Arc<Path>>,
    pub pending_creation: Option<PendingCreation>,

    /// Keeps the window-activation subscription alive for the lifetime of the view.
    _activation_subscription: Option<Subscription>,
}

impl EditorWindow {
    pub fn new(_id: usize, title: String, root_dir: PathBuf, cx: &mut Context<Self>) -> Self {
        let first_focus = cx.focus_handle();
        let first_tab = EditorTab::new(0, title, first_focus);

        let settings_global = cx.global::<SettingsGlobal>();
        let assets_dir = PathBuf::from(
            settings_global
                .get(vec!["assets", "path"])
                .expect("Failed to get assets path setting"),
        );
        let icon = |name: &str| -> Arc<Path> { Arc::from(assets_dir.join(name).as_path()) };

        let mut menu_icon = std::collections::HashMap::new();
        menu_icon.insert("explorer".to_string(), icon("explorer.png"));
        menu_icon.insert("search".to_string(), icon("search.png"));

        Self {
            tabs: vec![first_tab],
            active_tab_index: 0,
            next_tab_id: 1,
            menu_bar: MenuBar::new(),
            file_tree: FileTree::new(root_dir.clone(), cx),
            root_dir,
            explorer_open: false,
            search_open: false,
            search_input_state: TextInputState::new(TextInputType::Search),
            menu_icon,
            pending_creation: None,
            _activation_subscription: None,
        }
    }

    /// Adds a new tab
    pub fn add_tab(&mut self, title: String, cx: &mut Context<Self>) {
        let new_focus = cx.focus_handle();
        let new_tab = EditorTab::new(self.next_tab_id, title, new_focus);

        self.next_tab_id += 1;
        self.tabs.push(new_tab);
        self.active_tab_index = self.tabs.len() - 1;
        cx.notify();
    }

    /// Closes a tab by index
    pub fn close_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.tabs.len() > 1 && index < self.tabs.len() {
            self.tabs.remove(index);
            if self.active_tab_index >= self.tabs.len() {
                self.active_tab_index = self.tabs.len() - 1;
            }
            cx.notify();
        }
    }

    /// Changes the active tab
    pub fn set_active_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.tabs.len() {
            self.active_tab_index = index;
            cx.notify();
        }
    }

    /// Handles menu actions
    pub fn handle_menu_action(&mut self, action: MenuAction, cx: &mut Context<Self>) {
        self.menu_bar.file_menu_open = false;

        match action {
            MenuAction::NewFile => self.add_tab("Untitled".to_string(), cx),
            MenuAction::OpenFile => self.open_file(cx),
            MenuAction::SaveFile => self.save_current_file(cx),
            MenuAction::OpenSettings => self.open_settings(cx),
        }
    }

    /// Opens a file with a file selection dialog
    pub fn open_file(&mut self, cx: &mut Context<Self>) {
        if let Some(file_path) = rfd::FileDialog::new()
            .add_filter("Text Files", &["txt", "rs", "toml", "json", "md"])
            .add_filter("All Files", &["*"])
            .pick_file()
        {
            match std::fs::read_to_string(&file_path) {
                Ok(content) => {
                    let file_name = file_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Untitled")
                        .to_string();

                    let new_focus = cx.focus_handle();
                    let mut new_tab = EditorTab::new(self.next_tab_id, file_name, new_focus);
                    new_tab.buffer.load_from_file(file_path, content);

                    self.next_tab_id += 1;
                    self.tabs.push(new_tab);
                    self.active_tab_index = self.tabs.len() - 1;

                    cx.notify();
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                }
            }
        }
    }

    /// Saves the active file (public alias used by key shortcuts)
    pub fn save_current_file(&mut self, cx: &mut Context<Self>) {
        self.save_file(cx);
    }

    /// Saves the active file
    fn save_file(&mut self, cx: &mut Context<Self>) {
        let active_tab = &mut self.tabs[self.active_tab_index];

        if let Some(file_path) = &active_tab.buffer.file_path {
            match std::fs::write(file_path, &active_tab.buffer.text) {
                Ok(_) => {
                    println!("File saved: {:?}", file_path);
                }
                Err(e) => {
                    eprintln!("Error saving file: {}", e);
                }
            }
        } else {
            self.save_file_as(cx);
        }
    }

    /// Saves the file with a "Save As" dialog
    fn save_file_as(&mut self, cx: &mut Context<Self>) {
        let active_tab = &mut self.tabs[self.active_tab_index];

        if let Some(file_path) = rfd::FileDialog::new()
            .add_filter("Text Files", &["txt"])
            .add_filter("Rust Files", &["rs"])
            .add_filter("All Files", &["*"])
            .set_file_name(&active_tab.title)
            .save_file()
        {
            match std::fs::write(&file_path, &active_tab.buffer.text) {
                Ok(_) => {
                    let file_name = file_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Untitled")
                        .to_string();

                    active_tab.buffer.file_path = Some(file_path.clone());
                    active_tab.title = file_name;

                    println!("File saved as: {:?}", file_path);
                    cx.notify();
                }
                Err(e) => {
                    eprintln!("Error saving file: {}", e);
                }
            }
        }
    }

    /// Open Settings file in the editor
    fn open_settings(&mut self, cx: &mut Context<Self>) {
        match std::env::var("HOME")
            .map(|home| PathBuf::from(home).join(".my-editor").join("settings.json"))
        {
            Ok(settings_path) => {
                if settings_path.exists() {
                    self.open_file_from_path(settings_path, cx);
                } else {
                    eprintln!(
                        "Settings file not found at expected location: {:?}",
                        settings_path
                    );
                }
            }
            Err(e) => eprintln!("Error determining home directory: {}", e),
        }
    }

    /// Toggle the file explorer panel
    pub fn toggle_explorer(&mut self, cx: &mut Context<Self>) {
        self.explorer_open = !self.explorer_open;
        self.search_open = false; // Ensure search is closed when opening explorer
        cx.notify();
    }

    /// Toggle the file search panel
    pub fn toggle_search(&mut self, cx: &mut Context<Self>) {
        self.search_open = !self.search_open;
        self.explorer_open = false; // Ensure explorer is closed when opening search
        cx.notify();
    }

    /// Append a character to the search input
    pub fn search_input_push(&mut self, ch: char, cx: &mut Context<Self>) {
        self.search_input_state.push_char(ch);
        cx.notify();
    }

    /// Backspace on the search input
    pub fn search_input_backspace(&mut self, cx: &mut Context<Self>) {
        self.search_input_state.backspace();
        cx.notify();
    }

    /// Clear the search input
    pub fn search_input_clear(&mut self, cx: &mut Context<Self>) {
        self.search_input_state.clear();
        cx.notify();
    }

    /// Open a file directly from a PathBuf (called from the file tree)
    pub fn open_file_from_path(&mut self, file_path: PathBuf, cx: &mut Context<Self>) {
        match std::fs::read_to_string(&file_path) {
            Ok(content) => {
                let file_name = file_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Untitled")
                    .to_string();

                // If file already open, just switch to it
                if let Some(idx) = self
                    .tabs
                    .iter()
                    .position(|t| t.buffer.file_path.as_deref() == Some(&file_path))
                {
                    self.active_tab_index = idx;
                    cx.notify();
                    return;
                }

                let new_focus = cx.focus_handle();
                let mut new_tab = EditorTab::new(self.next_tab_id, file_name, new_focus);
                new_tab.buffer.load_from_file(file_path, content);

                self.next_tab_id += 1;
                self.tabs.push(new_tab);
                self.active_tab_index = self.tabs.len() - 1;
                cx.notify();
            }
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    }

    /// Start creating a new file inside the currently selected/root dir
    pub fn start_create_file(&mut self, cx: &mut Context<Self>) {
        let parent_dir = self.get_creation_parent_dir();
        self.pending_creation = Some(PendingCreation {
            kind: PendingCreationKind::File,
            parent_dir,
            input: String::new(),
        });
        cx.notify();
    }

    /// Start creating a new folder inside the currently selected/root dir
    pub fn start_create_folder(&mut self, cx: &mut Context<Self>) {
        let parent_dir = self.get_creation_parent_dir();
        self.pending_creation = Some(PendingCreation {
            kind: PendingCreationKind::Folder,
            parent_dir,
            input: String::new(),
        });
        cx.notify();
    }

    /// Returns the directory where the new item should be created.
    /// Uses last_selected if it's a dir, otherwise its parent, fallback to root_path.
    fn get_creation_parent_dir(&self) -> PathBuf {
        let last = &self.file_tree.last_selected;
        if last.is_dir() {
            last.clone()
        } else if let Some(parent) = last.parent() {
            parent.to_path_buf()
        } else {
            self.file_tree.root_path.clone()
        }
    }

    /// Append a character to the pending creation input
    pub fn creation_input_push(&mut self, ch: char, cx: &mut Context<Self>) {
        if let Some(ref mut pc) = self.pending_creation {
            pc.input.push(ch);
            cx.notify();
        }
    }

    /// Backspace on the pending creation input
    pub fn creation_input_backspace(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut pc) = self.pending_creation {
            pc.input.pop();
            cx.notify();
        }
    }

    /// Cancel pending creation
    pub fn cancel_creation(&mut self, cx: &mut Context<Self>) {
        self.pending_creation = None;
        cx.notify();
    }

    /// Confirm pending creation: create the file or folder on disk, refresh tree
    pub fn confirm_creation(&mut self, cx: &mut Context<Self>) {
        let Some(pc) = self.pending_creation.take() else {
            return;
        };
        let name = pc.input.trim().to_string();
        if name.is_empty() {
            cx.notify();
            return;
        }
        let target = pc.parent_dir.join(&name);
        match pc.kind {
            PendingCreationKind::File => {
                if let Err(e) = std::fs::write(&target, "") {
                    eprintln!("Error creating file: {}", e);
                } else {
                    self.file_tree.refresh();
                    self.open_file_from_path(target, cx);
                }
            }
            PendingCreationKind::Folder => {
                if let Err(e) = std::fs::create_dir_all(&target) {
                    eprintln!("Error creating folder: {}", e);
                } else {
                    self.file_tree.refresh();
                }
            }
        }
        cx.notify();
    }
}

impl Render for EditorWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Register the window-activation observer once (requires &mut Window, only available here)
        if self._activation_subscription.is_none() {
            self._activation_subscription =
                Some(cx.observe_window_activation(_window, |this, _window, cx| {
                    if _window.is_window_active() {
                        this.file_tree.refresh();

                        let settings_global = load_settings().expect("Failed to load settings");
                        cx.set_global(settings_global);

                        cx.notify();
                    }
                }));
        }

        use super::editor_element::EditorElement;

        let on_key = cx.listener(|this, event: &KeyDownEvent, window, cx| {
            key::key::handle_key(this, event, window, cx);
        });

        let on_mouse_down = cx.listener(|this, event: &MouseDownEvent, _window, cx| {
            let settings_global = cx.global::<SettingsGlobal>().clone();

            let line_height = settings_global
                .get_f32(vec!["ui", "editor", "line_height_px"])
                .unwrap_or(19.2);
            let gutter_width = settings_global
                .get_f32(vec!["ui", "panels", "explorer", "width_px"])
                .unwrap_or(50.0);
            let gutter_margin = settings_global
                .get_f32(vec!["ui", "editor", "gutter", "margin_px"])
                .unwrap_or(8.0);
            let padding_left = settings_global
                .get_f32(vec!["ui", "editor", "padding_left_px"])
                .unwrap_or(16.0);
            let total_offset = gutter_width + gutter_margin + padding_left;
            let monospace_char_width = settings_global
                .get_f32(vec!["ui", "editor", "monospace_char_width_px"])
                .unwrap_or(8.0);
            let tab_bar_height = settings_global
                .get_f32(vec!["ui", "panels", "tab_bar", "height_px"])
                .unwrap_or(40.0);
            let menu_bar_height = settings_global
                .get_f32(vec!["ui", "panels", "menu_bar", "height_px"])
                .unwrap_or(30.0);
            let scrollbar_width = settings_global
                .get_f32(vec!["ui", "editor", "scrollbar", "width_px"])
                .unwrap_or(14.0);
            let explorer_width = settings_global
                .get_f32(vec!["ui", "panels", "explorer", "width_px"])
                .unwrap_or(240.0);
            let status_bar_height = settings_global
                .get_f32(vec!["ui", "panels", "status_bar", "height_px"])
                .unwrap_or(60.0);

            let x: f32 = event.position.x.into();
            let y: f32 = event.position.y.into();

            if y < (menu_bar_height + tab_bar_height) {
                return;
            }

            // Ignore clicks inside the explorer panel
            if this.explorer_open && x < explorer_width {
                return;
            }

            let editor_x = if this.explorer_open {
                x - explorer_width
            } else {
                x
            };

            let window_width = 800.0;
            let active_tab = &mut this.tabs[this.active_tab_index];
            let buffer = &mut active_tab.buffer;

            if editor_x >= window_width - scrollbar_width {
                let viewport_height = 600.0 - tab_bar_height - status_bar_height;
                let scrollbar_y = y - menu_bar_height - tab_bar_height;
                let content_height = buffer.line_count as f32 * line_height;
                let max_scroll = (content_height - viewport_height).max(0.0);
                let scroll_ratio = scrollbar_y / viewport_height;
                buffer.scroll_y = (scroll_ratio * max_scroll).max(0.0).min(max_scroll);
                cx.notify();
                return;
            }

            let top_padding: f32 = padding_left + tab_bar_height + padding_left;
            let adjusted_y = y + buffer.scroll_y - top_padding;
            let line_index = (adjusted_y / line_height).max(1.0) as usize;
            let approximate_col =
                ((editor_x - total_offset) / monospace_char_width).round() as usize;
            buffer.set_cursor_from_position(line_index, approximate_col);
            cx.notify();
        });

        let on_scroll = cx.listener(|this, event: &ScrollWheelEvent, _window, cx| {
            let settings_global = cx.global::<SettingsGlobal>().clone();
            let line_height = settings_global
                .get_f32(vec!["ui", "editor", "line_height_px"])
                .unwrap_or(19.2);

            let active_tab = &mut this.tabs[this.active_tab_index];
            let buffer = &mut active_tab.buffer;
            let delta_y: f32 = event.delta.pixel_delta(px(1.0)).y.into();
            buffer.scroll_y = (buffer.scroll_y - delta_y * 2.5).max(0.0);
            let max_scroll = ((buffer.line_count as f32 * line_height) - 600.0).max(0.0);
            buffer.scroll_y = buffer.scroll_y.min(max_scroll);
            cx.notify();
        });

        let on_new_file = cx.listener(|this, _: &MouseDownEvent, _window, cx| {
            this.handle_menu_action(MenuAction::NewFile, cx);
        });
        let on_open_file = cx.listener(|this, _: &MouseDownEvent, _window, cx| {
            this.handle_menu_action(MenuAction::OpenFile, cx);
        });
        let on_save_file = cx.listener(|this, _: &MouseDownEvent, _window, cx| {
            this.handle_menu_action(MenuAction::SaveFile, cx);
        });

        let on_open_settings = cx.listener(|this, _: &MouseDownEvent, _window, cx| {
            this.handle_menu_action(MenuAction::OpenSettings, cx);
            this.explorer_open = false;
        });

        let on_toggle_explorer = cx.listener(|this, _: &MouseDownEvent, _window, cx| {
            this.toggle_explorer(cx);
        });

        let on_toggle_search = cx.listener(|this, _: &MouseDownEvent, _window, cx| {
            this.toggle_search(cx);
        });

        let settings_global = cx.global::<SettingsGlobal>().clone();
        let file_dropdown = menu_bar::bar_element::render_file_dropdown(
            on_new_file,
            on_open_file,
            on_save_file,
            settings_global.clone(),
        );
        let settings_dropdown = menu_bar::bar_element::render_setting_dropdown(
            on_open_settings,
            settings_global.clone(),
        );

        let focus_handle = self.tabs[self.active_tab_index].focus_handle.clone();

        let tabs_info: Vec<(usize, String, bool, bool)> = self
            .tabs
            .iter()
            .enumerate()
            .map(|(i, tab)| {
                (
                    i,
                    tab.title.clone(),
                    i == self.active_tab_index,
                    tab.is_modified,
                )
            })
            .collect();

        let active_tab = &mut self.tabs[self.active_tab_index];
        let buffer = &mut active_tab.buffer;

        let viewport_height: f32 = _window.viewport_size().height.into();

        let editor_element = EditorElement::new(
            buffer.text.clone(),
            buffer.cursor,
            buffer.line_count,
            buffer.current_line,
            buffer.current_col,
            buffer.scroll_y,
            viewport_height - 60.0,
            settings_global,
        )
        .with_file_extension(buffer.get_file_extension());

        let tabs_bar = tab_bar::bar_element::render_bar(&tabs_info, cx);
        let menu_bar_element = self.menu_bar.render(self.menu_bar.file_menu_open, cx);
        let file_tree_element =
            render_file_tree(&self.file_tree, self.pending_creation.as_ref(), cx);
        let search_results = search_in_files(&self.search_input_state.input, &self.root_dir);
        let search_element = render_search_files(&self.search_input_state, &search_results, cx);

        let settings_global_colors = cx.global::<SettingsGlobal>().clone();
        let bg_color = settings_global_colors
            .get_color(vec!["ui", "colors", "background"])
            .unwrap_or(0x1e1e1e);
        let toolbar_bg = settings_global_colors
            .get_color(vec!["ui", "panels", "toolbar", "background"])
            .unwrap_or(0x333333);
        let toolbar_active_bg = settings_global_colors
            .get_color(vec!["ui", "panels", "toolbar", "button_active_background"])
            .unwrap_or(0x505050);
        let toolbar_hover_bg = settings_global_colors
            .get_color(vec!["ui", "panels", "toolbar", "button_hover_background"])
            .unwrap_or(0x454545);
        let toolbar_text = settings_global_colors
            .get_color(vec!["ui", "panels", "toolbar", "button_text"])
            .unwrap_or(0x858585);
        let toolbar_active_text = settings_global_colors
            .get_color(vec!["ui", "panels", "toolbar", "button_active_text"])
            .unwrap_or(0xffffff);

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(bg_color))
            .track_focus(&focus_handle)
            .on_key_down(on_key)
            .on_mouse_down(MouseButton::Left, on_mouse_down)
            .on_scroll_wheel(on_scroll)
            .child(menu_bar_element)
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_row()
                    .min_h_0()
                    .child(
                        div()
                            .w(px(48.0))
                            .h_full()
                            .bg(rgb(toolbar_bg))
                            .flex()
                            .flex_col()
                            .items_center()
                            .pt(px(4.0))
                            .gap(px(4.0))
                            // Explorer icon button
                            .child(
                                div()
                                    .id("btn-explorer")
                                    .w(px(36.0))
                                    .h(px(36.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded(px(6.0))
                                    .cursor_pointer()
                                    .bg(if self.explorer_open {
                                        rgb(toolbar_active_bg)
                                    } else {
                                        rgb(toolbar_bg)
                                    })
                                    .hover(|s| s.bg(rgb(toolbar_hover_bg)))
                                    .on_mouse_down(MouseButton::Left, on_toggle_explorer)
                                    .child(
                                        div()
                                            .text_size(px(20.0))
                                            .text_color(if self.explorer_open {
                                                rgb(toolbar_active_text)
                                            } else {
                                                rgb(toolbar_text)
                                            })
                                            .child(
                                                img(self
                                                    .menu_icon
                                                    .get("explorer")
                                                    .unwrap()
                                                    .clone())
                                                .size(px(17.0)),
                                            ),
                                    ),
                            )
                            // Search icon button
                            .child(
                                div()
                                    .id("btn-search")
                                    .w(px(36.0))
                                    .h(px(36.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded(px(6.0))
                                    .cursor_pointer()
                                    .bg(if self.search_open {
                                        rgb(toolbar_active_bg)
                                    } else {
                                        rgb(toolbar_bg)
                                    })
                                    .hover(|s| s.bg(rgb(toolbar_hover_bg)))
                                    .on_mouse_down(MouseButton::Left, on_toggle_search)
                                    .child(
                                        div()
                                            .text_size(px(20.0))
                                            .text_color(if self.search_open {
                                                rgb(toolbar_active_text)
                                            } else {
                                                rgb(toolbar_text)
                                            })
                                            .child(
                                                img(self.menu_icon.get("search").unwrap().clone())
                                                    .size(px(17.0)),
                                            ),
                                    ),
                            ),
                    )
                    .when(self.explorer_open, |el| el.child(file_tree_element))
                    .when(self.search_open, |el| el.child(search_element))
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .min_h_0()
                            .child(tabs_bar)
                            .child(div().flex_1().min_h_0().child(editor_element)),
                    ),
            )
            .when(self.menu_bar.file_menu_open, |el| el.child(file_dropdown))
            .when(self.menu_bar.setting_menu_open, |el| {
                el.child(settings_dropdown)
            })
    }
}
