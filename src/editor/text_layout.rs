// use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping};
// use std::sync::{Arc, Mutex};

// /// Gestionnaire de mesure de texte utilisant cosmic-text
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

//     /// Mesure la largeur en pixels d'un texte jusqu'à une position donnée (en caractères)
//     pub fn measure_text_to_column(&self, text: &str, column: usize) -> f32 {
//         if text.is_empty() || column == 0 {
//             return 0.0;
//         }

//         // Prendre seulement le texte jusqu'à la colonne
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

//         // Calculer la largeur totale en additionnant les largeurs des glyphes
//         let mut width = 0.0;
//         for run in buffer.layout_runs() {
//             for glyph in run.glyphs.iter() {
//                 width += glyph.w;
//             }
//         }

//         width
//     }

//     /// Trouve la colonne (position en caractères) correspondant à une position X en pixels
//     /// Utilise une approche simple pour les polices monospace
//     pub fn column_from_x(&self, text: &str, x: f32) -> usize {
//         if text.is_empty() || x <= 0.0 {
//             return 0;
//         }

//         // Pour une police monospace, on estime la largeur moyenne par caractère
//         // en mesurant la largeur totale du texte
//         let char_count = text.chars().count();
//         if char_count == 0 {
//             return 0;
//         }

//         // Mesurer la largeur totale du texte
//         let total_width = self.measure_text_to_column(text, char_count);
        
//         if total_width <= 0.0 {
//             // Si on ne peut pas mesurer, utiliser une approximation
//             // Pour monospace 16px, chaque caractère fait environ 9.6px
//             let approx_char_width = 9.6;
//             return (x / approx_char_width).round().min(char_count as f32) as usize;
//         }
        
//         // Calculer la largeur moyenne par caractère
//         let avg_char_width = total_width / char_count as f32;
        
//         // Trouver la colonne en testant chaque position
//         let mut best_col = 0;
//         let mut min_distance = f32::MAX;
        
//         for col in 0..=char_count {
//             let col_width = col as f32 * avg_char_width;
//             let distance = (col_width - x).abs();
            
//             if distance < min_distance {
//                 min_distance = distance;
//                 best_col = col;
//             } else {
//                 // Si la distance augmente, on a trouvé le minimum
//                 break;
//             }
//         }
        
//         best_col
//     }

//     /// Mesure la largeur totale d'une ligne de texte
//     pub fn measure_text_width(&self, text: &str) -> f32 {
//         self.measure_text_to_column(text, text.chars().count())
//     }
// }
