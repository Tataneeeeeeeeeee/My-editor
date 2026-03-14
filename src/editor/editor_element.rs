use gpui::*;

use crate::settings::settings::SettingsGlobal;

use super::syntax_highlighter::SyntaxHighlighter;
use syntect::highlighting::Style as SyntectStyle;

pub struct EditorElement {
    text: String,
    cursor: usize,
    line_count: usize,
    current_line: usize,
    current_col: usize,
    scroll_y: f32,
    viewport_height: f32,
    syntax_highlighter: SyntaxHighlighter,
    file_extension: String,
    settings_global: SettingsGlobal,
}

impl EditorElement {
    pub fn new(text: String, cursor: usize, line_count: usize, current_line: usize, current_col: usize, scroll_y: f32, viewport_height: f32, settings_global: SettingsGlobal) -> Self {
        Self { 
            text,
            cursor,
            line_count,
            current_line,
            current_col,
            scroll_y,
            viewport_height,
            syntax_highlighter: SyntaxHighlighter::new(),
            file_extension: "rs".to_string(), // Default: Rust
            settings_global,
        }
    }
    
    pub fn with_file_extension(mut self, extension: String) -> Self {
        self.file_extension = extension;
        self
    }

    // Calculate visible lines based on scroll position
    fn get_visible_line_range(&self) -> (usize, usize) {
        let line_height = self.settings_global.get_f32(vec!["ui", "editor", "line_height_px"]).unwrap_or(19.2);
        const BUFFER_LINES: usize = 5; // Buffer lines above and below
        
        let first_visible = (self.scroll_y / line_height).floor() as usize;
        let visible_count = (self.viewport_height / line_height).ceil() as usize;
        let last_visible = first_visible + visible_count;
        
        // Add buffer for anticipatory rendering
        let start = first_visible.saturating_sub(BUFFER_LINES);
        let end = (last_visible + BUFFER_LINES).min(self.line_count);
        
        (start, end)
    }

    fn render_gutter(&self) -> impl IntoElement {
        let (start_line, end_line) = self.get_visible_line_range();
        let line_height = self.settings_global.get_f32(vec!["ui", "editor", "line_height_px"]).unwrap_or(19.2);
        let gutter_text_color = self.settings_global.get_color(vec!["ui", "editor", "gutter", "text_color"]).unwrap_or(0x858585);
        
        div()
            .text_color(rgb(gutter_text_color))
            .mr_2()
            .min_w(px(50.0))
            .text_align(TextAlign::Right)
            .text_size(px(16.0))
            .line_height(relative(1.2))
            .flex()
            .flex_col()
            .relative()
            .child(
                // Spacer to offset line numbers according to scroll
                div()
                    .h(px(start_line as f32 * line_height))
            )
            .children(
                (start_line..end_line).map(|i| {
                    div()
                        .child(format!("{:>4} ", i + 1))
                })
            )
    }

