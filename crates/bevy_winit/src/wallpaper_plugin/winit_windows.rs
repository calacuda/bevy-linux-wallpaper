use std::num::NonZeroU32;

use super::{
    converters::{convert_enabled_buttons, convert_window_level, convert_window_theme},
    winit_monitors::WinitMonitors,
};
use bevy::utils::HashMap;
use bevy_ecs::entity::Entity;
use bevy_ecs::entity::EntityHashMap;
use bevy_window::{
    CursorGrabMode, MonitorSelection, Window, WindowMode, WindowPosition, WindowResolution,
    WindowWrapper,
};
use tracing::{error, info, warn};
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    error::ExternalError,
    event_loop::ActiveEventLoop,
    monitor::MonitorHandle,
    raw_window_handle::{RawWindowHandle, XcbWindowHandle},
    window::{CursorGrabMode as WinitCursorGrabMode, Fullscreen, Window as WinitWindow, WindowId},
};
use winit::{
    platform::x11::{WindowAttributesExtX11, WindowType},
    window::WindowButtons,
};

/// A resource mapping window entities to their `winit`-backend [`Window`](winit::window::Window)
/// states.
#[derive(Debug, Default)]
pub struct WinitWindows {
    /// Stores [`winit`] windows by window identifier.
    pub windows: HashMap<WindowId, WindowWrapper<WinitWindow>>,
    /// Maps entities to `winit` window identifiers.
    pub entity_to_winit: EntityHashMap<WindowId>,
    /// Maps `winit` window identifiers to entities.
    pub winit_to_entity: HashMap<WindowId, Entity>,
    // Many `winit` window functions (e.g. `set_window_icon`) can only be called on the main thread.
    // If they're called on other threads, the program might hang. This marker indicates that this
    // type is not thread-safe and will be `!Send` and `!Sync`.
    _not_send_sync: core::marker::PhantomData<*const ()>,
}

