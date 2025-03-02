// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{cell::RefCell, rc::Rc};

use crate::{util::AddOp, ContextMenu, IsMenuItem, MenuItemKind, Position};

/// A root menu that can be added to a Window on Windows and Linux
/// and used as the app global menu on macOS.
#[derive(Clone)]
pub struct Menu(Rc<RefCell<crate::platform_impl::Menu>>);

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

impl Menu {
    /// Creates a new menu.
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(crate::platform_impl::Menu::new())))
    }

    /// Creates a new menu with given `items`. It calls [`Menu::new`] and [`Menu::append_items`] internally.
    pub fn with_items(items: &[&dyn IsMenuItem]) -> crate::Result<Self> {
        let menu = Self::new();
        menu.append_items(items)?;
        Ok(menu)
    }

    /// Returns a unique identifier associated with this menu.
    pub fn id(&self) -> u32 {
        self.0.borrow().id()
    }

    /// Add a menu item to the end of this menu.
    ///
    /// ## Platform-spcific:
    ///
    /// - **macOS:** Only [`Submenu`] can be added to the menu
    ///
    /// [`Submenu`]: crate::Submenu
    pub fn append(&self, item: &dyn IsMenuItem) -> crate::Result<()> {
        self.0.borrow_mut().add_menu_item(item, AddOp::Append)
    }

    /// Add menu items to the end of this menu. It calls [`Menu::append`] in a loop internally.
    ///
    /// ## Platform-spcific:
    ///
    /// - **macOS:** Only [`Submenu`] can be added to the menu
    ///
    /// [`Submenu`]: crate::Submenu
    pub fn append_items(&self, items: &[&dyn IsMenuItem]) -> crate::Result<()> {
        for item in items {
            self.append(*item)?
        }

        Ok(())
    }

    /// Add a menu item to the beginning of this menu.
    ///
    /// ## Platform-spcific:
    ///
    /// - **macOS:** Only [`Submenu`] can be added to the menu
    ///
    /// [`Submenu`]: crate::Submenu
    pub fn prepend(&self, item: &dyn IsMenuItem) -> crate::Result<()> {
        self.0.borrow_mut().add_menu_item(item, AddOp::Insert(0))
    }

    /// Add menu items to the beginning of this menu. It calls [`Menu::insert_items`] with position of `0` internally.
    ///
    /// ## Platform-spcific:
    ///
    /// - **macOS:** Only [`Submenu`] can be added to the menu
    ///
    /// [`Submenu`]: crate::Submenu
    pub fn prepend_items(&self, items: &[&dyn IsMenuItem]) -> crate::Result<()> {
        self.insert_items(items, 0)
    }

    /// Insert a menu item at the specified `postion` in the menu.
    ///
    /// ## Platform-spcific:
    ///
    /// - **macOS:** Only [`Submenu`] can be added to the menu
    ///
    /// [`Submenu`]: crate::Submenu
    pub fn insert(&self, item: &dyn IsMenuItem, position: usize) -> crate::Result<()> {
        self.0
            .borrow_mut()
            .add_menu_item(item, AddOp::Insert(position))
    }

    /// Insert menu items at the specified `postion` in the menu.
    ///
    /// ## Platform-spcific:
    ///
    /// - **macOS:** Only [`Submenu`] can be added to the menu
    ///
    /// [`Submenu`]: crate::Submenu
    pub fn insert_items(&self, items: &[&dyn IsMenuItem], position: usize) -> crate::Result<()> {
        for (i, item) in items.iter().enumerate() {
            self.insert(*item, position + i)?
        }

        Ok(())
    }

    /// Remove a menu item from this menu.
    pub fn remove(&self, item: &dyn IsMenuItem) -> crate::Result<()> {
        self.0.borrow_mut().remove(item)
    }

    /// Returns a list of menu items that has been added to this menu.
    pub fn items(&self) -> Vec<MenuItemKind> {
        self.0.borrow().items()
    }

    /// Adds this menu to a [`gtk::ApplicationWindow`]
    ///
    /// - `container`: this is an optional paramter to specify a container for the [`gtk::MenuBar`],
    /// it is highly recommended to pass a container, otherwise the menubar will be added directly to the window,
    /// which is usually not the desired behavior.
    ///
    /// ## Example:
    /// ```no_run
    /// let window = gtk::ApplicationWindow::builder().build();
    /// let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    /// let menu = muda::Menu::new();
    /// // -- snip, add your menu items --
    /// menu.init_for_gtk_window(&window, Some(&vbox));
    /// // then proceed to add your widgets to the `vbox`
    /// ```
    ///
    /// ## Panics:
    ///
    /// Panics if the gtk event loop hasn't been initialized on the thread.
    #[cfg(target_os = "linux")]
    pub fn init_for_gtk_window<W, C>(&self, window: &W, container: Option<&C>) -> crate::Result<()>
    where
        W: gtk::prelude::IsA<gtk::ApplicationWindow>,
        W: gtk::prelude::IsA<gtk::Window>,
        W: gtk::prelude::IsA<gtk::Container>,
        C: gtk::prelude::IsA<gtk::Container>,
    {
        self.0.borrow_mut().init_for_gtk_window(window, container)
    }

    /// Adds this menu to a win32 window.
    ///
    /// ##  Note about accelerators:
    ///
    /// For accelerators to work, the event loop needs to call
    /// [`TranslateAcceleratorW`](windows_sys::Win32::UI::WindowsAndMessaging::TranslateAcceleratorW)
    /// with the [`HACCEL`](windows_sys::Win32::UI::WindowsAndMessaging::HACCEL) returned from [`Menu::haccel`]
    ///
    /// #### Example:
    /// ```no_run
    /// # use muda::Menu;
    /// # use windows_sys::Win32::UI::WindowsAndMessaging::{MSG, GetMessageW, TranslateMessage, DispatchMessageW, TranslateAcceleratorW};
    /// let menu = Menu::new();
    /// unsafe {
    ///     let mut msg: MSG = std::mem::zeroed();
    ///     while GetMessageW(&mut msg, 0, 0, 0) == 1 {
    ///         let translated = TranslateAcceleratorW(msg.hwnd, menu.haccel(), &msg as *const _);
    ///         if translated != 1{
    ///             TranslateMessage(&msg);
    ///             DispatchMessageW(&msg);
    ///         }
    ///     }
    /// }
    /// ```
    #[cfg(target_os = "windows")]
    pub fn init_for_hwnd(&self, hwnd: isize) -> crate::Result<()> {
        self.0.borrow_mut().init_for_hwnd(hwnd)
    }

    /// Returns The [`HACCEL`](windows_sys::Win32::UI::WindowsAndMessaging::HACCEL) associated with this menu
    /// It can be used with [`TranslateAcceleratorW`](windows_sys::Win32::UI::WindowsAndMessaging::TranslateAcceleratorW)
    /// in the event loop to enable accelerators
    #[cfg(target_os = "windows")]
    pub fn haccel(&self) -> windows_sys::Win32::UI::WindowsAndMessaging::HACCEL {
        self.0.borrow_mut().haccel()
    }

    /// Removes this menu from a [`gtk::ApplicationWindow`]
    #[cfg(target_os = "linux")]
    pub fn remove_for_gtk_window<W>(&self, window: &W) -> crate::Result<()>
    where
        W: gtk::prelude::IsA<gtk::ApplicationWindow>,
        W: gtk::prelude::IsA<gtk::Window>,
    {
        self.0.borrow_mut().remove_for_gtk_window(window)
    }

    /// Removes this menu from a win32 window
    #[cfg(target_os = "windows")]
    pub fn remove_for_hwnd(&self, hwnd: isize) -> crate::Result<()> {
        self.0.borrow_mut().remove_for_hwnd(hwnd)
    }

    /// Hides this menu from a [`gtk::ApplicationWindow`]
    #[cfg(target_os = "linux")]
    pub fn hide_for_gtk_window<W>(&self, window: &W) -> crate::Result<()>
    where
        W: gtk::prelude::IsA<gtk::ApplicationWindow>,
    {
        self.0.borrow_mut().hide_for_gtk_window(window)
    }

    /// Hides this menu from a win32 window
    #[cfg(target_os = "windows")]
    pub fn hide_for_hwnd(&self, hwnd: isize) -> crate::Result<()> {
        self.0.borrow().hide_for_hwnd(hwnd)
    }

    /// Shows this menu on a [`gtk::ApplicationWindow`]
    #[cfg(target_os = "linux")]
    pub fn show_for_gtk_window<W>(&self, window: &W) -> crate::Result<()>
    where
        W: gtk::prelude::IsA<gtk::ApplicationWindow>,
    {
        self.0.borrow_mut().show_for_gtk_window(window)
    }

    /// Shows this menu on a win32 window
    #[cfg(target_os = "windows")]
    pub fn show_for_hwnd(&self, hwnd: isize) -> crate::Result<()> {
        self.0.borrow().show_for_hwnd(hwnd)
    }

    /// Returns whether this menu visible on a [`gtk::ApplicationWindow`]
    #[cfg(target_os = "linux")]
    pub fn is_visible_on_gtk_window<W>(&self, window: &W) -> bool
    where
        W: gtk::prelude::IsA<gtk::ApplicationWindow>,
    {
        self.0.borrow().is_visible_on_gtk_window(window)
    }

    #[cfg(target_os = "linux")]
    /// Returns the [`gtk::MenuBar`] that is associated with this window if it exists.
    /// This is useful to get information about the menubar for example its height.
    pub fn gtk_menubar_for_gtk_window<W>(self, window: &W) -> Option<gtk::MenuBar>
    where
        W: gtk::prelude::IsA<gtk::ApplicationWindow>,
    {
        self.0.borrow().gtk_menubar_for_gtk_window(window)
    }

    /// Returns whether this menu visible on a on a win32 window
    #[cfg(target_os = "windows")]
    pub fn is_visible_on_hwnd(&self, hwnd: isize) -> bool {
        self.0.borrow().is_visible_on_hwnd(hwnd)
    }

    /// Adds this menu to an NSApp.
    #[cfg(target_os = "macos")]
    pub fn init_for_nsapp(&self) {
        self.0.borrow_mut().init_for_nsapp()
    }

    /// Removes this menu from an NSApp.
    #[cfg(target_os = "macos")]
    pub fn remove_for_nsapp(&self) {
        self.0.borrow_mut().remove_for_nsapp()
    }
}