    fn render_text_with_cursor(&self) -> impl IntoElement {
        let char_spacing = self.settings_global.get_f32(vec!["ui", "editor", "char_spacing_px"]).unwrap_or(0.5);
        
        // Split text into lines
        let mut lines: Vec<String> = Vec::new();
        
        if self.text.is_empty() {
            lines.push(String::new());
        } else {
            let mut current_line = String::new();
            for ch in self.text.chars() {
                if ch == '\n' {
                    lines.push(current_line.clone());
                    current_line.clear();
                } else {
                    current_line.push(ch);
                }
            }
            lines.push(current_line);
        }
        
        // Configure highlighter
        let syntax = self.syntax_highlighter.get_syntax(&self.file_extension);
        
        let mut highlighter = self.syntax_highlighter.create_highlighter(syntax);
        
        // Only render visible lines
        let (start_line, end_line) = self.get_visible_line_range();
        
        // Highlight ALL lines to maintain correct state
        // but only keep visible lines for rendering
        let mut visible_lines_with_highlighting = Vec::new();
        
        for (idx, line) in lines.iter().enumerate() {
            let styled_segments = if line.is_empty() {
                vec![]
            } else {
                // Add a \n at the end of the line for syntect
                let line_with_newline = format!("{}\n", line);
                let mut segments = self.syntax_highlighter.highlight_line(&line_with_newline, syntax, &mut highlighter);
                
                // Remove the \n from the last segment
                if let Some(last_segment) = segments.last_mut() {
                    if last_segment.1.ends_with('\n') {
                        last_segment.1.pop();
                    }
                }
                
                segments
            };
            
            // Only keep lines in visible range
            if idx >= start_line && idx < end_line {
                visible_lines_with_highlighting.push(styled_segments);
            }
        }
        
        div()
            .relative()
            .text_size(px(16.0))
            .line_height(relative(1.2))
            .flex()
            .flex_col()
            .child(
                // Spacer to offset content according to scroll
                div()
                    .h(px(start_line as f32 * self.settings_global.get_f32(vec!["ui", "editor", "line_height_px"]).unwrap_or(19.2)))
            )
            .children(
                visible_lines_with_highlighting.into_iter().map(|styled_segments| {
                    div()
                        .flex()
                        .flex_row()
                        .min_h(px(19.2))
                        .font_family("monospace")
                        .children(
                            if styled_segments.is_empty() {
                                // Empty line
                                vec![div().child(" ")]
                            } else {
                                // Render each segment with its color
                                // BUT we split each segment into individual characters for spacing
                                styled_segments.into_iter().flat_map(|(style, text)| {
                                    let color = syntect_color_to_gpui(style);
                                    text.chars().map(move |c| {
                                        div()
                                            .pr(px(char_spacing))
                                            .text_color(color)
                                            .child(c.to_string())
                                    }).collect::<Vec<_>>()
                                }).collect()
                            }
                        )
                })
            )
            .child(self.render_cursor())
    }

