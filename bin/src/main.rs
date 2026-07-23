use kodanu::prelude::*;

fn main() {
    let window_config = WindowConfig::default()
        .with_title("Kodanu")
        .with_decorations(false);

    let renderer_config = RendererConfig::default()
        .with_backends(Backend::VULKAN | Backend::DX12 | Backend::METAL)
        .with_sample_count(SampleCount::Quad);

    let log_config = LogConfig::default()
        .with_directive("wgpu_hal=error")
        .with_directive("calloop=off");

    let mut app = App::default()
        .with_window_config(window_config)
        .with_renderer_config(renderer_config)
        .with_log_config(log_config);

    app.add_system(Stage::Startup, test_scene_system);
    app.add_system(Stage::Update, perspective_camera_system);

    app.run();
}

fn test_scene_system(ctx: &mut SystemContext) {
    let world = ctx.scene.world_mut();

    {
        let camera = world.spawn();

        world.insert(camera, Transform::default());
        world.insert(camera, Camera::default());
    }

    {
        let cube = world.spawn();

        world.insert(cube, Transform::default());
        world.insert(
            cube,
            MeshRenderer::new(Mesh::cube_2d(), Material::new(Color::GREEN)),
        );
    }
}

fn perspective_camera_system(ctx: &mut SystemContext) {
    let world = ctx.scene.world_mut();

    let Some(query) = world.query_mut::<(&mut Transform, &Camera)>() else {
        return;
    };

    for (transform, _) in query {
        let (input, action_map, time) = (ctx.input, ctx.action_map, ctx.time);

        let direction = transform.forward() * action_map.axis(Axis::MoveY, input)
            + -transform.right() * action_map.axis(Axis::MoveX, input)
            + transform.up() * action_map.axis(Axis::MoveZ, input);

        let yaw = action_map.axis(Axis::LookX, input) * 2.0 * time.delta();
        let pitch = action_map.axis(Axis::LookY, input) * 2.0 * time.delta();

        transform.rotate_world(Quat::from_rotation_y(yaw));
        transform.rotate_local(Quat::from_rotation_x(pitch));

        transform.translate(direction * 10.0 * time.delta());
    }
}
