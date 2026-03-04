use gpui::*;
use gpui::prelude::FluentBuilder;
use super::text_buffer::TextBuffer;
use super::menu_bar::menu_bar::{MenuBar, MenuAction};
use super::tab_bar;
use super::menu_bar;
use super::key;

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

/// Main window containing multiple editor tabs
pub struct EditorWindow {
    pub tabs: Vec<EditorTab>,
    pub active_tab_index: usize,
    pub next_tab_id: usize,
    pub menu_bar: MenuBar,
}

impl EditorWindow {
    pub fn new(_id: usize, title: String, cx: &mut Context<Self>) -> Self {
        let first_focus = cx.focus_handle();
        let first_tab = EditorTab::new(0, title, first_focus);
        
        Self {
            tabs: vec![first_tab],
            active_tab_index: 0,
            next_tab_id: 1,
            menu_bar: MenuBar::new(),
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
}

impl Render for EditorWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use super::editor_element::EditorElement;

        let on_key = cx.listener(|this, event: &KeyDownEvent, window, cx| {
            if !key::shortcuts::handle_ctrl(this, event, cx) {
                key::input::handle_input(this, event, window, cx);
            }
        });

        let on_mouse_down = cx.listener(|this, event: &MouseDownEvent, _window, cx| {
            const LINE_HEIGHT: f32 = 19.2;
            const GUTTER_WIDTH: f32 = 50.0;
            const GUTTER_MARGIN: f32 = 8.0;
            const PADDING_LEFT: f32 = 16.0;
            const TOTAL_OFFSET: f32 = GUTTER_WIDTH + GUTTER_MARGIN + PADDING_LEFT;
            const MONOSPACE_CHAR_WIDTH: f32 = 8.0;
            const TAB_BAR_HEIGHT: f32 = 40.0;
            const MENU_BAR_HEIGHT: f32 = 30.0;
            const SCROLLBAR_WIDTH: f32 = 14.0;

            let x: f32 = event.position.x.into();
            let y: f32 = event.position.y.into();

            if y < (MENU_BAR_HEIGHT + TAB_BAR_HEIGHT) {
                return;
            }

            let window_width = 800.0;
            let active_tab = &mut this.tabs[this.active_tab_index];
            let buffer = &mut active_tab.buffer;

            if x >= window_width - SCROLLBAR_WIDTH {
                const STATUS_BAR_HEIGHT: f32 = 60.0;
                let viewport_height = 600.0 - TAB_BAR_HEIGHT - STATUS_BAR_HEIGHT;
                let scrollbar_y = y - MENU_BAR_HEIGHT - TAB_BAR_HEIGHT;
                let content_height = buffer.line_count as f32 * LINE_HEIGHT;
                let max_scroll = (content_height - viewport_height).max(0.0);
                let scroll_ratio = scrollbar_y / viewport_height;
                buffer.scroll_y = (scroll_ratio * max_scroll).max(0.0).min(max_scroll);
                cx.notify();
                return;
            }

            let top_padding: f32 = 16.0 + TAB_BAR_HEIGHT + 16.0;
            let adjusted_y = y + buffer.scroll_y - top_padding;
            let line_index = (adjusted_y / LINE_HEIGHT).max(1.0) as usize;
            let approximate_col = ((x - TOTAL_OFFSET) / MONOSPACE_CHAR_WIDTH).round() as usize;
            buffer.set_cursor_from_position(line_index, approximate_col);
            cx.notify();
        });

        let on_scroll = cx.listener(|this, event: &ScrollWheelEvent, _window, cx| {
            const LINE_HEIGHT: f32 = 19.2;
            let active_tab = &mut this.tabs[this.active_tab_index];
            let buffer = &mut active_tab.buffer;
            let delta_y: f32 = event.delta.pixel_delta(px(1.0)).y.into();
            buffer.scroll_y = (buffer.scroll_y - delta_y * 2.5).max(0.0);
            let max_scroll = ((buffer.line_count as f32 * LINE_HEIGHT) - 600.0).max(0.0);
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

        let dropdown = menu_bar::bar_element::render_dropdown(on_new_file, on_open_file, on_save_file);

        let file_menu_open = self.menu_bar.file_menu_open;
        let focus_handle = self.tabs[self.active_tab_index].focus_handle.clone();

        let tabs_info: Vec<(usize, String, bool, bool)> = self.tabs.iter().enumerate()
            .map(|(i, tab)| (i, tab.title.clone(), i == self.active_tab_index, tab.is_modified))
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
        ).with_file_extension(buffer.get_file_extension());

        let tabs_bar = tab_bar::bar_element::render_bar(&tabs_info, cx);
        let menu_bar_element = self.menu_bar.render(file_menu_open, cx);

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x1e1e1e))
            .track_focus(&focus_handle)
            .on_key_down(on_key)
            .on_mouse_down(MouseButton::Left, on_mouse_down)
            .on_scroll_wheel(on_scroll)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .child(menu_bar_element)
                    .child(tabs_bar)
                    .child(div().flex_1().child(editor_element))
            )
            .when(file_menu_open, |el| el.child(dropdown))
    }
}
