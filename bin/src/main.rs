use kodanu::prelude::*;

fn main() {
    let log_config = LogConfig::default()
        .with_directive("wgpu_hal=error")
        .with_directive("calloop=off");

    let window_config = WindowConfig::default()
        .with_title("Kodanu")
        .with_decorations(false);

    let mut app = App::default()
        .with_window_config(window_config)
        .with_log_config(log_config);

    init_test_mesh(&mut app.scene_mut());

    app.run();
}

pub fn init_test_mesh(scene: &mut Scene) {
    scene.world_mut().spawn((
        Transform::default(),
        MeshRenderer::new(Mesh::cube_2d(), Material::new(Color::GREEN)),
    ));

    scene.world_mut().spawn((
        Transform::default(),
        MeshRenderer::new(Mesh::triangle_2d(), Material::new(Color::BLUE)),
    ));
}
