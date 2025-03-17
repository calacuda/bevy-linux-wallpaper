use bevy::{
    a11y::AccessibilityPlugin,
    audio::AudioPlugin,
    log::{Level, LogPlugin},
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    winit::{WakeUp, WinitPlugin},
};
use bevy_window::{PresentMode, WindowMode, WindowResized, WindowResolution};
use game_background::wallpaper_plugin::WallpaperPlugin;
use std::f32::consts::PI;

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

fn main() {
    let mut wp_plug = WallpaperPlugin::<WakeUp>::default();
    wp_plug.run_on_any_thread = true;

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::INFO,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoVsync,
                        name: Some("game-bg".into()),
                        // window_level: WindowLevel::AlwaysOnBottom,
                        mode: WindowMode::Windowed,
                        // resizable: true,
                        // fullsize_content_view: true,
                        resolution: WindowResolution::new(1920., 1080.),
                        position: WindowPosition::At((1680, 0).into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .disable::<AccessibilityPlugin>()
                .disable::<AudioPlugin>()
                .disable::<WinitPlugin>(),
            WireframePlugin,
            wp_plug,
        ))
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: Srgba {
                red: (166. / 255.),
                green: (227. / 255.),
                blue: (161. / 255.),
                alpha: 1.0,
            }
            .into(),
        })
        .add_systems(Startup, (camera_setup, spawn_cube))
        .add_systems(Update, (rotate, log_window_resize))
        .run();
}

fn camera_setup(mut commands: Commands) {
    // commands.insert_resource(ClearColor(
    //     Srgba {
    //         red: (30. / 255.),
    //         green: (30. / 255.),
    //         blue: (46. / 255.),
    //         alpha: 1.0,
    //     }
    //     .into(),
    // ));
    commands.insert_resource(ClearColor(
        Srgba {
            red: 0.,
            green: 0.,
            blue: 0.,
            alpha: 1.0,
        }
        .into(),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        Camera::default(),
        // VisualizationCamera,
        // ClearColorConfig: (Color::BLACK),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));
}

fn spawn_cube(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let cube = meshes.add(Cuboid::default());

    let rot_1 = Quat::from_rotation_x(45.0 * (-PI / 180.0));
    // rot.y = -PI * 2.;
    // rot.z = -PI * 2.0;
    let rot_2 = Quat::from_rotation_y(36.25 * (-PI / 180.0));

    commands.spawn((
        Mesh3d(cube),
        // MeshMaterial3d(debug_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(rot_1 * rot_2),
        Shape,
    ));
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs());
    }
}

fn log_window_resize(mut resize_reader: EventReader<WindowResized>) {
    for e in resize_reader.read() {
        // When resolution is being changed
        info!("{:.1} x {:.1}", e.width, e.height);
    }
}
