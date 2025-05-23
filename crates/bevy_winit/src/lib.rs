use bevy::prelude::*;
use bevy_window::{RawHandleWrapperHolder, Window, WindowCreated, WindowEvent, exit_on_all_closed};
use core::marker::PhantomData;
use state::winit_runner;
use system::{changed_windows, despawn_windows};
pub use system::{create_monitors, create_windows};
use winit::{event_loop::EventLoop, window::WindowId};
pub use winit::{
    event_loop::EventLoopProxy,
    window::{CustomCursor as WinitCustomCursor, CustomCursorSource},
};
pub use winit_config::*;
use winit_monitors::WinitMonitors;
pub use winit_windows::*;
use x11rb::connection::Connection;

mod converters;
mod state;
mod system;
mod winit_config;
mod winit_monitors;
mod winit_windows;

pub fn get_screen_roots() -> u32 {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    // println!(
    //     "root window id (i think) {:?}",
    //     &conn.setup().roots[screen_num].root
    // );
    // println!("screen_num {screen_num}");

    conn.setup().roots[screen_num].root
}

#[derive(Default)]
pub struct WallpaperPlugin<T: Event = WakeUp> {
    /// Allows the window (and the event loop) to be created on any thread
    /// instead of only the main thread.
    ///
    /// See [`EventLoopBuilder::build`](winit::event_loop::EventLoopBuilder::build) for more information on this.
    ///
    /// # Supported platforms
    ///
    /// Only works on Linux (X11/Wayland) and Windows.
    /// This field is ignored on other platforms.
    pub run_on_any_thread: bool,
    marker: PhantomData<T>,
}

impl<T: Event> Plugin for WallpaperPlugin<T> {
    fn build(&self, app: &mut App) {
        let mut event_loop_builder = EventLoop::<T>::with_user_event();

        // linux check is needed because x11 might be enabled on other platforms.
        #[cfg(all(target_os = "linux", feature = "x11"))]
        {
            use winit::platform::x11::EventLoopBuilderExtX11;

            // This allows a Bevy app to be started and ran outside the main thread.
            // A use case for this is to allow external applications to spawn a thread
            // which runs a Bevy app without requiring the Bevy app to need to reside on
            // the main thread, which can be problematic.
            event_loop_builder.with_any_thread(self.run_on_any_thread);
        }

        // // linux check is needed because wayland might be enabled on other platforms.
        // #[cfg(all(target_os = "linux", feature = "wayland"))]
        // {
        //     use winit::platform::wayland::EventLoopBuilderExtWayland;
        //     event_loop_builder.with_any_thread(self.run_on_any_thread);
        // }

        let event_loop = event_loop_builder
            .build()
            .expect("Failed to build event loop");

        app.init_non_send_resource::<WinitWindows>()
            .init_resource::<WinitMonitors>()
            .init_resource::<WinitSettings>()
            .add_event::<RawWinitWindowEvent>()
            .set_runner(|app| winit_runner(app, event_loop))
            .add_systems(
                Last,
                (
                    // `exit_on_all_closed` only checks if windows exist but doesn't access data,
                    // so we don't need to care about its ordering relative to `changed_windows`
                    changed_windows.ambiguous_with(exit_on_all_closed),
                    despawn_windows,
                    // check_keyboard_focus_lost,
                )
                    .chain(),
            );

        // app.add_plugins(AccessKitPlugin);
        // app.add_plugins(cursor::CursorPlugin);
    }
}

/// The default event that can be used to wake the window loop
/// Wakes up the loop if in wait state
#[derive(Debug, Default, Clone, Copy, Event, Reflect)]
#[reflect(Debug, Default)]
pub struct WakeUp;

/// The original window event as produced by Winit. This is meant as an escape
/// hatch for power users that wish to add custom Winit integrations.
/// If you want to process events for your app or game, you should instead use
/// `bevy::window::WindowEvent`, or one of its sub-events.
///
/// When you receive this event it has already been handled by Bevy's main loop.
/// Sending these events will NOT cause them to be processed by Bevy.
#[derive(Debug, Clone, Event)]
pub struct RawWinitWindowEvent {
    /// The window for which the event was fired.
    pub window_id: WindowId,
    /// The raw winit window event.
    pub event: winit::event::WindowEvent,
}

/// A wrapper type around [`winit::event_loop::EventLoopProxy`] with the specific
/// [`winit::event::Event::UserEvent`] used in the [`WinitPlugin`].
///
/// The `EventLoopProxy` can be used to request a redraw from outside bevy.
///
/// Use `Res<EventLoopProxy>` to receive this resource.
#[derive(Resource, Deref)]
pub struct EventLoopProxyWrapper<T: 'static>(EventLoopProxy<T>);

trait AppSendEvent {
    fn send(&mut self, event: impl Into<WindowEvent>);
}

impl AppSendEvent for Vec<WindowEvent> {
    fn send(&mut self, event: impl Into<WindowEvent>) {
        self.push(Into::<WindowEvent>::into(event));
    }
}

/// The parameters of the [`create_windows`] system.
pub type CreateWindowParams<'w, 's, F = ()> = (
    Commands<'w, 's>,
    Query<
        'w,
        's,
        (
            Entity,
            &'static mut Window,
            Option<&'static RawHandleWrapperHolder>,
        ),
        F,
    >,
    EventWriter<'w, WindowCreated>,
    NonSendMut<'w, WinitWindows>,
    // NonSendMut<'w, AccessKitAdapters>,
    // ResMut<'w, WinitActionRequestHandler>,
    // Res<'w, AccessibilityRequested>,
    Res<'w, WinitMonitors>,
);

/// The parameters of the [`create_monitors`] system.
pub type CreateMonitorParams<'w, 's> = (Commands<'w, 's>, ResMut<'w, WinitMonitors>);