    fn render_cursor(&self) -> impl IntoElement {
        let line_height = self.settings_global.get_f32(vec!["ui", "editor", "line_height_px"]).unwrap_or(19.2);
        let char_spacing = self.settings_global.get_f32(vec!["ui", "editor", "char_spacing_px"]).unwrap_or(0.5);
        let cursor_color = self.settings_global.get_color(vec!["ui", "editor", "cursor_color"]).unwrap_or(0xffffff);
        let cursor_opacity = self.settings_global.get_f32(vec!["ui", "editor", "cursor_opacity"]).unwrap_or(0.9);
        let cursor_width = self.settings_global.get_f32(vec!["ui", "editor", "cursor_width_px"]).unwrap_or(2.0);
        
        let safe_cursor = self.cursor.min(self.text.len());
        let text_before_cursor = &self.text[..safe_cursor];
        let line_index = self.current_line.saturating_sub(1);
        
        let text_on_line = text_before_cursor
            .chars()
            .rev()
            .take_while(|&c| c != '\n')
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<String>();
        
        let cursor_y = line_index as f32 * line_height;
        
        // Render each character individually with spacing to align cursor
        div()
            .absolute()
            .left_0()
            .top(px(cursor_y))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .text_size(px(16.0))
                    .font_family("monospace")
                    .child(
                        div()
                            .opacity(0.0)
                            .flex()
                            .flex_row()
                            .children(
                                text_on_line.chars().map(|c| {
                                    div()
                                        .pr(px(char_spacing))
                                        .child(c.to_string())
                                })
                            )
                    )
                    .child(
                        div()
                            .w(px(cursor_width))
                            .h(px(line_height))
                            .bg(rgb(cursor_color))
                            .opacity(cursor_opacity)
                    )
            )
    }

    fn render_status_bar(&self) -> impl IntoElement {
        let text_color = self.settings_global.get_color(vec!["ui", "panels", "status_bar", "text_color"]).unwrap_or(0x858585);
        let border_color = self.settings_global.get_color(vec!["ui", "panels", "status_bar", "border_color"]).unwrap_or(0x404040);
        
        div()
            .text_color(rgb(text_color))
            .p_2()
            .border_1()
            .border_color(rgb(border_color))
            .rounded_md()
            .flex()
            .justify_between()
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(format!("Ln {}, Col {}", self.current_line, self.current_col))
                    .child("|")
                    .child("UTF-8")
            )
            .child(
                div()
                    .child(match self.settings_global.get(vec!["file_extensions", &self.file_extension, "name"]) {
                        Ok(name) => name,
                        Err(_) => "Unknown".to_string(),
                    })
            )
    }

    fn render_editor_content(&self) -> impl IntoElement {
        let bg_color = self.settings_global.get_color(vec!["ui", "editor", "background"]).unwrap_or(0x1e1e1e);
        
        div()
            .id("scrollable-container")
            .h_full()
            .overflow_hidden()
            .bg(rgb(bg_color))
            .flex()
            .flex_row()
            .child(
                // Main editing area - absolute positioning to prevent overflow
                div()
                    .flex_1()
                    .h_full() // Force height to 100%
                    .overflow_hidden() // Prevent overflow
                    .relative() // Reference container for absolute positioning
                    .child(
                        // Scrollable content in absolute position
                        div()
                            .absolute()
                            .left_0()
                            .right_0()
                            .top(px(-self.scroll_y))
                            .flex()
                            .p_4()
                            .child(self.render_gutter())
                            .child(self.render_text_with_cursor())
                    )
            )
            .child(
                // Scrollbar on the right
                self.render_scrollbar()
            )
    }

    fn render_scrollbar(&self) -> impl IntoElement {
        let line_height = self.settings_global.get_f32(vec!["ui", "editor", "line_height_px"]).unwrap_or(19.2);
        let scrollbar_width = self.settings_global.get_f32(vec!["ui", "editor", "scrollbar", "width_px"]).unwrap_or(14.0);
        let scrollbar_bg = self.settings_global.get_color(vec!["ui", "editor", "scrollbar", "background"]).unwrap_or(0x1e1e1e);
        let scrollbar_track_bg = self.settings_global.get_color(vec!["ui", "colors", "background"]).unwrap_or(0x2d2d30);
        let thumb_color = self.settings_global.get_color(vec!["ui", "editor", "scrollbar", "thumb_color"]).unwrap_or(0x424242);
        let thumb_hover_color = self.settings_global.get_color(vec!["ui", "editor", "scrollbar", "thumb_hover_color"]).unwrap_or(0x4f4f4f);
        
        let content_height = self.line_count as f32 * line_height;
        let max_scroll = (content_height - self.viewport_height).max(0.0);
        
        let scrollbar_height = self.viewport_height;
        let thumb_ratio = (self.viewport_height / content_height.max(self.viewport_height)).min(1.0);
        let thumb_height = (scrollbar_height * thumb_ratio).max(30.0);
        
        let scroll_ratio = if max_scroll > 0.0 {
            self.scroll_y / max_scroll
        } else {
            0.0
        };
        let thumb_y = scroll_ratio * (scrollbar_height - thumb_height);
        
        div()
            .w(px(scrollbar_width))
            .h_full()
            .bg(rgb(scrollbar_bg))
            .flex()
            .flex_col()
            .justify_start()
            .child(
                div()
                    .relative()
                    .w_full()
                    .h_full()
                    .bg(rgb(scrollbar_track_bg))
                    .child(
                        div()
                            .absolute()
                            .left(px(2.0))
                            .top(px(thumb_y))
                            .w(px(scrollbar_width - 4.0))
                            .h(px(thumb_height))
                            .bg(rgb(thumb_color))
                            .rounded(px(4.0))
                            .hover(|style| style.bg(rgb(thumb_hover_color)))
                    )
            )
    }
}

impl IntoElement for EditorElement
{
    type Element = Div;

    fn into_element(self) -> Self::Element {
        div()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .font_family("monospace")
            .flex()
            .flex_col()
            .overflow_hidden() // Prevent overflow of main container
            .child(
                // Editing area with flexible but strictly limited height
                div()
                    .flex_1()
                    .min_h_0() // Important: allows flex_1 to shrink
                    .overflow_hidden() // Prevent overflow
                    .child(self.render_editor_content())
            )
            .child(
                // Fixed status bar at bottom - fixed height
                div()
                    .flex_shrink_0() // Never shrink
                    .h(px(60.0)) // Fixed height for status bar
                    .px_4()
                    .py_2()
                    .child(self.render_status_bar())
            )
    }
}

// Convert a syntect color to gpui color
fn syntect_color_to_gpui(style: SyntectStyle) -> Rgba {
    // Convert RGB components (0-255) to 0xRRGGBB format
    let r = style.foreground.r as u32;
    let g = style.foreground.g as u32;
    let b = style.foreground.b as u32;
    
    // Format: 0xRRGGBB
    let color_value = (r << 16) | (g << 8) | b;
    
    rgb(color_value)
}
