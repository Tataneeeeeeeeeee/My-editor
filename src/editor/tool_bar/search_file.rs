use gpui::*;
use gpui::prelude::FluentBuilder;
use std::path::PathBuf;
use crate::editor::editor_window::EditorWindow;
use super::text_input::{TextInputState, render_text_input_box};

/// One occurrence of the query found in a file
#[derive(Clone, Debug)]
pub struct SearchResult {
    pub file_name: String,
    pub file_path: PathBuf,
    pub line_number: usize,
    pub line_content: String,
}

/// Walk `root_dir` recursively and collect every line that contains `query`.
/// Skips binary files, the `.git` directory and the `target` directory.
pub fn search_in_files(query: &str, root_dir: &PathBuf) -> Vec<SearchResult> {
    let mut results = Vec::new();
    if query.is_empty() {
        return results;
    }
    walk_dir(root_dir, query, &mut results);
    results
}

fn walk_dir(dir: &PathBuf, query: &str, out: &mut Vec<SearchResult>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        // Skip hidden dirs, .git, target
        if name.starts_with('.') || name == "target" {
            continue;
        }

        if path.is_dir() {
            walk_dir(&path, query, out);
        } else {
            search_in_file(&path, query, out);
        }
    }
}

fn search_in_file(path: &PathBuf, query: &str, out: &mut Vec<SearchResult>) {
    let Ok(content) = std::fs::read_to_string(path) else { return };
    let query_lower = query.to_lowercase();
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    for (i, line) in content.lines().enumerate() {
        if line.to_lowercase().contains(&query_lower) {
            out.push(SearchResult {
                file_name: file_name.clone(),
                file_path: path.clone(),
                line_number: i + 1,
                line_content: line.trim().to_string(),
            });
        }
    }
}

pub fn render_search_files(
    search_input_state: &TextInputState,
    results: &[SearchResult],
    cx: &mut Context<EditorWindow>,
) -> impl IntoElement + use<> {
    let result_count = results.len();
    let query_len = search_input_state.input.chars().count();
    let results_clone: Vec<SearchResult> = results.to_vec();

    div()
        .w(px(240.0))
        .h_full()
        .bg(rgb(0x252526))
        .border_r_1()
        .border_color(rgb(0x1e1e1e))
        .flex()
        .flex_col()
        .overflow_hidden()
        .child(
            div()
                .px(px(12.0))
                .py(px(8.0))
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x888888))
                        .child("SEARCH")
                )
                .when(result_count > 0, |el| {
                    el.child(
                        div()
                            .text_size(px(10.0))
                            .text_color(rgb(0x888888))
                            .child(format!("{} résultat(s)", result_count))
                    )
                })
        )
        .child(
            div()
                .px(px(8.0))
                .py(px(4.0))
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(render_text_input_box(search_input_state))
        )
        .child(
            div()
                .id("search-results-scroll")
                .flex_1()
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .when(query_len == 0, |el| {
                    el.child(
                        div()
                            .px(px(12.0))
                            .py(px(8.0))
                            .text_size(px(12.0))
                            .text_color(rgb(0x555555))
                            .child("Tapez un mot à rechercher.")
                    )
                })
                .when(query_len > 0 && query_len < 4, |el| {
                    let remaining = 4 - query_len;
                    el.child(
                        div()
                            .px(px(12.0))
                            .py(px(8.0))
                            .text_size(px(12.0))
                            .text_color(rgb(0x888888))
                            .child(format!("Encore {} caractère(s)…", remaining))
                    )
                })
                .when(query_len >= 4 && result_count == 0, |el| {
                    el.child(
                        div()
                            .px(px(12.0))
                            .py(px(8.0))
                            .text_size(px(12.0))
                            .text_color(rgb(0x888888))
                            .child("Aucun résultat.")
                    )
                })
                .when (query_len >= 4 && result_count > 0, |el| {
                    el.children(results_clone.into_iter().enumerate().map(|(idx, result)| {
                        let path = result.file_path.clone();
                        let row_id = SharedString::from(format!("sr-{}", idx));

                        div()
                            .id(row_id)
                            .px(px(8.0))
                            .py(px(4.0))
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0x2a2d2e)))
                            .border_b_1()
                            .border_color(rgb(0x1e1e1e))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(rgb(0x4ec9b0))
                                            .font_weight(FontWeight::BOLD)
                                            .font_family("monospace")
                                            .child(result.file_name.clone())
                                    )
                                    .child(
                                        div()
                                            .text_size(px(10.0))
                                            .text_color(rgb(0x888888))
                                            .child(format!(":{}", result.line_number))
                                    )
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(0xbbbbbb))
                                    .font_family("monospace")
                                    .overflow_hidden()
                                    .child({
                                        let preview: String = result.line_content
                                            .chars()
                                            .take(30)
                                            .collect();
                                        if result.line_content.chars().count() > 30 {
                                            format!("{}…", preview)
                                        } else {
                                            preview
                                        }
                                    })
                            )
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _: &MouseDownEvent, _window, cx| {
                                    this.open_file_from_path(path.clone(), cx);
                                }),
                            )
                    }))
                })
        )
}