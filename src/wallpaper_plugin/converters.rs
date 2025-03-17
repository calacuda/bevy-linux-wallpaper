use bevy_math::CompassOctant;
use bevy_window::SystemCursorIcon;
use bevy_window::{EnabledButtons, WindowLevel, WindowTheme};

/// Converts a [`SystemCursorIcon`] to a [`winit::window::CursorIcon`].
pub fn convert_system_cursor_icon(cursor_icon: SystemCursorIcon) -> winit::window::CursorIcon {
    match cursor_icon {
        SystemCursorIcon::Crosshair => winit::window::CursorIcon::Crosshair,
        SystemCursorIcon::Pointer => winit::window::CursorIcon::Pointer,
        SystemCursorIcon::Move => winit::window::CursorIcon::Move,
        SystemCursorIcon::Text => winit::window::CursorIcon::Text,
        SystemCursorIcon::Wait => winit::window::CursorIcon::Wait,
        SystemCursorIcon::Help => winit::window::CursorIcon::Help,
        SystemCursorIcon::Progress => winit::window::CursorIcon::Progress,
        SystemCursorIcon::NotAllowed => winit::window::CursorIcon::NotAllowed,
        SystemCursorIcon::ContextMenu => winit::window::CursorIcon::ContextMenu,
        SystemCursorIcon::Cell => winit::window::CursorIcon::Cell,
        SystemCursorIcon::VerticalText => winit::window::CursorIcon::VerticalText,
        SystemCursorIcon::Alias => winit::window::CursorIcon::Alias,
        SystemCursorIcon::Copy => winit::window::CursorIcon::Copy,
        SystemCursorIcon::NoDrop => winit::window::CursorIcon::NoDrop,
        SystemCursorIcon::Grab => winit::window::CursorIcon::Grab,
        SystemCursorIcon::Grabbing => winit::window::CursorIcon::Grabbing,
        SystemCursorIcon::AllScroll => winit::window::CursorIcon::AllScroll,
        SystemCursorIcon::ZoomIn => winit::window::CursorIcon::ZoomIn,
        SystemCursorIcon::ZoomOut => winit::window::CursorIcon::ZoomOut,
        SystemCursorIcon::EResize => winit::window::CursorIcon::EResize,
        SystemCursorIcon::NResize => winit::window::CursorIcon::NResize,
        SystemCursorIcon::NeResize => winit::window::CursorIcon::NeResize,
        SystemCursorIcon::NwResize => winit::window::CursorIcon::NwResize,
        SystemCursorIcon::SResize => winit::window::CursorIcon::SResize,
        SystemCursorIcon::SeResize => winit::window::CursorIcon::SeResize,
        SystemCursorIcon::SwResize => winit::window::CursorIcon::SwResize,
        SystemCursorIcon::WResize => winit::window::CursorIcon::WResize,
        SystemCursorIcon::EwResize => winit::window::CursorIcon::EwResize,
        SystemCursorIcon::NsResize => winit::window::CursorIcon::NsResize,
        SystemCursorIcon::NeswResize => winit::window::CursorIcon::NeswResize,
        SystemCursorIcon::NwseResize => winit::window::CursorIcon::NwseResize,
        SystemCursorIcon::ColResize => winit::window::CursorIcon::ColResize,
        SystemCursorIcon::RowResize => winit::window::CursorIcon::RowResize,
        _ => winit::window::CursorIcon::Default,
    }
}

pub fn convert_window_level(window_level: WindowLevel) -> winit::window::WindowLevel {
    match window_level {
        WindowLevel::AlwaysOnBottom => winit::window::WindowLevel::AlwaysOnBottom,
        WindowLevel::Normal => winit::window::WindowLevel::Normal,
        WindowLevel::AlwaysOnTop => winit::window::WindowLevel::AlwaysOnTop,
    }
}

pub fn convert_winit_theme(theme: winit::window::Theme) -> WindowTheme {
    match theme {
        winit::window::Theme::Light => WindowTheme::Light,
        winit::window::Theme::Dark => WindowTheme::Dark,
    }
}

pub fn convert_window_theme(theme: WindowTheme) -> winit::window::Theme {
    match theme {
        WindowTheme::Light => winit::window::Theme::Light,
        WindowTheme::Dark => winit::window::Theme::Dark,
    }
}

pub fn convert_enabled_buttons(enabled_buttons: EnabledButtons) -> winit::window::WindowButtons {
    let mut window_buttons = winit::window::WindowButtons::empty();
    if enabled_buttons.minimize {
        window_buttons.insert(winit::window::WindowButtons::MINIMIZE);
    }
    if enabled_buttons.maximize {
        window_buttons.insert(winit::window::WindowButtons::MAXIMIZE);
    }
    if enabled_buttons.close {
        window_buttons.insert(winit::window::WindowButtons::CLOSE);
    }
    window_buttons
}

pub fn convert_resize_direction(resize_direction: CompassOctant) -> winit::window::ResizeDirection {
    match resize_direction {
        CompassOctant::West => winit::window::ResizeDirection::West,
        CompassOctant::North => winit::window::ResizeDirection::North,
        CompassOctant::East => winit::window::ResizeDirection::East,
        CompassOctant::South => winit::window::ResizeDirection::South,
        CompassOctant::NorthWest => winit::window::ResizeDirection::NorthWest,
        CompassOctant::NorthEast => winit::window::ResizeDirection::NorthEast,
        CompassOctant::SouthWest => winit::window::ResizeDirection::SouthWest,
        CompassOctant::SouthEast => winit::window::ResizeDirection::SouthEast,
    }
}