impl ContextMenu for Menu {
    #[cfg(target_os = "windows")]
    fn hpopupmenu(&self) -> windows_sys::Win32::UI::WindowsAndMessaging::HMENU {
        self.0.borrow().hpopupmenu()
    }

    #[cfg(target_os = "windows")]
    fn show_context_menu_for_hwnd(&self, hwnd: isize, position: Option<Position>) {
        self.0.borrow().show_context_menu_for_hwnd(hwnd, position)
    }

    #[cfg(target_os = "windows")]
    fn attach_menu_subclass_for_hwnd(&self, hwnd: isize) {
        self.0.borrow().attach_menu_subclass_for_hwnd(hwnd)
    }

    #[cfg(target_os = "windows")]
    fn detach_menu_subclass_from_hwnd(&self, hwnd: isize) {
        self.0.borrow().detach_menu_subclass_from_hwnd(hwnd)
    }

    #[cfg(target_os = "linux")]
    fn show_context_menu_for_gtk_window(
        &self,
        window: &gtk::ApplicationWindow,
        position: Option<Position>,
    ) {
        self.0
            .borrow_mut()
            .show_context_menu_for_gtk_window(window, position)
    }

    #[cfg(target_os = "linux")]
    fn gtk_context_menu(&self) -> gtk::Menu {
        self.0.borrow_mut().gtk_context_menu()
    }

    #[cfg(target_os = "macos")]
    fn show_context_menu_for_nsview(&self, view: cocoa::base::id, position: Option<Position>) {
        self.0
            .borrow_mut()
            .show_context_menu_for_nsview(view, position)
    }

    #[cfg(target_os = "macos")]
    fn ns_menu(&self) -> *mut std::ffi::c_void {
        self.0.borrow().ns_menu()
    }
}
