use gpui::*;
// use super::text_layout::TextLayout;
use super::syntax_highlighter::SyntaxHighlighter;
use syntect::highlighting::Style as SyntectStyle;


const EXTENSION_MAP: &[(&str, &str)] = &[
    ("rs", "Rust"),
    ("js", "JavaScript"),
    ("py", "Python"),
    ("java", "Java"),
    ("cpp", "C++"),
    ("c", "C"),
    ("html", "HTML"),
    ("css", "CSS"),
    ("json", "JSON"),
    ("md", "Markdown"),
    ("toml", "TOML"),
    ("txt", "Text"),
];


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
}

impl EditorElement {
    pub fn new(text: String, cursor: usize, line_count: usize, current_line: usize, current_col: usize, scroll_y: f32, viewport_height: f32) -> Self {
        Self { 
            text,
            cursor,
            line_count,
            current_line,
            current_col,
            scroll_y,
            viewport_height,
            syntax_highlighter: SyntaxHighlighter::new(),
            file_extension: "rs".to_string(), // Par défaut Rust
        }
    }
    
    pub fn with_file_extension(mut self, extension: String) -> Self {
        self.file_extension = extension;
        self
    }

    // Calculer les lignes visibles en fonction du scroll
    fn get_visible_line_range(&self) -> (usize, usize) {
        const LINE_HEIGHT: f32 = 19.2;
        const BUFFER_LINES: usize = 5; // Lignes de buffer au-dessus et en-dessous
        
        let first_visible = (self.scroll_y / LINE_HEIGHT).floor() as usize;
        let visible_count = (self.viewport_height / LINE_HEIGHT).ceil() as usize;
        let last_visible = first_visible + visible_count;
        
        // Ajouter un buffer pour le rendu anticipé
        let start = first_visible.saturating_sub(BUFFER_LINES);
        let end = (last_visible + BUFFER_LINES).min(self.line_count);
        
        (start, end)
    }

    fn render_gutter(&self) -> impl IntoElement {
        let (start_line, end_line) = self.get_visible_line_range();
        const LINE_HEIGHT: f32 = 19.2;
        
        div()
            .text_color(rgb(0x858585))
            .mr_2()
            .min_w(px(50.0))
            .text_align(TextAlign::Right)
            .text_size(px(16.0))
            .line_height(relative(1.2))
            .flex()
            .flex_col()
            .relative()
            .child(
                // Spacer pour décaler les numéros selon le scroll
                div()
                    .h(px(start_line as f32 * LINE_HEIGHT))
            )
            .children(
                (start_line..end_line).map(|i| {
                    div()
                        .child(format!("{:>4}", i + 1))
                })
            )
    }

    fn render_text_with_cursor(&self) -> impl IntoElement {
        const LINE_HEIGHT: f32 = 19.2;
        const CHAR_SPACING: f32 = 0.5;
        
        // Diviser le texte en lignes
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
        
        // Configurer le highlighter
        let syntax = self.syntax_highlighter.get_syntax(&self.file_extension);
        
        let mut highlighter = self.syntax_highlighter.create_highlighter(syntax);
        
        // Ne rendre que les lignes visibles
        let (start_line, end_line) = self.get_visible_line_range();
        
        // Highlighter TOUTES les lignes pour maintenir l'état correct
        // mais ne garder que les lignes visibles pour le rendu
        let mut visible_lines_with_highlighting = Vec::new();
        
        for (idx, line) in lines.iter().enumerate() {
            let styled_segments = if line.is_empty() {
                vec![]
            } else {
                // Ajouter un \n à la fin de la ligne pour syntect
                let line_with_newline = format!("{}\n", line);
                let mut segments = self.syntax_highlighter.highlight_line(&line_with_newline, syntax, &mut highlighter);
                
                // Retirer le \n du dernier segment
                if let Some(last_segment) = segments.last_mut() {
                    if last_segment.1.ends_with('\n') {
                        last_segment.1.pop();
                    }
                }
                
                segments
            };
            
            // Ne garder que les lignes dans la plage visible
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
                // Spacer pour décaler le contenu selon le scroll
                div()
                    .h(px(start_line as f32 * LINE_HEIGHT))
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
                                // Ligne vide
                                vec![div().child(" ")]
                            } else {
                                // Rendre chaque segment avec sa couleur
                                // MAIS on divise chaque segment en caractères individuels pour l'espacement
                                styled_segments.into_iter().flat_map(|(style, text)| {
                                    let color = syntect_color_to_gpui(style);
                                    text.chars().map(move |c| {
                                        div()
                                            .pr(px(CHAR_SPACING))
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
        const LINE_HEIGHT: f32 = 19.2;
        const CHAR_SPACING: f32 = 0.5;
        
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
        
        let cursor_y = line_index as f32 * LINE_HEIGHT;
        
        // Rendre chaque caractère individuellement avec l'espacement pour aligner le curseur
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
                                        .pr(px(CHAR_SPACING))
                                        .child(c.to_string())
                                })
                            )
                    )
                    .child(
                        div()
                            .w(px(2.0))
                            .h(px(LINE_HEIGHT))
                            .bg(rgb(0xffffff))
                            .opacity(0.9)
                    )
            )
    }

