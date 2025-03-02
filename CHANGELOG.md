# Changelog

## \[0.7.0]

- [`ee30bf8`](https://www.github.com/tauri-apps/muda/commit/ee30bf8d29895c35d7cda0d67d9d64b71910380a)([#73](https://www.github.com/tauri-apps/muda/pull/73)) Added the `builders` which contains convenient builder types, like `AboutMetadataBuilder`, `MenuItemBuilder`, `SubmenuBuilder` ...etc.
- [`c7ec320`](https://www.github.com/tauri-apps/muda/commit/c7ec3207388947b5572847e589eb494d0222373d)([#78](https://www.github.com/tauri-apps/muda/pull/78)) **Breaking Change**: `ContextMenu::show_context_menu_for_hwnd`, `ContextMenu::show_context_menu_for_gtk_window` and `ContextMenu::show_context_menu_for_nsview` has been changed to take an optional `Into<Position>` type instead of `x` and `y`. if `None` is provided, it will use the current cursor position.
- [`98701d0`](https://www.github.com/tauri-apps/muda/commit/98701d0b3277dcb63ee50a8a11f5b008ed432307)([#75](https://www.github.com/tauri-apps/muda/pull/75)) **Breaking Change**: Changed `Menu::init_for_gtk_window` to accept a second argument for a container to which the menu bar should be added, if `None` was provided, it will add it to the window directly. The method will no longer create a `gtk::Box` and append it to the window, instead you should add the box to the window yourself, then pass a reference to it to the method so it can be used as the container for the menu bar.
- [`20c05ce`](https://www.github.com/tauri-apps/muda/commit/20c05ceae677338b2b9dbf247a86d4049280cc90)([#79](https://www.github.com/tauri-apps/muda/pull/79)) **Breaking Change**: Removed `MenuItemType` enum and replaced with `MenuItemKind` enum. `Menu::items` and `Submenu::items` methods will now return `Vec<MenuItemKind>` instead of `Vec<Box<dyn MenuItemExt>>`
- [`0000e56`](https://www.github.com/tauri-apps/muda/commit/0000e569746e7cb630a1453a401bf8f6b0568e9d)([#71](https://www.github.com/tauri-apps/muda/pull/71)) **Breaking Change**: Changed `MenuItemExt` trait name to `IsMenuItem`
- [`ee30bf8`](https://www.github.com/tauri-apps/muda/commit/ee30bf8d29895c35d7cda0d67d9d64b71910380a)([#73](https://www.github.com/tauri-apps/muda/pull/73)) Impl `TryFrom<&str>` and `TryFrom<String>` for `Accelerator`.

## \[0.6.0]

- [`ac14222`](https://www.github.com/tauri-apps/muda/commit/ac142229340c8ded63316fbc1cd1c11bf27e0890)([#69](https://www.github.com/tauri-apps/muda/pull/69)) Add `common-controls-v6` feature flag, disabled by default, which could be used to enable usage of `TaskDialogIndirect` API from `ComCtl32.dll` v6 on Windows for The predefined `About` menu item.
- [`7af4477`](https://www.github.com/tauri-apps/muda/commit/7af44778962de62bf6d8b05aab08bb2e689295fe)([#67](https://www.github.com/tauri-apps/muda/pull/67)) Add `libxdo` feature flag, enabled by default, to control whether to link `libxdo` on Linux or not.
- [`fabbbac`](https://www.github.com/tauri-apps/muda/commit/fabbbacb4b8d77c84cd318a21df1951063e7ea14)([#66](https://www.github.com/tauri-apps/muda/pull/66)) Add support for `AboutMetadata` on macOS

## \[0.5.0]

- Add `(MenuItem|CheckMenuItem|IconMenuItem)::set_accelerator` to change or disable the accelerator after creation.
  - [47ba0b4](https://www.github.com/tauri-apps/muda/commit/47ba0b47ed28a93428c253e8bac397e0b9cb8dec) feat: add `set_accelerator` ([#64](https://www.github.com/tauri-apps/muda/pull/64)) on 2023-05-04

## \[0.4.5]

- On Windows, fix panic when click a menu item while the `PredefinedMenuItem::about` dialog is open.
  - [f3883ee](https://www.github.com/tauri-apps/muda/commit/f3883ee2d4d8773e6b77e36700edb4ca7cb0988e) fix(windows): run the about dialog in its own thread, closes [#57](https://www.github.com/tauri-apps/muda/pull/57) ([#60](https://www.github.com/tauri-apps/muda/pull/60)) on 2023-03-27
- On Windows, Fix a panic when adding `CheckMenuItem` to a `Menu`.
  - [059fceb](https://www.github.com/tauri-apps/muda/commit/059fceb13007760d9e41b65068c91442eda64626) fix(windows): downcast check menu item correctly ([#58](https://www.github.com/tauri-apps/muda/pull/58)) on 2023-03-27

## \[0.4.4]

- On Windows, fix `MenuEvent` not triggered for `IconMenuItem`.
  - [88d3520](https://www.github.com/tauri-apps/muda/commit/88d352033ba571126a11bc681ee3b346b7579916) fix(Windows): dispatch menu event for icon menu item ([#53](https://www.github.com/tauri-apps/muda/pull/53)) on 2023-03-06
- On Windows, The `Close` predefined menu item will send `WM_CLOSE` to the window instead of calling `DestroyWindow` to let the developer catch this event and decide whether to close the window or not.
  - [f322ad4](https://www.github.com/tauri-apps/muda/commit/f322ad454dcd206e2802bb7c65f0a55616a8d002) fix(Windows): send `WM_CLOSE` instead of `DestroyWindow` ([#55](https://www.github.com/tauri-apps/muda/pull/55)) on 2023-03-06

## \[0.4.3]

- Implement `PredefinedMenuItemm::maximize` and `PredefinedMenuItemm::hide` on Windows.
  - [d2bd85b](https://www.github.com/tauri-apps/muda/commit/d2bd85bf7ec4b0bc974d487adaacb6a99b82fa91) docs: add docs for `PredefinedMenuItem` ([#51](https://www.github.com/tauri-apps/muda/pull/51)) on 2023-02-28
- Add docs for predefined menu items
  - [d2bd85b](https://www.github.com/tauri-apps/muda/commit/d2bd85bf7ec4b0bc974d487adaacb6a99b82fa91) docs: add docs for `PredefinedMenuItem` ([#51](https://www.github.com/tauri-apps/muda/pull/51)) on 2023-02-28

## \[0.4.2]

- Fix panic when updating a `CheckMenuItem` right after it was clicked.
  - [923af09](https://www.github.com/tauri-apps/muda/commit/923af09abfe885995ae0a4ef30f8a304cc4c20d2) fix(linux): fix multiple borrow panic ([#48](https://www.github.com/tauri-apps/muda/pull/48)) on 2023-02-14

## \[0.4.1]

- Update docs
  - [4b2ebc2](https://www.github.com/tauri-apps/muda/commit/4b2ebc247cfef64bcaab2ab619e30b65db37a72f) docs: update docs on 2023-02-08

## \[0.4.0]

- Bump gtk version: 0.15 -> 0.16
  - [fb3d0aa](https://www.github.com/tauri-apps/muda/commit/fb3d0aa303a0ee4ffff6d3de97cc363f1ef6d34b) chore(deps): bump gtk version 0.15 -> 0.16 ([#38](https://www.github.com/tauri-apps/muda/pull/38)) on 2023-01-26

## \[0.3.0]

- Add `MenuEvent::set_event_handler` to set a handler for new menu events.
  - [f871c68](https://www.github.com/tauri-apps/muda/commit/f871c68e81aa10f9541c386615a05a2e455e5a82) refactor: allow changing the menu event sender ([#35](https://www.github.com/tauri-apps/muda/pull/35)) on 2023-01-03
- **Breaking change** Remove `menu_event_receiver` function, use `MenuEvent::receiver` instead.
  - [f871c68](https://www.github.com/tauri-apps/muda/commit/f871c68e81aa10f9541c386615a05a2e455e5a82) refactor: allow changing the menu event sender ([#35](https://www.github.com/tauri-apps/muda/pull/35)) on 2023-01-03

## \[0.2.0]

- Add `IconMenuItem`
  - [7fc1b02](https://www.github.com/tauri-apps/muda/commit/7fc1b02cac65f2524220cb79263643505e286863) feat: add `IconMenuItem`, closes [#30](https://www.github.com/tauri-apps/muda/pull/30) ([#32](https://www.github.com/tauri-apps/muda/pull/32)) on 2022-12-30

## \[0.1.1]

- Derive `Copy` for `Accelerator` type.
  - [e80c113](https://www.github.com/tauri-apps/muda/commit/e80c113d8c8db8137f97829b071b443772d4805c) feat: derive `Copy` for `Accelerator` on 2022-12-12
- Fix parsing one letter string as valid accelerator without modifiers.
  - [0173987](https://www.github.com/tauri-apps/muda/commit/0173987ed5da605ddc20e49fce57ba884ed0d5f4) fix: parse one letter string to valid accelerator ([#28](https://www.github.com/tauri-apps/muda/pull/28)) on 2022-12-20

## \[0.1.0]

- Initial Release.
  - [0309d10](https://www.github.com/tauri-apps/muda/commit/0309d101b16663ce67b518f8aa1d2c4af0de6dee) chore: prepare for first release on 2022-12-05
