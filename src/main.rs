use std::{sync::Arc, thread, time::Duration};

use lapce_core::app;
use lapce_core::palette::Palette;
use lapce_core::split::LapceSplit;
use lapce_core::theme::LapceTheme;
use lapce_core::{container::LapceContainer, state::hex_to_color};
use lapce_core::{editor::Editor, window::LapceTab, window::LapceWindow};

use druid::{
    piet::Color, theme, FontDescriptor, FontFamily, FontWeight, Key, MenuDesc, Size,
    Target, WidgetId,
};
use druid::{
    widget::IdentityWrapper,
    widget::{Align, Container, Flex, Label, Padding, Scroll, Split},
    AppDelegate, Command, DelegateCtx, Env, Point, WindowId,
};
use druid::{AppLauncher, LocalizedString, Widget, WidgetExt, WindowDesc};
use lapce_core::command::{LapceUICommand, LAPCE_UI_COMMAND};
use lapce_core::state::{
    LapceTabState, LapceUIState, LapceWindowState, LAPCE_APP_STATE,
};
// use tree_sitter::{Language, Parser};

// extern "C" {
//     fn tree_sitter_rust() -> Language;
// }

struct Delegate {
    windows: Vec<WindowId>,
}

impl Delegate {
    pub fn new() -> Delegate {
        Delegate {
            windows: Vec::new(),
        }
    }
}

impl AppDelegate<LapceUIState> for Delegate {
    fn window_added(
        &mut self,
        id: WindowId,
        data: &mut LapceUIState,
        env: &Env,
        ctx: &mut DelegateCtx,
    ) {
    }

    fn window_removed(
        &mut self,
        id: WindowId,
        data: &mut LapceUIState,
        env: &Env,
        ctx: &mut DelegateCtx,
    ) {
    }
}

fn build_app(window_id: WindowId) -> impl Widget<LapceUIState> {
    let window = LapceWindow::new(window_id);
    window.env_scope(|env: &mut druid::Env, data: &LapceUIState| {
        let theme = &LAPCE_APP_STATE.theme;
        if let Some(line_highlight) = theme.get("line_highlight") {
            env.set(
                LapceTheme::EDITOR_CURRENT_LINE_BACKGROUND,
                line_highlight.clone(),
            );
        };
        if let Some(caret) = theme.get("caret") {
            env.set(LapceTheme::EDITOR_CURSOR_COLOR, caret.clone());
        };
        if let Some(foreground) = theme.get("foreground") {
            env.set(LapceTheme::EDITOR_FOREGROUND, foreground.clone());
        };
        if let Some(background) = theme.get("background") {
            env.set(LapceTheme::EDITOR_BACKGROUND, background.clone());
        };
        if let Some(selection) = theme.get("selection") {
            env.set(LapceTheme::EDITOR_SELECTION_COLOR, selection.clone());
        };
        if let Some(color) = theme.get("comment") {
            env.set(LapceTheme::EDITOR_COMMENT, color.clone());
        };
        if let Some(color) = theme.get("error") {
            env.set(LapceTheme::EDITOR_ERROR, color.clone());
        };
        if let Some(color) = theme.get("warn") {
            env.set(LapceTheme::EDITOR_WARN, color.clone());
        };
        env.set(LapceTheme::EDITOR_LINE_HEIGHT, 25.0);
        env.set(LapceTheme::PALETTE_BACKGROUND, Color::rgb8(125, 125, 125));
        env.set(LapceTheme::PALETTE_INPUT_FOREROUND, Color::rgb8(0, 0, 0));
        env.set(
            LapceTheme::PALETTE_INPUT_BACKGROUND,
            Color::rgb8(255, 255, 255),
        );
        env.set(LapceTheme::PALETTE_INPUT_BORDER, Color::rgb8(0, 0, 0));
        env.set(
            LapceTheme::EDITOR_FONT,
            FontDescriptor::new(FontFamily::new_unchecked("Cascadia Code"))
                .with_size(13.0),
        );
        env.set(theme::SCROLLBAR_COLOR, hex_to_color("#c4c4c4").unwrap());
    })
    // .debug_invalidation()
}

pub fn main() {
    app::lanuch();
}

pub fn old_main() {
    {
        // only for #[cfg]
        use parking_lot::deadlock;
        use std::thread;
        use std::time::Duration;

        // Create a background thread which checks for deadlocks every 10s
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(10));
            let deadlocks = deadlock::check_deadlock();
            if deadlocks.is_empty() {
                continue;
            }

            println!("{} deadlocks detected", deadlocks.len());
            for (i, threads) in deadlocks.iter().enumerate() {
                println!("Deadlock #{}", i);
                for t in threads {
                    println!("Thread Id {:#?}", t.thread_id());
                    println!("{:#?}", t.backtrace());
                }
            }
        });
    }
    // WindowDesc::new(|| LapceContainer::new());
    let window_state = LapceWindowState::new();
    let window_id = window_state.window_id.clone();
    LAPCE_APP_STATE
        .states
        .lock()
        .insert(window_id.clone(), window_state);
    let mut window = WindowDesc::new(move || build_app(window_id))
        .title(LocalizedString::new("lapce").with_placeholder("Lapce"))
        .menu(MenuDesc::empty())
        .window_size(Size::new(800.0, 600.0))
        .with_min_size(Size::new(800.0, 600.0));
    window.id = window_id;

    // thread::spawn(move || {
    //     ui_event_sink.submit_command(
    //         LAPCE_UI_COMMAND,
    //         LapceUICommand::OpenFile(
    //             "/Users/Lulu/go/src/uni/main.go".to_string(),
    //             // "/Users/Lulu/lapce/core/src/editor.rs".to_string(),
    //         ),
    //         Target::Global,
    //     );
    // });
    //let mut parser = Parser::new();
    //let language = unsafe { tree_sitter_rust() };
    //parser.set_language(language);
    //parser.parse("pub fn main() {}", None).unwrap();
    let launcher = AppLauncher::with_window(window);
    LAPCE_APP_STATE.set_ui_sink(launcher.get_external_handle());
    let ui_event_sink = launcher.get_external_handle();
    let ui_state = LapceUIState::new(ui_event_sink);
    let delegate = Delegate::new();
    launcher
        .delegate(delegate)
        .use_simple_logger()
        .launch(ui_state)
        .expect("launch failed");
}
