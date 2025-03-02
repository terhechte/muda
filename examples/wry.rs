// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![allow(unused)]
use muda::{
    accelerator::{Accelerator, Code, Modifiers},
    AboutMetadata, CheckMenuItem, ContextMenu, IconMenuItem, Menu, MenuEvent, MenuItem,
    PredefinedMenuItem, Submenu,
};
#[cfg(target_os = "macos")]
use tao::platform::macos::WindowExtMacOS;
#[cfg(target_os = "linux")]
use tao::platform::unix::WindowExtUnix;
#[cfg(target_os = "windows")]
use tao::platform::windows::{EventLoopBuilderExtWindows, WindowExtWindows};
use tao::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::{Window, WindowBuilder},
};
use wry::webview::WebViewBuilder;

fn main() -> wry::Result<()> {
    let mut event_loop_builder = EventLoopBuilder::new();

    let menu_bar = Menu::new();

    #[cfg(target_os = "windows")]
    {
        let menu_bar = menu_bar.clone();
        event_loop_builder.with_msg_hook(move |msg| {
            use windows_sys::Win32::UI::WindowsAndMessaging::{TranslateAcceleratorW, MSG};
            unsafe {
                let msg = msg as *const MSG;
                let translated = TranslateAcceleratorW((*msg).hwnd, menu_bar.haccel(), msg);
                translated == 1
            }
        });
    }

    let event_loop = event_loop_builder.build();

    let window = WindowBuilder::new()
        .with_title("Window 1")
        .build(&event_loop)
        .unwrap();

    let window2 = WindowBuilder::new()
        .with_title("Window 2")
        .build(&event_loop)
        .unwrap();
    let window2_id = window2.id();

    #[cfg(target_os = "macos")]
    {
        let app_m = Submenu::new("App", true);
        menu_bar.append(&app_m);
        app_m.append_items(&[
            &PredefinedMenuItem::about(None, None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::services(None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::hide(None),
            &PredefinedMenuItem::hide_others(None),
            &PredefinedMenuItem::show_all(None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::quit(None),
        ]);
    }

    let file_m = Submenu::new("&File", true);
    let edit_m = Submenu::new("&Edit", true);
    let window_m = Submenu::new("&Window", true);

    menu_bar.append_items(&[&file_m, &edit_m, &window_m]);

    let custom_i_1 = MenuItem::new(
        "C&ustom 1",
        true,
        Some(Accelerator::new(Some(Modifiers::ALT), Code::KeyC)),
    );

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/icon.png");
    let icon = load_icon(std::path::Path::new(path));
    let image_item = IconMenuItem::new(
        "Image custom 1",
        true,
        Some(icon),
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyC)),
    );

    let check_custom_i_1 = CheckMenuItem::new("Check Custom 1", true, true, None);
    let check_custom_i_2 = CheckMenuItem::new("Check Custom 2", false, true, None);
    let check_custom_i_3 = CheckMenuItem::new(
        "Check Custom 3",
        true,
        true,
        Some(Accelerator::new(Some(Modifiers::SHIFT), Code::KeyD)),
    );

    let copy_i = PredefinedMenuItem::copy(None);
    let cut_i = PredefinedMenuItem::cut(None);
    let paste_i = PredefinedMenuItem::paste(None);

    file_m.append_items(&[
        &custom_i_1,
        &image_item,
        &window_m,
        &PredefinedMenuItem::separator(),
        &check_custom_i_1,
        &check_custom_i_2,
    ]);

    window_m.append_items(&[
        &PredefinedMenuItem::minimize(None),
        &PredefinedMenuItem::maximize(None),
        &PredefinedMenuItem::close_window(Some("Close")),
        &PredefinedMenuItem::fullscreen(None),
        &PredefinedMenuItem::about(
            None,
            Some(AboutMetadata {
                name: Some("tao".to_string()),
                version: Some("1.2.3".to_string()),
                copyright: Some("Copyright tao".to_string()),
                ..Default::default()
            }),
        ),
        &check_custom_i_3,
        &image_item,
        &custom_i_1,
    ]);

    edit_m.append_items(&[&copy_i, &PredefinedMenuItem::separator(), &paste_i]);

    #[cfg(target_os = "windows")]
    {
        menu_bar.init_for_hwnd(window.hwnd() as _);
        menu_bar.init_for_hwnd(window2.hwnd() as _);
    }
    #[cfg(target_os = "linux")]
    {
        menu_bar.init_for_gtk_window(window.gtk_window(), window.default_vbox());
        menu_bar.init_for_gtk_window(window2.gtk_window(), window2.default_vbox());
    }
    #[cfg(target_os = "macos")]
    {
        menu_bar.init_for_nsapp();
        window_m.set_windows_menu_for_nsapp();
    }

    #[cfg(windows)]
    let condition = "e.button !== 2";
    #[cfg(not(windows))]
    let condition = "e.button == 2 && e.buttons === 0";
    let html: String = format!(
        r#"
    <html>
    <body>
        <style>
            main {{
                width: 100vw;
                height: 100vh;
            }}
        </style>
        <main>
            <h4> WRYYYYYYYYYYYYYYYYYYYYYY! </h4>
            <input />
            <button> Hi </button>
        </main>
        <script>
            window.addEventListener('contextmenu', (e) => {{
                e.preventDefault();
                console.log(e)
                // contextmenu was requested from keyboard
                if ({condition}) {{
                    window.ipc.postMessage(`showContextMenuPos:${{e.clientX}},${{e.clientY}}`);
                }}
            }})
            let x = true;
            window.addEventListener('mouseup', (e) => {{
                if (e.button === 2) {{
                    if (x) {{
                        window.ipc.postMessage(`showContextMenuPos:${{e.clientX}},${{e.clientY}}`);
                    }} else {{
                        window.ipc.postMessage(`showContextMenu`);
                    }}
                    x = !x;
                }}
            }})
        </script>
    </body>
    </html>
  "#,
    );

    let file_m_c = file_m.clone();
    let handler = move |window: &Window, req: String| {
        if &req == "showContextMenu" {
            show_context_menu(window, &file_m_c, None)
        } else if let Some(rest) = req.strip_prefix("showContextMenuPos:") {
            let (x, mut y) = rest
                .split_once(',')
                .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                .unwrap();

            #[cfg(target_os = "linux")]
            {
                if let Some(menu_bar) = menu_bar
                    .clone()
                    .gtk_menubar_for_gtk_window(window.gtk_window())
                {
                    use gtk::prelude::*;
                    y += menu_bar.allocated_height();
                }
            }

            show_context_menu(
                window,
                &file_m_c,
                Some(muda::Position::Logical((x, y).into())),
            )
        }
    };

    let webview = WebViewBuilder::new(window)?
        .with_html(&html)?
        .with_ipc_handler(handler.clone())
        .build()?;
    let webview2 = WebViewBuilder::new(window2)?
        .with_html(html)?
        .with_ipc_handler(handler)
        .build()?;

    let menu_channel = MenuEvent::receiver();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }

        if let Ok(event) = menu_channel.try_recv() {
            if event.id == custom_i_1.id() {
                custom_i_1
                    .set_accelerator(Some(Accelerator::new(Some(Modifiers::SHIFT), Code::KeyF)));
                file_m.insert(&MenuItem::new("New Menu Item", true, None), 2);
            }
            println!("{event:?}");
        }
    })
}

fn show_context_menu(window: &Window, menu: &dyn ContextMenu, position: Option<muda::Position>) {
    println!("Show context menu at position {position:?}");
    #[cfg(target_os = "windows")]
    menu.show_context_menu_for_hwnd(window.hwnd() as _, position);
    #[cfg(target_os = "linux")]
    menu.show_context_menu_for_gtk_window(window.gtk_window(), position);
    #[cfg(target_os = "macos")]
    menu.show_context_menu_for_nsview(window.ns_view() as _, position);
}

fn load_icon(path: &std::path::Path) -> muda::icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    muda::icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
