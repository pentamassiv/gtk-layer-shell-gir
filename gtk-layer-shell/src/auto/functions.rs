// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::Edge;
#[cfg(any(feature = "v0_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
use crate::KeyboardMode;
use crate::Layer;
use glib::object::IsA;
use glib::translate::*;


/// When auto exclusive zone is enabled, exclusive zone is automatically set to the
/// size of the `window` + relevant margin. To disable auto exclusive zone, just set the
/// exclusive zone to 0 or any other fixed value.
///
/// NOTE: you can control the auto exclusive zone by changing the margin on the non-anchored
/// edge. This behavior is specific to gtk-layer-shell and not part of the underlying protocol
/// ## `window`
/// A layer surface.
#[doc(alias = "gtk_layer_auto_exclusive_zone_enable")]
pub fn auto_exclusive_zone_enable(window: &impl IsA<gtk::Window>) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_auto_exclusive_zone_enable(window.as_ref().to_glib_none().0);
    }
}

/// ## `window`
/// A layer surface.
///
/// # Returns
///
/// if the surface's exclusive zone is set to change based on the window's size
#[cfg(any(feature = "v0_5", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_5")))]
#[doc(alias = "gtk_layer_auto_exclusive_zone_is_enabled")]
pub fn auto_exclusive_zone_is_enabled(window: &impl IsA<gtk::Window>) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_layer_auto_exclusive_zone_is_enabled(window.as_ref().to_glib_none().0))
    }
}

/// ## `window`
/// A layer surface.
///
/// # Returns
///
/// if this surface is anchored to the given edge.
#[cfg(any(feature = "v0_5", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_5")))]
#[doc(alias = "gtk_layer_get_anchor")]
#[doc(alias = "get_anchor")]
pub fn is_anchor(window: &impl IsA<gtk::Window>, edge: Edge) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_layer_get_anchor(window.as_ref().to_glib_none().0, edge.into_glib()))
    }
}

/// ## `window`
/// A layer surface.
///
/// # Returns
///
/// the window's exclusive zone (which may have been set manually or automatically)
#[cfg(any(feature = "v0_5", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_5")))]
#[doc(alias = "gtk_layer_get_exclusive_zone")]
#[doc(alias = "get_exclusive_zone")]
pub fn exclusive_zone(window: &impl IsA<gtk::Window>) -> i32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_get_exclusive_zone(window.as_ref().to_glib_none().0)
    }
}

///
/// # Deprecated since 0.6
///
/// Use gtk_layer_get_keyboard_mode () instead.
/// ## `window`
/// A layer surface.
///
/// # Returns
///
/// if keybaord interacitvity is enabled
#[cfg_attr(feature = "v0_6", deprecated = "Since 0.6")]
#[cfg(any(feature = "v0_5", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_5")))]
#[doc(alias = "gtk_layer_get_keyboard_interactivity")]
#[doc(alias = "get_keyboard_interactivity")]
pub fn is_keyboard_interactivity(window: &impl IsA<gtk::Window>) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_layer_get_keyboard_interactivity(window.as_ref().to_glib_none().0))
    }
}

/// ## `window`
/// A layer surface.
///
/// # Returns
///
/// current keyboard interactivity mode for `window`.
#[cfg(any(feature = "v0_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
#[doc(alias = "gtk_layer_get_keyboard_mode")]
#[doc(alias = "get_keyboard_mode")]
pub fn keyboard_mode(window: &impl IsA<gtk::Window>) -> KeyboardMode {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_layer_get_keyboard_mode(window.as_ref().to_glib_none().0))
    }
}

/// ## `window`
/// A layer surface.
///
/// # Returns
///
/// the current layer.
#[cfg(any(feature = "v0_5", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_5")))]
#[doc(alias = "gtk_layer_get_layer")]
#[doc(alias = "get_layer")]
pub fn layer(window: &impl IsA<gtk::Window>) -> Layer {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_layer_get_layer(window.as_ref().to_glib_none().0))
    }
}

///
/// # Returns
///
/// the major version number of the GTK Layer Shell library
#[cfg(any(feature = "v0_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
#[doc(alias = "gtk_layer_get_major_version")]
#[doc(alias = "get_major_version")]
pub fn major_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_get_major_version()
    }
}

/// ## `window`
/// A layer surface.
///
/// # Returns
///
/// the size of the margin for the given edge.
#[cfg(any(feature = "v0_5", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_5")))]
#[doc(alias = "gtk_layer_get_margin")]
#[doc(alias = "get_margin")]
pub fn margin(window: &impl IsA<gtk::Window>, edge: Edge) -> i32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_get_margin(window.as_ref().to_glib_none().0, edge.into_glib())
    }
}

