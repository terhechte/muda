// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{cell::RefCell, rc::Rc};

use crate::{accelerator::Accelerator, IsMenuItem, MenuItemKind};

/// A check menu item inside a [`Menu`] or [`Submenu`]
/// and usually contains a text and a check mark or a similar toggle
/// that corresponds to a checked and unchecked states.
///
/// [`Menu`]: crate::Menu
/// [`Submenu`]: crate::Submenu
#[derive(Clone)]
pub struct CheckMenuItem(pub(crate) Rc<RefCell<crate::platform_impl::MenuChild>>);

unsafe impl IsMenuItem for CheckMenuItem {
    fn kind(&self) -> MenuItemKind {
        MenuItemKind::Check(self.clone())
    }
}

impl CheckMenuItem {
    /// Create a new check menu item.
    ///
    /// - `text` could optionally contain an `&` before a character to assign this character as the mnemonic
    /// for this check menu item. To display a `&` without assigning a mnemenonic, use `&&`.
    pub fn new<S: AsRef<str>>(
        text: S,
        enabled: bool,
        checked: bool,
        acccelerator: Option<Accelerator>,
    ) -> Self {
        Self(Rc::new(RefCell::new(
            crate::platform_impl::MenuChild::new_check(
                text.as_ref(),
                enabled,
                checked,
                acccelerator,
            ),
        )))
    }

    /// Returns a unique identifier associated with this submenu.
    pub fn id(&self) -> u32 {
        self.0.borrow().id()
    }

    /// Get the text for this check menu item.
    pub fn text(&self) -> String {
        self.0.borrow().text()
    }

    /// Set the text for this check menu item. `text` could optionally contain
    /// an `&` before a character to assign this character as the mnemonic
    /// for this check menu item. To display a `&` without assigning a mnemenonic, use `&&`.
    pub fn set_text<S: AsRef<str>>(&self, text: S) {
        self.0.borrow_mut().set_text(text.as_ref())
    }

    /// Get whether this check menu item is enabled or not.
    pub fn is_enabled(&self) -> bool {
        self.0.borrow().is_enabled()
    }

    /// Enable or disable this check menu item.
    pub fn set_enabled(&self, enabled: bool) {
        self.0.borrow_mut().set_enabled(enabled)
    }

    /// Set this check menu item accelerator.
    pub fn set_accelerator(&self, acccelerator: Option<Accelerator>) -> crate::Result<()> {
        self.0.borrow_mut().set_accelerator(acccelerator)
    }

    /// Get whether this check menu item is checked or not.
    pub fn is_checked(&self) -> bool {
        self.0.borrow().is_checked()
    }

    /// Check or Uncheck this check menu item.
    pub fn set_checked(&self, checked: bool) {
        self.0.borrow_mut().set_checked(checked)
    }
}
