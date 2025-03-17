use bevy::{
    app::{RunMode, ScheduleRunnerPlugin},
    log::{Level, LogPlugin},
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::RenderPlugin,
    window::{PresentMode, WindowLevel, WindowMode},
    winit::WinitPlugin,
};
use bevy_capture::{CameraTargetHeadless, Capture, CaptureBundle, Encoder};
use std::f32::consts::PI;
use x11rb::{
    connection::Connection,
    image::{BitsPerPixel, ImageOrder, ScanlinePad},
    protocol::xproto::{ChangeWindowAttributesAux, ConnectionExt, CreateGCAux, Screen},
    rust_connection::RustConnection,
    wrapper::ConnectionExt as _,
};

struct ToWallpaper {
    conn: RustConnection,
    screen: Screen,
    root: u32,
    depth: u8,
    size: (u16, u16),
}

impl ToWallpaper {
    pub fn new() -> Self {
        let (conn, screen_num) = x11rb::connect(None).unwrap();
        let screen = conn.setup().roots[screen_num].clone();
        let root = screen.root;
        let depth = screen.root_depth;
        let size = (screen.width_in_pixels, screen.height_in_pixels);

        Self {
            conn,
            screen,
            root,
            depth,
            size,
        }
    }
}

impl Encoder for ToWallpaper {
    fn encode(&mut self, image: &Image) -> bevy_capture::encoder::Result<()> {
        // Called for each frame.
        let pixmap = self.conn.generate_id().unwrap();
        self.conn
            .create_pixmap(self.depth, pixmap, self.root, self.size.0, self.size.1)
            .unwrap();

        // let img = x11rb::protocol::xv::Image::default();
        // let img = ppm_parser::parse_ppm_bytes(image.data);
        // let mut img = x11rb::image::Image::allocate(
        //     self.size.0,
        //     self.size.1,
        //     ScanlinePad::Pad8,
        //     24,
        //     BitsPerPixel::B24,
        //     ImageOrder::MsbFirst,
        // );
        // input.read_exact(image.data_mut())?;
        // for (i, img_p) in img.data_mut().iter_mut().enumerate()
        // // .iter_mut()
        // // .zip(image.data.clone())
        // // .for_each(|(img_p, game)| {
        // // *img_p = game;
        // // });
        // {
        //     *img_p = image.data[i];
        // }
        let img = x11rb::image::Image::new(
            // self.size.0,
            // self.size.1,
            image.width() as u16,
            image.height() as u16,
            ScanlinePad::Pad8,
            24,
            BitsPerPixel::B24,
            ImageOrder::MsbFirst,
            image.data.clone().into(),
        )
        .unwrap();

        // println!("{}", img.data() == image.data);
        let gc = self.conn.generate_id().unwrap();
        let gc_aux = CreateGCAux::new().graphics_exposures(0); //.foreground(img);
        self.conn.create_gc(gc, self.root, &gc_aux).unwrap();

        img.put(&self.conn, pixmap, gc, 0, 0)?;
        // let rect = x11rb::protocol::xproto::Rectangle {
        //     x: 0,
        //     y: 0,
        //     width: self.size.0,
        //     height: self.size.1,
        // };

        self.conn.flush().unwrap();
        // self.conn.poly_fill_rectangle(pixmap, gc, &[rect]).unwrap();
        self.conn
            .change_window_attributes(
                self.root,
                &ChangeWindowAttributesAux::new().background_pixmap(pixmap),
            )
            .unwrap();
        self.conn.clear_area(false, self.root, 0, 0, 0, 0).unwrap();

        self.conn.free_pixmap(pixmap).unwrap();
        self.conn.free_gc(gc).unwrap();

        // Make sure the server handled our requests. A flush() is not enough for that.
        self.conn.sync().unwrap();

        Ok(())
    }

    fn finish(self: Box<Self>) {
        // Called when the encoder is stopped.
        // todo!("Finish encoding the frames, if necessary.")
    }
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::INFO,
                    ..default()
                })
                // Disable the WinitPlugin to prevent the creation of a window
                .disable::<WinitPlugin>()
                // Make sure pipelines are ready before rendering
                .set(RenderPlugin {
                    synchronous_pipeline_compilation: true,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoVsync,
                        name: Some("game-bg".into()),
                        // window_level: WindowLevel::AlwaysOn,
                        mode: WindowMode::Windowed,
                        resolution: (1920., 1080.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            WireframePlugin,
            // Add the ScheduleRunnerPlugin to run the app in loop mode
            ScheduleRunnerPlugin {
                run_mode: RunMode::Loop { wait: None },
            },
            // Add the CapturePlugin
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
        // VisualizationCamera,
        // ClearColorConfig: (Color::BLACK),
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

fn update(
    // mut app_exit: EventWriter<AppExit>,
    mut capture: Query<&mut Capture>,
    // mut cubes: Query<&mut Transform, With<Cube>>,
    // mut frame: Local<u32>,
) {
    let mut capture = capture.single_mut();
    if !capture.is_capturing() {
        capture.start(
            // gif::GifEncoder::new(fs::File::create("captures/simple/simple.gif").unwrap())
            //     .with_repeat(gif::Repeat::Infinite),
            // frames::FramesEncoder::new("captures/simple/frames"),
            // mp4_ffmpeg_cli::Mp4FfmpegCliEncoder::new("captures/simple/simple_ffmpeg.mp4")
            //     .unwrap()
            //     .with_framerate(10),
            // mp4_openh264::Mp4Openh264Encoder::new(
            //     fs::File::create("captures/simple/simple_openh264.mp4").unwrap(),
            //     512,
            //     512,
            // )
            // .unwrap(),
            ToWallpaper::new(),
        );
    }
}
