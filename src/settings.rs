//! Configure your application;

use baseview::WindowOpenOptions;

/// The settings of an application.
#[derive(Debug)]
pub struct Settings<Flags> {
    /// The [`Window`] settings
    ///
    /// [`Window`]: struct.Window.html
    pub window: WindowOpenOptions,

    /// The data needed to initialize an [`Application`].
    ///
    /// [`Application`]: trait.Application.html
    pub flags: Flags,
}
