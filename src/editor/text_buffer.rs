/// Centralized module for text buffer management
/// Contains all editing, navigation and cursor position calculation logic

use std::path::PathBuf;

#[derive(Clone)]
pub struct TextBuffer {
    pub text: String,
    pub cursor: usize,
    pub line_count: usize,
    pub current_line: usize,
    pub current_col: usize,
    pub scroll_y: f32,
    pub file_path: Option<PathBuf>,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            line_count: 1,
            current_line: 1,
            current_col: 0,
            scroll_y: 0.0,
            file_path: None,
        }
    }

    // pub fn from_text(text: String) -> Self {
    //     let mut buffer = Self {
    //         text,
    //         cursor: 0,
    //         line_count: 1,
    //         current_line: 1,
    //         current_col: 0,
    //         scroll_y: 0.0,
    //         file_path: None,
    //     };
    //     buffer.update_stats();
    //     buffer
    // }

    /// Insert a character at cursor position
    pub fn insert_char(&mut self, ch: char) {
        self.text.insert(self.cursor, ch);
        self.cursor += ch.len_utf8();
        self.update_stats();
    }

    /// Delete the character before the cursor (backspace)
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            // Find the previous character boundary
            let mut new_cursor = self.cursor - 1;
            while new_cursor > 0 && !self.text.is_char_boundary(new_cursor) {
                new_cursor -= 1;
            }
            
            self.text.drain(new_cursor..self.cursor);
            self.cursor = new_cursor;
            self.update_stats();
        }
    }

    /// Insert a tab (4 spaces)
    pub fn insert_tab(&mut self) {
        for _ in 0..4 {
            self.text.insert(self.cursor, ' ');
            self.cursor += 1;
        }
        self.update_stats();
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            let text_before = &self.text[..self.cursor];
            if let Some(ch) = text_before.chars().last() {
                self.cursor -= ch.len_utf8();
                self.update_stats();
            }
        }
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        if self.cursor < self.text.len() {
            let text_after = &self.text[self.cursor..];
            if let Some(ch) = text_after.chars().next() {
                self.cursor += ch.len_utf8();
                self.update_stats();
            }
        }
    }

    /// Move cursor up
    pub fn move_up(&mut self) {
        let safe_cursor = self.get_safe_cursor();
        let text_before_cursor = &self.text[..safe_cursor];
        
        // Find the start of the current line
        if let Some(current_line_start) = text_before_cursor.rfind('\n') {
            let current_line_start = current_line_start + 1;
            let current_line_text = &self.text[current_line_start..safe_cursor];
            let col_in_chars = current_line_text.chars().count();
            
            // Find the start of the previous line
            if let Some(prev_line_start) = self.text[..current_line_start.saturating_sub(1)].rfind('\n') {
                let prev_line_start = prev_line_start + 1;
                let prev_line_end = current_line_start - 1;
                let prev_line_text = &self.text[prev_line_start..prev_line_end];
                
                let new_pos = prev_line_text.chars()
                    .take(col_in_chars)
                    .map(|c| c.len_utf8())
                    .sum::<usize>();
                
                self.cursor = prev_line_start + new_pos;
            } else if current_line_start > 0 {
                // First line of the file
                let first_line = &self.text[..current_line_start.saturating_sub(1)];
                let new_pos = first_line.chars()
                    .take(col_in_chars)
                    .map(|c| c.len_utf8())
                    .sum::<usize>();
                self.cursor = new_pos.min(current_line_start.saturating_sub(1));
            }
            self.update_stats();
        }
    }

    /// Move cursor down
    pub fn move_down(&mut self) {
        let safe_cursor = self.get_safe_cursor();
        let text_before_cursor = &self.text[..safe_cursor];
        
        let current_line_start = text_before_cursor.rfind('\n').map(|p| p + 1).unwrap_or(0);
        let current_line_text = &self.text[current_line_start..safe_cursor];
        let col_in_chars = current_line_text.chars().count();
        
        // Find the end of the current line
        if let Some(next_line_offset) = self.text[safe_cursor..].find('\n') {
            let next_line_start = safe_cursor + next_line_offset + 1;
            
            let next_line_end = self.text[next_line_start..]
                .find('\n')
                .map(|p| next_line_start + p)
                .unwrap_or(self.text.len());
            
            let next_line_text = &self.text[next_line_start..next_line_end];
            
            let new_pos = next_line_text.chars()
                .take(col_in_chars)
                .map(|c| c.len_utf8())
                .sum::<usize>();
            
            self.cursor = next_line_start + new_pos;
            self.update_stats();
        }
    }

    /// Position cursor from a click (line, column in characters)
    pub fn set_cursor_from_position(&mut self, line: usize, col: usize) {
        let lines: Vec<&str> = self.text.lines().collect();
        
        if lines.is_empty() {
            self.cursor = 0;
            self.update_stats();
            return;
        }
        
        let target_line = line.min(self.line_count).saturating_sub(1);
        let mut position = 0;
        
        // Calculate byte position up to the start of target line
        for i in 0..target_line {
            if i < lines.len() {
                position += lines[i].len() + 1; // +1 for the \n
            }
        }
        
        if target_line < lines.len() {
            let line_text = lines[target_line];
            let char_count = line_text.chars().count();
            let target_col = col.min(char_count);
            
            // Convert character position to byte position
            let byte_offset = line_text.chars()
                .take(target_col)
                .map(|c| c.len_utf8())
                .sum::<usize>();
            
            position += byte_offset;
        }
        
        self.cursor = self.ensure_char_boundary(position.min(self.text.len()));
        self.update_stats();
    }

    /// Update buffer statistics (line count, cursor position)
    pub fn update_stats(&mut self) {
        let safe_cursor = self.get_safe_cursor();
        
        // Calculate line count
        let lines: Vec<&str> = self.text.lines().collect();
        let mut line_count = if self.text.is_empty() { 1 } else { lines.len().max(1) };
        line_count = match self.text.chars().last() {
            Some('\n') => line_count + 1,
            _ => line_count,
        };
        
        // Calculate current line and column
        let text_before_cursor = &self.text[..safe_cursor];
        let mut current_line = if text_before_cursor.is_empty() { 1 } else { text_before_cursor.lines().count() };
        let current_col = text_before_cursor.chars().rev().take_while(|&c| c != '\n').count();
        current_line = match text_before_cursor.chars().last() {
            Some('\n') => current_line + 1,
            _ => current_line,
        };
        
        self.line_count = line_count;
        self.current_line = current_line;
        self.current_col = current_col;
        self.cursor = safe_cursor;
    }

    /// Get a valid cursor on a UTF-8 character boundary
    fn get_safe_cursor(&self) -> usize {
        let safe_cursor = self.cursor.min(self.text.len());
        self.ensure_char_boundary(safe_cursor)
    }

    /// Ensure position is on a valid character boundary
    fn ensure_char_boundary(&self, position: usize) -> usize {
        if self.text.is_char_boundary(position) {
            position
        } else {
            (0..=position).rev().find(|&i| self.text.is_char_boundary(i)).unwrap_or(0)
        }
    }

    /// Auto-scroll to keep cursor visible
    pub fn auto_scroll_to_cursor(&mut self, viewport_height: f32, line_height: f32) {
        let cursor_y = (self.current_line.saturating_sub(1)) as f32 * line_height;
        
        // Scroll down if necessary
        if cursor_y > self.scroll_y + viewport_height - line_height * 2.0 {
            self.scroll_y = cursor_y - viewport_height + line_height * 3.0;
        }
        
        // Scroll up if necessary
        if cursor_y < self.scroll_y + line_height {
            self.scroll_y = (cursor_y - line_height).max(0.0);
        }
        
        // Limit scroll
        self.scroll_y = self.scroll_y.max(0.0);
        let content_height = self.line_count as f32 * line_height;
        let max_scroll = (content_height - viewport_height).max(0.0);
        self.scroll_y = self.scroll_y.min(max_scroll);
    }

    /// Load text from a file
    pub fn load_from_file(&mut self, path: PathBuf, content: String) {
        self.text = content;
        self.file_path = Some(path);
        self.cursor = 0;
        self.scroll_y = 0.0;
        self.update_stats();
    }

    /// Get file extension for syntax highlighting
    pub fn get_file_extension(&self) -> String {
        self.file_path
            .as_ref()
            .and_then(|p| p.extension())
            .and_then(|e| e.to_str())
            .unwrap_or("txt")
            .to_string()
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}