///
/// # Returns
///
/// the micro/patch version number of the GTK Layer Shell library
#[cfg(any(feature = "v0_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
#[doc(alias = "gtk_layer_get_micro_version")]
#[doc(alias = "get_micro_version")]
pub fn micro_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_get_micro_version()
    }
}

///
/// # Returns
///
/// the minor version number of the GTK Layer Shell library
#[cfg(any(feature = "v0_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
#[doc(alias = "gtk_layer_get_minor_version")]
#[doc(alias = "get_minor_version")]
pub fn minor_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_get_minor_version()
    }
}

/// NOTE: To get which monitor the surface is actually on, use
/// `gdk_display_get_monitor_at_window()`.
/// ## `window`
/// A layer surface.
///
/// # Returns
///
/// the monitor this surface will/has requested to be on, can be [`None`].
#[cfg(any(feature = "v0_5", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_5")))]
#[doc(alias = "gtk_layer_get_monitor")]
#[doc(alias = "get_monitor")]
pub fn monitor(window: &impl IsA<gtk::Window>) -> Option<gdk::Monitor> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gtk_layer_get_monitor(window.as_ref().to_glib_none().0))
    }
}

/// NOTE: this function does not return ownership of the string. Do not free the returned string.
/// Future calls into the library may invalidate the returned string.
/// ## `window`
/// A layer surface.
///
/// # Returns
///
/// a reference to the namespace property. If namespace is unset, returns the
/// default namespace ("gtk-layer-shell"). Never returns [`None`].
#[cfg(any(feature = "v0_5", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_5")))]
#[doc(alias = "gtk_layer_get_namespace")]
#[doc(alias = "get_namespace")]
pub fn namespace(window: &impl IsA<gtk::Window>) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gtk_layer_get_namespace(window.as_ref().to_glib_none().0))
    }
}

/// May block for a Wayland roundtrip the first time it's called.
///
/// # Returns
///
/// version of the zwlr_layer_shell_v1 protocol supported by the
/// compositor or 0 if the protocol is not supported.
#[cfg(any(feature = "v0_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
#[doc(alias = "gtk_layer_get_protocol_version")]
#[doc(alias = "get_protocol_version")]
pub fn protocol_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_get_protocol_version()
    }
}

//#[cfg(any(feature = "v0_4", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
//#[doc(alias = "gtk_layer_get_zwlr_layer_surface_v1")]
//#[doc(alias = "get_zwlr_layer_surface_v1")]
//pub fn zwlr_layer_surface_v1(window: &impl IsA<gtk::Window>) -> /*Unimplemented*/Option<Fundamental: Pointer> {
//    unsafe { TODO: call ffi:gtk_layer_get_zwlr_layer_surface_v1() }
//}

/// Set the `window` up to be a layer surface once it is mapped. this must be called before
/// the `window` is realized.
/// ## `window`
/// A [`gtk::Window`][gtk::Window] to be turned into a layer surface.
#[doc(alias = "gtk_layer_init_for_window")]
pub fn init_for_window(window: &impl IsA<gtk::Window>) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_init_for_window(window.as_ref().to_glib_none().0);
    }
}

/// ## `window`
/// A [`gtk::Window`][gtk::Window] that may or may not have a layer surface.
///
/// # Returns
///
/// if `window` has been initialized as a layer surface.
#[cfg(any(feature = "v0_5", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_5")))]
#[doc(alias = "gtk_layer_is_layer_window")]
pub fn is_layer_window(window: &impl IsA<gtk::Window>) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_layer_is_layer_window(window.as_ref().to_glib_none().0))
    }
}

/// May block for a Wayland roundtrip the first time it's called.
///
/// # Returns
///
/// [`true`] if the platform is Wayland and Wayland compositor supports the
/// zwlr_layer_shell_v1 protocol.
#[cfg(any(feature = "v0_5", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_5")))]
#[doc(alias = "gtk_layer_is_supported")]
pub fn is_supported() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_layer_is_supported())
    }
}

/// Set whether `window` should be anchored to `edge`.
/// - If two perpendicular edges are anchored, the surface with be anchored to that corner
/// - If two opposite edges are anchored, the window will be stretched across the screen in that direction
///
/// Default is [`false`] for each [`Edge`][crate::Edge]
/// ## `window`
/// A layer surface.
/// ## `edge`
/// A [`Edge`][crate::Edge] this layer suface may be anchored to.
/// ## `anchor_to_edge`
/// Whether or not to anchor this layer surface to `edge`.
#[doc(alias = "gtk_layer_set_anchor")]
pub fn set_anchor(window: &impl IsA<gtk::Window>, edge: Edge, anchor_to_edge: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_set_anchor(window.as_ref().to_glib_none().0, edge.into_glib(), anchor_to_edge.into_glib());
    }
}

