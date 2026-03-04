use gpui::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::editor::editor_window::EditorWindow;

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
            .filter(|n| !n.name.starts_with('.'))
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
    pub dir_icon: Arc<Path>,
}

impl FileTree {
    pub fn new(_cx: &mut Context<EditorWindow>) -> Self {
        let root_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut root = FileNode::from_path(root_path.clone(), 0);
        root.is_expanded = true;
        root.load_children();

        let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
        let icon = |name: &str| -> Arc<Path> { Arc::from(assets_dir.join(name).as_path()) };

        let mut file_icons: std::collections::HashMap<String, Arc<Path>> = std::collections::HashMap::new();
        file_icons.insert("rs".into(),  icon("rust_logo.png"));
        file_icons.insert("c".into(),   icon("c_logo.png"));
        file_icons.insert("cpp".into(), icon("cpp_logo.png"));
        file_icons.insert("c++".into(), icon("cpp_logo.png"));
        file_icons.insert("cs".into(),  icon("c-sharp_logo.png"));

        let dir_icon: Arc<Path> = Arc::from(assets_dir.join("directory_logo.png").as_path());

        Self {
            root: Some(root),
            root_path,
            file_icons,
            dir_icon,
        }
    }

    #[allow(dead_code)]
    pub fn set_root(&mut self, path: PathBuf) {
        let mut root = FileNode::from_path(path.clone(), 0);
        root.is_expanded = true;
        root.load_children();
        self.root_path = path;
        self.root = Some(root);
    }

    /// Toggle a node by its path
    pub fn toggle_node(&mut self, path: &PathBuf) {
        if let Some(root) = &mut self.root {
            toggle_in_node(root, path);
        }
    }

    #[allow(dead_code)]
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
    cx: &mut Context<EditorWindow>,
) -> impl IntoElement + use<> {
    let flat = file_tree.flatten();
    let root_name = file_tree
        .root_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("EXPLORER")
        .to_string()
        .to_uppercase();

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
                .text_color(rgb(0xbbbbbb))
                .text_size(px(11.0))
                .font_weight(FontWeight::BOLD)
                .child(root_name),
        )
        .child(
            div()
                .flex_1()
                .overflow_hidden()
                .flex()
                .flex_col()
                .children(flat.into_iter().map(|node| {
                    let path = node.path.clone();
                    let indent = node.depth as f32 * 12.0 + 8.0;

                    let ext = node.name.rsplit('.').next().unwrap_or("").to_string();

                    let text_color = if node.is_dir {
                        rgb(0xcccccc)
                    } else {
                        rgb(0xffffff)
                    };

                    let arrow_el: Option<AnyElement> = if node.is_dir {
                        let arrow = if node.is_expanded { "▾" } else { "▸" };
                        Some(
                            div()
                                .text_size(px(13.0))
                                .text_color(rgb(0x888888))
                                .child(arrow)
                                .into_any_element(),
                        )
                    } else {
                        None
                    };

                    let icon_el: Option<AnyElement> = if node.is_dir {
                        Some(img(file_tree.dir_icon.clone()).size(px(14.0)).into_any_element())
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
                        .hover(|s| s.bg(rgb(0x2a2d2e)));

                    let row = if let Some(arrow) = arrow_el {
                        row.child(arrow)
                    } else {
                        row
                    };

                    let row = if let Some(icon) = icon_el {
                        row.child(icon)
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

                    row
                        .id(node_id)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _: &MouseDownEvent, _window, cx| {
                                if path.is_dir() {
                                    this.file_tree.toggle_node(&path);
                                    cx.notify();
                                } else {
                                    this.open_file_from_path(path.clone(), cx);
                                }
                            }),
                        )
                })),
        )
}