impl WinitWindows {
    /// Creates a `winit` window and associates it with our entity.
    pub fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        entity: Entity,
        window: &Window,
        // adapters: &mut AccessKitAdapters,
        // handlers: &mut WinitActionRequestHandlers,
        // accessibility_requested: &AccessibilityRequested,
        monitors: &WinitMonitors,
        parent_window_id: u32,
    ) -> &WindowWrapper<WinitWindow> {
        // println!("create_window");
        let mut winit_window_attributes = WinitWindow::default_attributes();

        // Due to a UIA limitation, winit windows need to be invisible for the
        // AccessKit adapter is initialized.
        // winit_window_attributes = winit_window_attributes.with_visible(false);

        let maybe_selected_monitor = &match window.mode {
            WindowMode::BorderlessFullscreen(monitor_selection)
            | WindowMode::Fullscreen(monitor_selection) => select_monitor(
                monitors,
                event_loop.primary_monitor(),
                None,
                &monitor_selection,
            ),
            WindowMode::Windowed => None,
            WindowMode::SizedFullscreen(_) => {
                // panic!("Sized full screen is disabled for this build");
                None
            }
        };

        match window.mode {
            WindowMode::BorderlessFullscreen(_) => {
                winit_window_attributes = winit_window_attributes
                    .with_fullscreen(Some(Fullscreen::Borderless(maybe_selected_monitor.clone())));
            }
            // WindowMode::Fullscreen(monitor_selection, video_mode_selection) => {
            //     let select_monitor = &maybe_selected_monitor
            //         .clone()
            //         .expect("Unable to get monitor.");
            //
            //     if let Some(video_mode) =
            //         get_selected_videomode(select_monitor, &video_mode_selection)
            //     {
            //         winit_window_attributes.with_fullscreen(Some(Fullscreen::Exclusive(video_mode)))
            //     } else {
            //         warn!(
            //             "Could not find valid fullscreen video mode for {:?} {:?}",
            //             monitor_selection, video_mode_selection
            //         );
            //         winit_window_attributes
            //     }
            // }
            WindowMode::Windowed => {
                if let Some(position) = winit_window_position(
                    &window.position,
                    &window.resolution,
                    monitors,
                    event_loop.primary_monitor(),
                    None,
                ) {
                    winit_window_attributes = winit_window_attributes.with_position(position);
                }
                let logical_size = LogicalSize::new(window.width(), window.height());

                winit_window_attributes =
                    if let Some(sf) = window.resolution.scale_factor_override() {
                        let inner_size = logical_size.to_physical::<f64>(sf.into());
                        winit_window_attributes.with_inner_size(inner_size)
                    } else {
                        winit_window_attributes.with_inner_size(logical_size)
                    };
            }
            _ => {
                // panic!("selected as disabled window mode");
                error!("selected as disabled window mode");
            }
        };

        info!(
            "making window with geometry: {} x {}",
            window.resolution.physical_width(),
            window.resolution.physical_height()
        );

        // It's crucial to avoid setting the window's final visibility here;
        // as explained above, the window must be invisible until the AccessKit
        // adapter is created.
        winit_window_attributes = winit_window_attributes
            .with_window_level(convert_window_level(window.window_level))
            .with_theme(window.window_theme.map(convert_window_theme))
            .with_resizable(window.resizable)
            .with_enabled_buttons(convert_enabled_buttons(window.enabled_buttons))
            .with_decorations(window.decorations)
            .with_transparent(window.transparent);

        let display_info = DisplayInfo {
            window_physical_resolution: (
                window.resolution.physical_width(),
                window.resolution.physical_height(),
            ),
            window_logical_resolution: (window.resolution.width(), window.resolution.height()),
            monitor_name: maybe_selected_monitor
                .as_ref()
                .and_then(MonitorHandle::name),
            scale_factor: maybe_selected_monitor
                .as_ref()
                .map(MonitorHandle::scale_factor),
            refresh_rate_millihertz: maybe_selected_monitor
                .as_ref()
                .and_then(MonitorHandle::refresh_rate_millihertz),
        };
        bevy_log::debug!("{display_info}");

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "windows"
        ))]
        if let Some(name) = &window.name {
            #[cfg(all(
                feature = "x11",
                any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd"
                )
            ))]
            {
                winit_window_attributes = winit::platform::x11::WindowAttributesExtX11::with_name(
                    winit_window_attributes,
                    name.clone(),
                    "",
                );
            }
        }

        // let parent_window_id = 0x1e8;
        // let parent_window_ids = get_screen_roots();
        // println!("attempting to parent to window {:?}", parent_window_ids);
        //
        // for parent_window_id in parent_window_ids {
        winit_window_attributes = winit_window_attributes
            .with_embed_parent_window(parent_window_id)
            .with_override_redirect(true)
            .with_fullscreen(Some(Fullscreen::Borderless(None)))
            .with_maximized(true)
            .with_decorations(false)
            .with_enabled_buttons(WindowButtons::empty())
            // .with_x11_visual(0x1);
            .with_x11_window_type(Vec::from([WindowType::Notification]));

        // winit_window_attributes = unsafe {
        //     winit_window_attributes.with_parent_window(Some(RawWindowHandle::Xcb(
        //         XcbWindowHandle::new(NonZeroU32::new(parent_window_id).unwrap()),
        //     )))
        // };

        let constraints = window.resize_constraints.check_constraints();
        let min_inner_size = LogicalSize {
            width: constraints.min_width,
            height: constraints.min_height,
        };
        let max_inner_size = LogicalSize {
            width: constraints.max_width,
            height: constraints.max_height,
        };

        let winit_window_attributes =
            if constraints.max_width.is_finite() && constraints.max_height.is_finite() {
                winit_window_attributes
                    .with_min_inner_size(min_inner_size)
                    .with_max_inner_size(max_inner_size)
            } else {
                winit_window_attributes.with_min_inner_size(min_inner_size)
            };

        #[expect(clippy::allow_attributes, reason = "`unused_mut` is not always linted")]
        #[allow(
            unused_mut,
            reason = "This variable needs to be mutable if `cfg(target_arch = \"wasm32\")`"
        )]
        let mut winit_window_attributes = winit_window_attributes.with_title(window.title.as_str());

        // let winit_window = event_loop.create_window(winit_window_attributes).unwrap();
        // println!("winit_window_atters => {:?}", winit_window_attributes);
        let winit_window_res = event_loop.create_window(winit_window_attributes);
        // println!("winit_window_res => {:?}", winit_window_res);
        let winit_window = winit_window_res.unwrap();
        let _name = window.title.clone();

        // Now that the AccessKit adapter is created, it's safe to show
        // the window.
        winit_window.set_visible(window.visible);

        // winit_window.

        // Do not set the grab mode on window creation if it's none. It can fail on mobile.
        if window.cursor_options.grab_mode != CursorGrabMode::None {
            let _ = attempt_grab(&winit_window, window.cursor_options.grab_mode);
        }

        winit_window.set_cursor_visible(window.cursor_options.visible);

        // Do not set the cursor hittest on window creation if it's false, as it will always fail on
        // some platforms and log an unfixable warning.
        if !window.cursor_options.hit_test {
            if let Err(err) = winit_window.set_cursor_hittest(window.cursor_options.hit_test) {
                warn!(
                    "Could not set cursor hit test for window {}: {}",
                    window.title, err
                );
            }
        }

        let pwi: WindowId = (parent_window_id as u64).into();
        info!("pwi => {pwi:?}");

        self.entity_to_winit.insert(entity, winit_window.id());
        self.winit_to_entity.insert(winit_window.id(), entity);
        // self.entity_to_winit.insert(entity, pwi);
        // self.winit_to_entity.insert(pwi, entity);

        self.windows
            .entry(winit_window.id())
            // .entry(pwi)
            .insert(WindowWrapper::new(winit_window))
            .into_mut()
    }

    /// Get the winit window that is associated with our entity.
    pub fn get_window(&self, entity: Entity) -> Option<&WindowWrapper<WinitWindow>> {
        self.entity_to_winit
            .get(&entity)
            .and_then(|winit_id| self.windows.get(winit_id))
    }

    /// Get the entity associated with the winit window id.
    ///
    /// This is mostly just an intermediary step between us and winit.
    pub fn get_window_entity(&self, winit_id: WindowId) -> Option<Entity> {
        self.winit_to_entity.get(&winit_id).cloned()
    }

    /// Remove a window from winit.
    ///
    /// This should mostly just be called when the window is closing.
    pub fn remove_window(&mut self, entity: Entity) -> Option<WindowWrapper<WinitWindow>> {
        let winit_id = self.entity_to_winit.remove(&entity)?;
        self.winit_to_entity.remove(&winit_id);
        self.windows.remove(&winit_id)
    }
}