/// Has no effect unless the surface is anchored to an edge. Requests that the compositor
/// does not place other surfaces within the given exclusive zone of the anchored edge.
/// For example, a panel can request to not be covered by maximized windows. See
/// wlr-layer-shell-unstable-v1.xml for details.
///
/// Default is 0
/// ## `window`
/// A layer surface.
/// ## `exclusive_zone`
/// The size of the exclusive zone.
#[doc(alias = "gtk_layer_set_exclusive_zone")]
pub fn set_exclusive_zone(window: &impl IsA<gtk::Window>, exclusive_zone: i32) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_set_exclusive_zone(window.as_ref().to_glib_none().0, exclusive_zone);
    }
}

/// Whether the `window` should receive keyboard events from the compositor.
///
/// Default is [`false`]
///
/// # Deprecated since 0.6
///
/// Use gtk_layer_set_keyboard_mode () instead.
/// ## `window`
/// A layer surface.
#[cfg_attr(feature = "v0_6", deprecated = "Since 0.6")]
#[doc(alias = "gtk_layer_set_keyboard_interactivity")]
pub fn set_keyboard_interactivity(window: &impl IsA<gtk::Window>, interactivity: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_set_keyboard_interactivity(window.as_ref().to_glib_none().0, interactivity.into_glib());
    }
}

/// Sets if/when `window` should receive keyboard events from the compositor, see
/// GtkLayerShellKeyboardMode for details.
///
/// Default is [`KeyboardMode::None`][crate::KeyboardMode::None]
/// ## `window`
/// A layer surface.
/// ## `mode`
/// The type of keyboard interactivity requested.
#[cfg(any(feature = "v0_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
#[doc(alias = "gtk_layer_set_keyboard_mode")]
pub fn set_keyboard_mode(window: &impl IsA<gtk::Window>, mode: KeyboardMode) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_set_keyboard_mode(window.as_ref().to_glib_none().0, mode.into_glib());
    }
}

/// Set the "layer" on which the surface appears (controls if it is over top of or below other surfaces). The layer may
/// be changed on-the-fly in the current version of the layer shell protocol, but on compositors that only support an
/// older version the `window` is remapped so the change can take effect.
///
/// Default is [`Layer::Top`][crate::Layer::Top]
/// ## `window`
/// A layer surface.
/// ## `layer`
/// The layer on which this surface appears.
#[doc(alias = "gtk_layer_set_layer")]
pub fn set_layer(window: &impl IsA<gtk::Window>, layer: Layer) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_set_layer(window.as_ref().to_glib_none().0, layer.into_glib());
    }
}

/// Set the margin for a specific `edge` of a `window`. Effects both surface's distance from
/// the edge and its exclusive zone size (if auto exclusive zone enabled).
///
/// Default is 0 for each [`Edge`][crate::Edge]
/// ## `window`
/// A layer surface.
/// ## `edge`
/// The [`Edge`][crate::Edge] for which to set the margin.
/// ## `margin_size`
/// The margin for `edge` to be set.
#[doc(alias = "gtk_layer_set_margin")]
pub fn set_margin(window: &impl IsA<gtk::Window>, edge: Edge, margin_size: i32) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_set_margin(window.as_ref().to_glib_none().0, edge.into_glib(), margin_size);
    }
}

/// Set the output for the window to be placed on, or [`None`] to let the compositor choose.
/// If the window is currently mapped, it will get remapped so the change can take effect.
///
/// Default is [`None`]
/// ## `window`
/// A layer surface.
/// ## `monitor`
/// The output this layer surface will be placed on ([`None`] to let the compositor decide).
#[doc(alias = "gtk_layer_set_monitor")]
pub fn set_monitor(window: &impl IsA<gtk::Window>, monitor: &gdk::Monitor) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_set_monitor(window.as_ref().to_glib_none().0, monitor.to_glib_none().0);
    }
}

/// Set the "namespace" of the surface.
///
/// No one is quite sure what this is for, but it probably should be something generic
/// ("panel", "osk", etc). The `name_space` string is copied, and caller maintians
/// ownership of original. If the window is currently mapped, it will get remapped so
/// the change can take effect.
///
/// Default is "gtk-layer-shell" (which will be used if set to [`None`])
/// ## `window`
/// A layer surface.
/// ## `name_space`
/// The namespace of this layer surface.
#[doc(alias = "gtk_layer_set_namespace")]
pub fn set_namespace(window: &impl IsA<gtk::Window>, name_space: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_layer_set_namespace(window.as_ref().to_glib_none().0, name_space.to_glib_none().0);
    }
}
