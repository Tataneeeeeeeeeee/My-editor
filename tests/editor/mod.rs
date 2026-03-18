// Module aggregating all editor-related tests
mod key;
mod log;
mod menu_bar;
mod tab_bar;
mod tool_bar;

#[path = "text_buffer_tests.rs"]
mod text_buffer_tests;

#[path = "editor_element_tests.rs"]
mod editor_element_tests;

#[path = "editor_window_tests.rs"]
mod editor_window_tests;

#[path = "syntax_highlighter_tests.rs"]
mod syntax_highlighter_tests;
