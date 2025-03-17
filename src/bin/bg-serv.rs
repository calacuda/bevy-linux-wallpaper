// use anyhow::Result;
use bevy::{
    app::{RunMode, ScheduleRunnerPlugin},
    log::{Level, LogPlugin},
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::RenderPlugin,
    // window::{PresentMode, WindowLevel, WindowMode},
    winit::WinitPlugin,
};
use bevy_capture::{
    CameraTargetHeadless, Capture, CaptureBundle, Encoder,
    encoder::{
        gif::{self, GifEncoder},
        mp4_openh264::Mp4Openh264Encoder,
    },
};
// use rml_rtmp::time::RtmpTimestamp;
// use tokio::{net::TcpListener, sync::mpsc::UnboundedSender};
// use rtmp::{
//     // relay::{pull_client::PullClient, push_client::PushClient},
//     rtmp::RtmpServer,
// };
use std::{
    f32::consts::PI,
    fs::File,
    io::{Read, Seek, Stdout, Write},
    time::Instant,
};
// use streamhub::{StreamsHub, define::StreamHubEventSender};

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

// #[derive(Resource, Clone)]
// struct Sender {
//     epoch: Instant,
//     sender: UnboundedSender<StreamManagerMessage>,
// }
//
// impl Write for Sender {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         if let Err(e) = self.sender.send(StreamManagerMessage::NewVideoData {
//             timestamp: RtmpTimestamp {
//                 value: self.epoch.elapsed().as_millis() as u32,
//             },
//             data: buf.to_owned().into(),
//         }) {
//             println!("{e}");
//         }
//
//         Ok(buf.len())
//     }
//
//     fn flush(&mut self) -> std::io::Result<()> {
//         Ok(())
//     }
// }
//
// impl Seek for Sender {
//     fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
//         // Ok(1920 * 1080 * 4)
//         Ok(0)
//     }
// }
//
// impl Encoder for Sender {
//     fn encode(&mut self, image: &Image) -> bevy_capture::encoder::Result<()> {
//         // if let Err(e) = self.sender.send(StreamManagerMessage::NewVideoData {
//         //     timestamp: RtmpTimestamp {
//         //         value: self.epoch.elapsed().as_millis() as u32,
//         //     },
//         //     data: image.data.to_owned().into(),
//         // }) {
//         //     println!("{e}");
//         // }
//         let image = image.clone().try_into_dynamic()?;
//         let buffer = image.to_rgba8();
//
//         // buffer.to_owned().iter().for_each(|byte| print!("{byte}"));
//         buffer.save("/tmp/frame.png")?;
//
//         Ok(())
//     }
// }

// #[tokio::main]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: Level::INFO,
                    ..default()
                })
                .disable::<WinitPlugin>()
                .set(RenderPlugin {
                    synchronous_pipeline_compilation: true,
                    ..default()
                }),
            WireframePlugin,
            // Add the ScheduleRunnerPlugin to run the app in loop mode
            ScheduleRunnerPlugin {
                run_mode: RunMode::Loop { wait: None },
            },
            bevy_capture::CapturePlugin,
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
        // .insert_resource(sender)
        .add_systems(Startup, (camera_setup, spawn_cube))
        .add_systems(Update, (rotate, update))
        .run();
}

fn camera_setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.insert_resource(ClearColor(
        Srgba {
            red: (30. / 255.),
            green: (30. / 255.),
            blue: (46. / 255.),
            alpha: 1.0,
        }
        .into(),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        Camera::default().target_headless(1920, 1080, &mut images),
        CaptureBundle::default(),
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

// Start capturing
fn update(mut capture: Query<&mut Capture> /* , sender: ResMut<Sender> */) {
    let mut capture = capture.single_mut();
    if !capture.is_capturing() {
        capture.start(
            GifEncoder::new(File::create("/tmp/frame.gif").unwrap())
                .with_repeat(gif::Repeat::Infinite),
            // Mp4Openh264Encoder::new(sender.clone(), 1920, 1080).unwrap(),
            // Mp4Openh264Encoder::new(File::create("/tmp/tmp-bevy.mp4").unwrap(), 1920, 1080)
            //     .unwrap(),
            // sender.clone(),
        );
        info!("started");
    }
}
