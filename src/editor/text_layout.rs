// use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping};
// use std::sync::{Arc, Mutex};

// /// Text measurement manager using cosmic-text
// pub struct TextLayout {
//     font_system: Arc<Mutex<FontSystem>>,
//     font_size: f32,
//     line_height: f32,
// }

// impl TextLayout {
//     pub fn new(font_size: f32, line_height: f32) -> Self {
//         Self {
//             font_system: Arc::new(Mutex::new(FontSystem::new())),
//             font_size,
//             line_height,
//         }
//     }

//     /// Measures the pixel width of text up to a given position (in characters)
//     pub fn measure_text_to_column(&self, text: &str, column: usize) -> f32 {
//         if text.is_empty() || column == 0 {
//             return 0.0;
//         }

//         // Take only the text up to the column
//         let text_slice: String = text.chars().take(column).collect();
        
//         let mut font_system = self.font_system.lock().unwrap();
//         let metrics = Metrics::new(self.font_size, self.line_height);
//         let mut buffer = Buffer::new(&mut *font_system, metrics);
        
//         buffer.set_size(&mut *font_system, Some(f32::MAX), Some(self.line_height));
//         buffer.set_text(
//             &mut *font_system,
//             &text_slice,
//             Attrs::new().family(Family::Monospace),
//             Shaping::Advanced,
//         );
        
//         buffer.shape_until_scroll(&mut *font_system, false);

//         // Calculate total width by adding glyph widths
//         let mut width = 0.0;
//         for run in buffer.layout_runs() {
//             for glyph in run.glyphs.iter() {
//                 width += glyph.w;
//             }
//         }

//         width
//     }

//     /// Finds the column (character position) corresponding to an X position in pixels
//     /// Uses a simple approach for monospace fonts
//     pub fn column_from_x(&self, text: &str, x: f32) -> usize {
//         if text.is_empty() || x <= 0.0 {
//             return 0;
//         }

//         // For a monospace font, we estimate the average width per character
//         // by measuring the total width of the text
//         let char_count = text.chars().count();
//         if char_count == 0 {
//             return 0;
//         }

//         // Measure total text width
//         let total_width = self.measure_text_to_column(text, char_count);
        
//         if total_width <= 0.0 {
//             // If we can't measure, use an approximation
//             // For 16px monospace, each character is approximately 9.6px
//             let approx_char_width = 9.6;
//             return (x / approx_char_width).round().min(char_count as f32) as usize;
//         }
        
//         // Calculate average character width
//         let avg_char_width = total_width / char_count as f32;
        
//         // Find column by testing each position
//         let mut best_col = 0;
//         let mut min_distance = f32::MAX;
        
//         for col in 0..=char_count {
//             let col_width = col as f32 * avg_char_width;
//             let distance = (col_width - x).abs();
            
//             if distance < min_distance {
//                 min_distance = distance;
//                 best_col = col;
//             } else {
//                 // If distance increases, we found the minimum
//                 break;
//             }
//         }
        
//         best_col
//     }

//     /// Measures the total width of a line of text
//     pub fn measure_text_width(&self, text: &str) -> f32 {
//         self.measure_text_to_column(text, text.chars().count())
//     }
// }