// /// Returns some [`winit::monitor::VideoModeHandle`] given a [`MonitorHandle`] and a
// /// [`VideoModeSelection`] or None if no valid matching video mode was found.
// pub fn get_selected_videomode(
//     monitor: &MonitorHandle,
//     selection: &VideoModeSelection,
// ) -> Option<VideoModeHandle> {
//     match selection {
//         VideoModeSelection::Current => get_current_videomode(monitor),
//         VideoModeSelection::Specific(specified) => monitor.video_modes().find(|mode| {
//             mode.size().width == specified.physical_size.x
//                 && mode.size().height == specified.physical_size.y
//                 && mode.refresh_rate_millihertz() == specified.refresh_rate_millihertz
//                 && mode.bit_depth() == specified.bit_depth
//         }),
//     }
// }

// /// Gets a monitor's current video-mode.
// ///
// /// TODO: When Winit 0.31 releases this function can be removed and replaced with
// /// `MonitorHandle::current_video_mode()`
// fn get_current_videomode(monitor: &MonitorHandle) -> Option<VideoModeHandle> {
//     monitor
//         .video_modes()
//         .filter(|mode| {
//             mode.size() == monitor.size()
//                 && Some(mode.refresh_rate_millihertz()) == monitor.refresh_rate_millihertz()
//         })
//         .max_by_key(VideoModeHandle::bit_depth)
// }

pub(crate) fn attempt_grab(
    winit_window: &WinitWindow,
    grab_mode: CursorGrabMode,
) -> Result<(), ExternalError> {
    let grab_result = match grab_mode {
        CursorGrabMode::None => winit_window.set_cursor_grab(WinitCursorGrabMode::None),
        CursorGrabMode::Confined => winit_window
            .set_cursor_grab(WinitCursorGrabMode::Confined)
            .or_else(|_e| winit_window.set_cursor_grab(WinitCursorGrabMode::Locked)),
        CursorGrabMode::Locked => winit_window
            .set_cursor_grab(WinitCursorGrabMode::Locked)
            .or_else(|_e| winit_window.set_cursor_grab(WinitCursorGrabMode::Confined)),
    };

    if let Err(err) = grab_result {
        let err_desc = match grab_mode {
            CursorGrabMode::Confined | CursorGrabMode::Locked => "grab",
            CursorGrabMode::None => "ungrab",
        };

        tracing::error!("Unable to {} cursor: {}", err_desc, err);
        Err(err)
    } else {
        Ok(())
    }
}

