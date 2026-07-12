use kodanu::prelude::*;

fn main() {
    let window_config = WindowConfig::default()
        .with_title("Kodanu")
        .with_decorations(false);

    let renderer_config =
        RendererConfig::default().with_backends(Backend::VULKAN | Backend::DX12 | Backend::METAL);

    let log_config = LogConfig::default()
        .with_directive("wgpu_hal=error")
        .with_directive("calloop=off");

    let mut app = App::default()
        .with_window_config(window_config)
        .with_renderer_config(renderer_config)
        .with_log_config(log_config);

    init_test_mesh(app.scene_mut());

    app.run();
}

fn init_test_mesh(scene: &mut Scene) {
    scene.world_mut().spawn((
        Transform::default(),
        MeshRenderer::new(Mesh::cube_2d(), Material::new(Color::GREEN)),
    ));

    scene.world_mut().spawn((
        Transform::from_position(Vec3::new(-1.0, 0.0, -5.0)),
        MeshRenderer::new(Mesh::triangle_2d(), Material::new(Color::BLUE)),
    ));

    scene.world_mut().spawn((
        Transform::from_position(Vec3::new(-2.5, 0.0, 0.0)),
        MeshRenderer::new(Mesh::cube_2d(), Material::new(Color::RED)),
    ));
}
