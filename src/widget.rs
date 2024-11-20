//! Display information and interactive controls in your application.
//!
//! # Re-exports
//! For convenience, the contents of this module are available at the root
//! module. Therefore, you can directly type:
//!
//! ```
//! use iced::{button, Button};
//! ```
//!
//! # Stateful widgets
//! Some widgets need to keep track of __local state__.
//!
//! These widgets have their own module with a `State` type. For instance, a
//! [`TextInput`] has some [`text_input::State`].
pub use iced::widget::{
    button, checkbox, container, pane_grid, pick_list, progress_bar, radio, rule, scrollable,
    slider, text_input, toggler, tooltip, Column, Row, Space, Text,
};

#[cfg(any(feature = "wgpu_canvas", feature = "glow_canvas"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "wgpu_canvas", feature = "glow_canvas")))
)]
pub use crate::backend::widget::canvas;

#[cfg(any(feature = "wgpu_qr_code", feature = "glow_qr_code"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "wgpu_qr_code", feature = "glow_qr_code")))
)]
pub use crate::backend::widget::qr_code;

#[cfg_attr(docsrs, doc(cfg(feature = "wgpu_image")))]
pub mod image {
    //! Display images in your user interface.
    pub use iced_native::image::Handle;
    pub use iced_native::widget::image::viewer;
    pub use iced_native::widget::image::{Image, Viewer};
}

#[cfg_attr(docsrs, doc(cfg(feature = "wgpu_svg")))]
pub mod svg {
    //! Display vector graphics in your user interface.
    pub use iced_native::svg::Handle;
    pub use iced_native::widget::svg::Svg;
}

#[doc(no_inline)]
pub use {
    button::Button, checkbox::Checkbox, container::Container, iced_native::widget::tree,
    iced_native::widget::tree::Tree, iced_style::theme, iced_style::Theme, image::Image,
    pane_grid::PaneGrid, pick_list::PickList, progress_bar::ProgressBar, radio::Radio, rule::Rule,
    scrollable::Scrollable, slider::Slider, svg::Svg, text_input::TextInput, toggler::Toggler,
    tooltip::Tooltip,
};

#[cfg(any(feature = "wgpu_canvas", feature = "glow_canvas"))]
#[doc(no_inline)]
pub use canvas::Canvas;

#[cfg(any(feature = "wgpu_qr_code", feature = "glow_qr_code"))]
#[doc(no_inline)]
pub use qr_code::QRCode;