    fn render_status_bar(&self) -> impl IntoElement {
        div()
            .text_color(rgb(0x858585))
            .p_2()
            .border_1()
            .border_color(rgb(0x404040))
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
                    .child(EXTENSION_MAP.iter()
                        .find(|&&(ext, _)| ext == self.file_extension)
                        .map(|&(_, name)| name)
                        .unwrap_or("Unknown"))
            )
    }

    fn render_editor_content(&self) -> impl IntoElement {
        div()
            .id("scrollable-container")
            .h_full() // Utiliser toute la hauteur disponible
            .overflow_hidden() // Crucial: empêcher le débordement
            .bg(rgb(0x1e1e1e))
            .flex()
            .flex_row()
            .child(
                // Zone d'édition principale - positionnement absolu pour empêcher le débordement
                div()
                    .flex_1()
                    .h_full() // Forcer la hauteur à 100%
                    .overflow_hidden() // Empêcher le débordement
                    .relative() // Conteneur de référence pour le positionnement absolu
                    .child(
                        // Contenu scrollable en position absolue
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
                // Barre de défilement à droite
                self.render_scrollbar()
            )
    }

    fn render_scrollbar(&self) -> impl IntoElement {
        const LINE_HEIGHT: f32 = 19.2;
        const SCROLLBAR_WIDTH: f32 = 14.0;
        
        let content_height = self.line_count as f32 * LINE_HEIGHT;
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
            .w(px(SCROLLBAR_WIDTH))
            .h_full()
            .bg(rgb(0x1e1e1e))
            .flex()
            .flex_col()
            .justify_start()
            .child(
                div()
                    .relative()
                    .w_full()
                    .h_full()
                    .bg(rgb(0x2d2d30))
                    .child(
                        div()
                            .absolute()
                            .left(px(2.0))
                            .top(px(thumb_y))
                            .w(px(SCROLLBAR_WIDTH - 4.0))
                            .h(px(thumb_height))
                            .bg(rgb(0x424242))
                            .rounded(px(4.0))
                            .hover(|style| style.bg(rgb(0x4f4f4f)))
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
            .overflow_hidden() // Empêcher le débordement du conteneur principal
            .child(
                // Zone d'édition avec hauteur flexible mais strictement limitée
                div()
                    .flex_1()
                    .min_h_0() // Important: permet au flex_1 de rétrécir
                    .overflow_hidden() // Empêcher le débordement
                    .child(self.render_editor_content())
            )
            .child(
                // Barre de statut fixe en bas - hauteur fixe
                div()
                    .flex_shrink_0() // Ne jamais rétrécir
                    .h(px(60.0)) // Hauteur fixe pour la barre de statut
                    .px_4()
                    .py_2()
                    .child(self.render_status_bar())
            )
    }
}

// Convertir une couleur syntect en couleur gpui
fn syntect_color_to_gpui(style: SyntectStyle) -> Rgba {
    // Convertir les composantes RGB (0-255) en format 0xRRGGBB
    let r = style.foreground.r as u32;
    let g = style.foreground.g as u32;
    let b = style.foreground.b as u32;
    
    // Format: 0xRRGGBB
    let color_value = (r << 16) | (g << 8) | b;
    
    rgb(color_value)
}
