use super::text_input::{TextInputState, TextInputType, render_text_input_section};
use crate::editor::editor_window::{EditorWindow, PendingCreation, PendingCreationKind};
use crate::settings::settings::SettingsGlobal;
use gpui::prelude::FluentBuilder;
use gpui::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub depth: usize,
    pub children: Vec<FileNode>,
}

impl FileNode {
    pub fn from_path(path: PathBuf, depth: usize) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("?")
            .to_string();
        let is_dir = path.is_dir();
        Self {
            name,
            path,
            is_dir,
            is_expanded: false,
            depth,
            children: vec![],
        }
    }

    pub fn load_children(&mut self) {
        if !self.is_dir {
            return;
        }
        let Ok(entries) = std::fs::read_dir(&self.path) else {
            return;
        };
        let mut children: Vec<FileNode> = entries
            .filter_map(|e| e.ok())
            .map(|e| FileNode::from_path(e.path(), self.depth + 1))
            .filter(|n| n.name != ".git") // Ignore .git directories
            .collect();

        children.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });
        self.children = children;
    }

    pub fn toggle_expand(&mut self) {
        if !self.is_dir {
            return;
        }
        self.is_expanded = !self.is_expanded;
        if self.is_expanded && self.children.is_empty() {
            self.load_children();
        }
    }

    fn collect_flat(&self, out: &mut Vec<FlatNode>) {
        out.push(FlatNode {
            name: self.name.clone(),
            path: self.path.clone(),
            is_dir: self.is_dir,
            is_expanded: self.is_expanded,
            depth: self.depth,
        });
        if self.is_dir && self.is_expanded {
            for child in &self.children {
                child.collect_flat(out);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct FlatNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub depth: usize,
}

pub struct FileTree {
    pub root: Option<FileNode>,
    pub root_path: PathBuf,
    pub file_icons: std::collections::HashMap<String, Arc<Path>>,
    pub explorer_icon: std::collections::HashMap<String, Arc<Path>>,
    pub dir_icon: Arc<Path>,
    pub last_selected: PathBuf,
}

impl FileTree {
    pub fn new(root_dir: PathBuf, _cx: &mut Context<EditorWindow>) -> Self {
        let root_path = root_dir;
        let mut root = FileNode::from_path(root_path.clone(), 0);
        root.is_expanded = true;
        root.load_children();

        let settings_global = _cx.global::<SettingsGlobal>();
        let assets_dir = PathBuf::from(
            settings_global
                .get(vec!["assets", "path"])
                .expect("Failed to get assets path setting"),
        );
        let icon = |name: &str| -> Arc<Path> { Arc::from(assets_dir.join(name).as_path()) };

        let mut file_icons: std::collections::HashMap<String, Arc<Path>> =
            std::collections::HashMap::new();

        match settings_global.get(vec!["file_extensions"]) {
            Ok(exts_json) => {
                let exts: serde_json::Value = serde_json::from_str(&exts_json).unwrap_or_default();

                if let Some(ext_map) = exts.as_object() {
                    for (ext, info) in ext_map {
                        if let Some(icon_name) = info.get("icon").and_then(|v| v.as_str()) {
                            file_icons.insert(ext.clone(), icon(icon_name));
                        }
                    }
                }
            }
            Err(_) => {} // If the setting is missing or invalid, just leave file_icons empty
        }

        let mut explorer_icon: std::collections::HashMap<String, Arc<Path>> =
            std::collections::HashMap::new();
        explorer_icon.insert(
            "new_document".to_string(),
            icon(
                settings_global
                    .get(vec!["icons", "explorer_icons", "new_document"])
                    .unwrap_or_else(|_| "".to_string())
                    .as_str(),
            ),
        );
        explorer_icon.insert(
            "new_folder".to_string(),
            icon(
                settings_global
                    .get(vec!["icons", "explorer_icons", "new_folder"])
                    .unwrap_or_else(|_| "".to_string())
                    .as_str(),
            ),
        );

        let dir_icon: Arc<Path> = icon(
            settings_global
                .get(vec!["icons", "default", "directory"])
                .unwrap_or_else(|_| "".to_string())
                .as_str(),
        );

        Self {
            root: Some(root),
            last_selected: root_path.clone(),
            root_path,
            file_icons,
            explorer_icon,
            dir_icon,
        }
    }

    /// Toggle a node by its path
    pub fn toggle_node(&mut self, path: &PathBuf) {
        if let Some(root) = &mut self.root {
            toggle_in_node(root, path);
        }
    }

    pub fn flatten(&self) -> Vec<FlatNode> {
        match &self.root {
            Some(root) => {
                // Don't render the root itself, only its children
                let mut result = vec![];
                for child in &root.children {
                    child.collect_flat(&mut result);
                }
                result
            }
            None => vec![],
        }
    }

    /// Reload the tree from disk (keeping expanded state where possible)
    pub fn refresh(&mut self) {
        let mut root = FileNode::from_path(self.root_path.clone(), 0);
        root.is_expanded = true;
        root.load_children();
        self.root = Some(root);
    }
}

fn toggle_in_node(node: &mut FileNode, target: &PathBuf) {
    if &node.path == target {
        node.toggle_expand();
        return;
    }
    for child in &mut node.children {
        toggle_in_node(child, target);
    }
}

pub fn render_file_tree(
    file_tree: &FileTree,
    pending_creation: Option<&PendingCreation>,
    cx: &mut Context<EditorWindow>,
) -> impl IntoElement + use<> {
    let settings_global = cx.global::<SettingsGlobal>().clone();

    let explorer_bg = settings_global
        .get_color(vec!["ui", "panels", "explorer", "background"])
        .unwrap_or(0x252526);
    let explorer_border = settings_global
        .get_color(vec!["ui", "panels", "explorer", "border_color"])
        .unwrap_or(0x1e1e1e);
    let explorer_text = settings_global
        .get_color(vec!["ui", "panels", "explorer", "text_color"])
        .unwrap_or(0xbbbbbb);
    let explorer_header_text = settings_global
        .get_color(vec!["ui", "panels", "explorer", "header_text_color"])
        .unwrap_or(0x888888);
    let explorer_hover_bg = settings_global
        .get_color(vec!["ui", "panels", "explorer", "hover_background"])
        .unwrap_or(0x2a2d2e);
    let text_primary = settings_global
        .get_color(vec!["ui", "colors", "text_primary"])
        .unwrap_or(0xffffff);

    let flat = file_tree.flatten();
    let root_name = file_tree
        .root_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("EXPLORER")
        .to_string()
        .to_uppercase();

    // Convert pending_creation to TextInputState for reusable rendering
    let input_state: Option<TextInputState> = pending_creation.map(|pc| {
        let input_type = if pc.kind == PendingCreationKind::File {
            TextInputType::CreateFile
        } else {
            TextInputType::CreateFolder
        };
        let mut state = TextInputState::new(input_type);
        state.input = pc.input.clone();
        state
    });

    div()
        .w(px(240.0))
        .h_full()
        .bg(rgb(explorer_bg))
        .border_r_1()
        .border_color(rgb(explorer_border))
        .flex()
        .flex_col()
        .overflow_hidden()
        .child(
            div()
                .px(px(12.0))
                .py(px(8.0))
                .text_color(rgb(explorer_text))
                .text_size(px(11.0))
                .font_weight(FontWeight::BOLD)
                .child(
                    div()
                        .id("explorer-header")
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(rgb(explorer_header_text))
                                .child(root_name),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(4.0))
                                .child(
                                    img(file_tree
                                        .explorer_icon
                                        .get("new_document")
                                        .unwrap()
                                        .clone())
                                    .size(px(16.0))
                                    .id("new-file-btn")
                                    .cursor_pointer()
                                    .rounded(px(4.0))
                                    .hover(|s| s.bg(rgb(explorer_hover_bg)))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _: &MouseDownEvent, _window, cx| {
                                            this.start_create_file(cx);
                                        }),
                                    ),
                                )
                                .child(
                                    img(file_tree.explorer_icon.get("new_folder").unwrap().clone())
                                        .size(px(16.0))
                                        .id("new-folder-btn")
                                        .cursor_pointer()
                                        .rounded(px(4.0))
                                        .hover(|s| s.bg(rgb(explorer_hover_bg)))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _: &MouseDownEvent, _window, cx| {
                                                this.start_create_folder(cx);
                                            }),
                                        ),
                                ),
                        ),
                ),
        )
        .child(
            div()
                .id("file-tree-scroll")
                .flex_1()
                .overflow_y_scroll()
                .flex()
                .flex_col()
                // Inline input row using the reusable TextInputState system
                .when_some(input_state, |el, state| {
                    el.child(render_text_input_section(&state))
                })
                // File tree rows
                .children(flat.into_iter().map(|node| {
                    let path = node.path.clone();
                    let indent = node.depth as f32 * 12.0 + 8.0;
                    let ext = node.name.rsplit('.').next().unwrap_or("").to_string();

                    let text_color = if node.is_dir {
                        rgb(explorer_text)
                    } else {
                        rgb(text_primary)
                    };

                    let arrow_el: Option<AnyElement> = if node.is_dir {
                        let arrow = if node.is_expanded { "▾" } else { "▸" };
                        Some(
                            div()
                                .text_size(px(13.0))
                                .text_color(rgb(explorer_header_text))
                                .child(arrow)
                                .into_any_element(),
                        )
                    } else {
                        None
                    };

                    let icon_el: Option<AnyElement> = if node.is_dir {
                        Some(
                            img(file_tree.dir_icon.clone())
                                .size(px(14.0))
                                .into_any_element(),
                        )
                    } else if let Some(asset) = file_tree.file_icons.get(&ext).cloned() {
                        Some(img(asset).size(px(14.0)).into_any_element())
                    } else {
                        None
                    };

                    let node_id = SharedString::from(path.to_string_lossy().to_string());

                    let row = div()
                        .pl(px(indent))
                        .pr(px(8.0))
                        .h(px(22.0))
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(explorer_hover_bg)));

                    let row = if let Some(a) = arrow_el {
                        row.child(a)
                    } else {
                        row
                    };
                    let row = if let Some(i) = icon_el {
                        row.child(i)
                    } else {
                        row
                    };

                    let row = row.child(
                        div()
                            .text_size(px(13.0))
                            .text_color(text_color)
                            .font_family("monospace")
                            .child(node.name.clone()),
                    );

                    row.id(node_id).on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _: &MouseDownEvent, _window, cx| {
                            if path.is_dir() {
                                this.file_tree.toggle_node(&path);
                                this.file_tree.last_selected = path.clone();
                                cx.notify();
                            } else {
                                this.open_file_from_path(path.clone(), cx);
                            }
                        }),
                    )
                })),
        )
}