/// Compute the physical window position for a given [`WindowPosition`].
// Ideally we could generify this across window backends, but we only really have winit atm
// so whatever.
pub fn winit_window_position(
    position: &WindowPosition,
    resolution: &WindowResolution,
    monitors: &WinitMonitors,
    primary_monitor: Option<MonitorHandle>,
    current_monitor: Option<MonitorHandle>,
) -> Option<PhysicalPosition<i32>> {
    match position {
        WindowPosition::Automatic => {
            // Window manager will handle position
            None
        }
        WindowPosition::Centered(monitor_selection) => {
            let maybe_monitor = select_monitor(
                monitors,
                primary_monitor,
                current_monitor,
                monitor_selection,
            );

            if let Some(monitor) = maybe_monitor {
                let screen_size = monitor.size();

                let scale_factor = match resolution.scale_factor_override() {
                    Some(scale_factor_override) => scale_factor_override as f64,
                    // We use the monitors scale factor here since `WindowResolution.scale_factor` is
                    // not yet populated when windows are created during plugin setup.
                    None => monitor.scale_factor(),
                };

                // Logical to physical window size
                let (width, height): (u32, u32) =
                    LogicalSize::new(resolution.width(), resolution.height())
                        .to_physical::<u32>(scale_factor)
                        .into();

                let position = PhysicalPosition {
                    x: screen_size.width.saturating_sub(width) as f64 / 2.
                        + monitor.position().x as f64,
                    y: screen_size.height.saturating_sub(height) as f64 / 2.
                        + monitor.position().y as f64,
                };

                Some(position.cast::<i32>())
            } else {
                warn!("Couldn't get monitor selected with: {monitor_selection:?}");
                None
            }
        }
        WindowPosition::At(position) => {
            Some(PhysicalPosition::new(position[0] as f64, position[1] as f64).cast::<i32>())
        }
    }
}

/// Selects a monitor based on the given [`MonitorSelection`].
pub fn select_monitor(
    monitors: &WinitMonitors,
    primary_monitor: Option<MonitorHandle>,
    current_monitor: Option<MonitorHandle>,
    monitor_selection: &MonitorSelection,
) -> Option<MonitorHandle> {
    use bevy_window::MonitorSelection::*;

    match monitor_selection {
        Current => {
            if current_monitor.is_none() {
                warn!(
                    "Can't select current monitor on window creation or cannot find current monitor!"
                );
            }
            current_monitor
        }
        Primary => primary_monitor,
        Index(n) => monitors.nth(*n),
        Entity(entity) => monitors.find_entity(*entity),
    }
}

struct DisplayInfo {
    window_physical_resolution: (u32, u32),
    window_logical_resolution: (f32, f32),
    monitor_name: Option<String>,
    scale_factor: Option<f64>,
    refresh_rate_millihertz: Option<u32>,
}

impl core::fmt::Display for DisplayInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Display information:")?;
        write!(
            f,
            "  Window physical resolution: {}x{}",
            self.window_physical_resolution.0, self.window_physical_resolution.1
        )?;
        write!(
            f,
            "  Window logical resolution: {}x{}",
            self.window_logical_resolution.0, self.window_logical_resolution.1
        )?;
        write!(
            f,
            "  Monitor name: {}",
            self.monitor_name.as_deref().unwrap_or("")
        )?;
        write!(f, "  Scale factor: {}", self.scale_factor.unwrap_or(0.))?;
        let millihertz = self.refresh_rate_millihertz.unwrap_or(0);
        let hertz = millihertz / 1000;
        let extra_millihertz = millihertz % 1000;
        write!(f, "  Refresh rate (Hz): {}.{:03}", hertz, extra_millihertz)?;
        Ok(())
    }
}
