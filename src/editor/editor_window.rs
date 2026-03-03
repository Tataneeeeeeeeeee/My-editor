use gpui::*;
use gpui::prelude::*;
use super::text_buffer::TextBuffer;
use super::menu_bar::{MenuBar, MenuAction};
// use super::text_layout::TextLayout;

// Configuration de l'affichage
const LINE_HEIGHT: f32 = 19.2;
// const FONT_SIZE: f32 = 16.0;
const GUTTER_WIDTH: f32 = 50.0;
const GUTTER_MARGIN: f32 = 8.0;
const PADDING_LEFT: f32 = 16.0;
const TOTAL_OFFSET: f32 = GUTTER_WIDTH + GUTTER_MARGIN + PADDING_LEFT;
const MONOSPACE_CHAR_WIDTH: f32 = 8.0;

/// Représente un onglet d'éditeur avec son contenu
pub struct EditorTab {
    pub buffer: TextBuffer,
    pub title: String,
    pub focus_handle: FocusHandle,
}

impl EditorTab {
    pub fn new(_id: usize, title: String, focus_handle: FocusHandle) -> Self {
        Self {
            buffer: TextBuffer::new(),
            title,
            focus_handle,
        }
    }
}

/// Fenêtre principale qui contient plusieurs onglets d'éditeur
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

    /// Ajoute un nouvel onglet
    pub fn add_tab(&mut self, title: String, cx: &mut Context<Self>) {
        let new_focus = cx.focus_handle();
        let new_tab = EditorTab::new(self.next_tab_id, title, new_focus);
        
        self.next_tab_id += 1;
        self.tabs.push(new_tab);
        self.active_tab_index = self.tabs.len() - 1;
        cx.notify();
    }

    /// Ferme un onglet par index
    pub fn close_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.tabs.len() > 1 && index < self.tabs.len() {
            self.tabs.remove(index);
            if self.active_tab_index >= self.tabs.len() {
                self.active_tab_index = self.tabs.len() - 1;
            }
            cx.notify();
        }
    }

    /// Change l'onglet actif
    pub fn set_active_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.tabs.len() {
            self.active_tab_index = index;
            cx.notify();
        }
    }

    /// Gère les actions du menu
    pub fn handle_menu_action(&mut self, action: MenuAction, cx: &mut Context<Self>) {
        self.menu_bar.file_menu_open = false;
        
        match action {
            MenuAction::NewFile => {
                self.add_tab("Untitled".to_string(), cx);
            }
            MenuAction::OpenFile => {
                self.open_file(cx);
            }
            MenuAction::SaveFile => {
                self.save_file(cx);
            }
        }
    }

    /// Ouvre un fichier avec un dialogue de sélection
    fn open_file(&mut self, cx: &mut Context<Self>) {
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

    /// Sauvegarde le fichier actif
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

    /// Sauvegarde le fichier avec un dialogue "Save As"
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
        
        // Collecter les informations des tabs
        let tabs_info: Vec<(usize, String, bool)> = self.tabs.iter().enumerate()
            .map(|(index, tab)| (index, tab.title.clone(), index == self.active_tab_index))
            .collect();
        
        let active_tab = &mut self.tabs[self.active_tab_index];
        let buffer = &mut active_tab.buffer;
        
        let on_key = cx.listener(
            |this: &mut Self, event: &KeyDownEvent, _window, cx| {
                // Ctrl+T pour nouveau tab
                if event.keystroke.modifiers.control && event.keystroke.key == "t" {
                    this.add_tab("Untitled".to_string(), cx);
                    return;
                }
                
                // Ctrl+W pour fermer le tab actif
                if event.keystroke.modifiers.control && event.keystroke.key == "w" {
                    if this.tabs.len() > 1 {
                        this.close_tab(this.active_tab_index, cx);
                    }
                    return;
                }

                // Ctrl+Tab pour changer de tab
                if event.keystroke.modifiers.control && event.keystroke.key == "tab" {
                    let next_index = (this.active_tab_index + 1) % this.tabs.len();
                    this.set_active_tab(next_index, cx);
                    return;
                }

                let active_tab = &mut this.tabs[this.active_tab_index];
                let buffer = &mut active_tab.buffer;
                
                // Gestion des caractères
                if let Some(s) = &event.keystroke.key_char {
                    if let Some(ch) = s.chars().next() {
                        buffer.insert_char(ch);
                        cx.notify();
                        return;
                    }
                }
        
                // Gestion des touches spéciales
                match event.keystroke.key.as_str() {
                    "backspace" => buffer.backspace(),
                    "enter" => {
                        buffer.insert_char('\n');
                        let viewport_height: f32 = _window.viewport_size().height.into();
                        let viewport_height = viewport_height - 100.0;
                        buffer.auto_scroll_to_cursor(viewport_height, LINE_HEIGHT);
                    }
                    "tab" => buffer.insert_tab(),
                    "left" => buffer.move_left(),
                    "right" => buffer.move_right(),
                    "up" => buffer.move_up(),
                    "down" => buffer.move_down(),
                    _ => return,
                }
        
                cx.notify();
            },
        );

        let on_mouse_down = cx.listener(
            |this: &mut Self, event: &MouseDownEvent, _window, cx| {
                const TOP_PADDING: f32 = 16.0 + 40.0 + 16.0;
                const TAB_BAR_HEIGHT: f32 = 40.0;
                const MENU_BAR_HEIGHT: f32 = 30.0;
                const SCROLLBAR_WIDTH: f32 = 14.0;
                
                let x: f32 = event.position.x.into();
                let y: f32 = event.position.y.into();
                
                // Ignorer les clics dans la zone des menus/onglets
                if y < (MENU_BAR_HEIGHT + TAB_BAR_HEIGHT) {
                    return;
                }
                
                let window_width = 800.0;
                let scrollbar_x = window_width - SCROLLBAR_WIDTH;
                
                let active_tab = &mut this.tabs[this.active_tab_index];
                let buffer = &mut active_tab.buffer;
                
                // Gestion du clic sur la scrollbar
                if x >= scrollbar_x {
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
                
                // Positionnement du curseur par clic
                let adjusted_y = y + buffer.scroll_y - TOP_PADDING;
                let line_index = (adjusted_y / LINE_HEIGHT).max(1.0) as usize;
                let click_offset = x - TOTAL_OFFSET;
                let approximate_col = (click_offset / MONOSPACE_CHAR_WIDTH).round() as usize;
                
                buffer.set_cursor_from_position(line_index, approximate_col);
                cx.notify();
            },
        );

        let on_scroll = cx.listener(
            |this: &mut Self, event: &ScrollWheelEvent, _window, cx| {
                let active_tab = &mut this.tabs[this.active_tab_index];
                let buffer = &mut active_tab.buffer;
                
                let delta_y: f32 = event.delta.pixel_delta(px(1.0)).y.into();
                buffer.scroll_y -= delta_y * 2.5;
                buffer.scroll_y = buffer.scroll_y.max(0.0);
                
                let content_height = buffer.line_count as f32 * LINE_HEIGHT;
                let viewport_height = 600.0;
                let max_scroll = (content_height - viewport_height).max(0.0);
                
                buffer.scroll_y = buffer.scroll_y.min(max_scroll);
                cx.notify();
            },
        );

        // Calculer la hauteur de la viewport
        let viewport_height: f32 = _window.viewport_size().height.into();
        let viewport_height = viewport_height - 60.0;

        let file_extension = buffer.get_file_extension();
        let editor_element = EditorElement::new(
            buffer.text.clone(),
            buffer.cursor,
            buffer.line_count,
            buffer.current_line,
            buffer.current_col,
            buffer.scroll_y,
            viewport_height,
        ).with_file_extension(file_extension);

        // Barre d'onglets
        let tabs_bar = div()
            .h(px(40.0))
            .bg(rgb(0x252526))
            .flex()
            .flex_row()
            .items_center()
            .children(
                tabs_info.iter().map(|(tab_index, title, is_active)| {
                    let tab_idx = *tab_index;
                    let is_act = *is_active;

                    let mut name = format!("{}", title);
                    if tabs_info.iter().filter(|(_, t, _)| t == title).count() > 1 {
                        // Si plusieurs onglets ont le même titre, ajouter un suffixe pour les différencier
                        let suffix = format!(" ({})", tab_idx + 1);
                        name.push_str(&suffix);
                    }
                    
                    div()
                        .h_full()
                        .px_4()
                        .flex()
                        .items_center()
                        .bg(if is_act { rgb(0x1e1e1e) } else { rgb(0x2d2d30) })
                        .border_r_1()
                        .border_color(rgb(0x1e1e1e))
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                            this.set_active_tab(tab_idx, cx);
                        }))
                        .child(
                            div()
                                .text_color(if is_act { rgb(0xffffff) } else { rgb(0x969696) })
                                .child(name)
                        )
                })
            );

        // Barre de menu
        let file_menu_open = self.menu_bar.file_menu_open;
        let menu_bar_element = self.menu_bar.render(file_menu_open, cx);
        
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x1e1e1e))
            .track_focus(&active_tab.focus_handle)
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
                    .child(
                        div()
                            .flex_1()
                            .child(editor_element)
                    )
            )
            .when(file_menu_open, |element| {
                element.child(
                    div()
                        .absolute()
                        .top(px(30.0))
                        .left(px(8.0))
                        .w(px(200.0))
                        .bg(rgb(0x252526))
                        .border_1()
                        .border_color(rgb(0x454545))
                        .shadow_lg()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .px_4()
                                .py_2()
                                .text_color(rgb(0xcccccc))
                                .hover(|style| style.bg(rgb(0x094771)))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                    this.handle_menu_action(MenuAction::NewFile, cx);
                                }))
                                .child("New Text File")
                        )
                        .child(
                            div()
                                .px_4()
                                .py_2()
                                .text_color(rgb(0xcccccc))
                                .hover(|style| style.bg(rgb(0x094771)))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                    this.handle_menu_action(MenuAction::OpenFile, cx);
                                }))
                                .child("Open File...")
                        )
                        .child(
                            div()
                                .px_4()
                                .py_2()
                                .text_color(rgb(0xcccccc))
                                .hover(|style| style.bg(rgb(0x094771)))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                    this.handle_menu_action(MenuAction::SaveFile, cx);
                                }))
                                .child("Save File")
                        )
                )
            })
    }
}
