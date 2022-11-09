#![allow(unused)]
fn main() {
    extern crate fyrox;
    // Import everything we need for the tutorial.
    use fyrox::{
        core::{
            algebra::{UnitQuaternion, Vector3},
            pool::Handle,
        },
        engine::resource_manager::ResourceManager,
        event::DeviceEvent,
        resource::texture::TextureWrapMode,
        scene::{
            base::BaseBuilder,
            camera::{CameraBuilder, SkyBox, SkyBoxBuilder},
            graph::Graph,
            node::Node,
            transform::TransformBuilder,
            pivot::PivotBuilder
        },
    };

    async fn create_skybox(resource_manager: ResourceManager) -> SkyBox {
        // Load skybox textures in parallel.
        let (front, back, left, right, top, bottom) = fyrox::core::futures::join!(
        resource_manager.request_texture("data/textures/skybox/front.jpg"),
        resource_manager.request_texture("data/textures/skybox/back.jpg"),
        resource_manager.request_texture("data/textures/skybox/left.jpg"),
        resource_manager.request_texture("data/textures/skybox/right.jpg"),
        resource_manager.request_texture("data/textures/skybox/up.jpg"),
        resource_manager.request_texture("data/textures/skybox/down.jpg")
    );

        // Unwrap everything.
        let skybox = SkyBoxBuilder {
            front: Some(front.unwrap()),
            back: Some(back.unwrap()),
            left: Some(left.unwrap()),
            right: Some(right.unwrap()),
            top: Some(top.unwrap()),
            bottom: Some(bottom.unwrap()),
        }
            .build()
            .unwrap();

        // Set S and T coordinate wrap mode, ClampToEdge will remove any possible seams on edges
        // of the skybox.
        let cubemap = skybox.cubemap();
        let mut data = cubemap.as_ref().unwrap().data_ref();
        data.set_s_wrap_mode(TextureWrapMode::ClampToEdge);
        data.set_t_wrap_mode(TextureWrapMode::ClampToEdge);

        skybox
    }

    pub struct CameraController {
        pivot: Handle<Node>,
        hinge: Handle<Node>,
        camera: Handle<Node>,
    }

    impl CameraController {
        pub async fn new(graph: &mut Graph, resource_manager: ResourceManager) -> Self {
            let camera;
            let hinge;
            let pivot = PivotBuilder::new(BaseBuilder::new()
                .with_children(&[{
                    hinge = PivotBuilder::new(BaseBuilder::new()
                        .with_local_transform(
                            TransformBuilder::new()
                                .with_local_position(Vector3::new(0.0, 0.55, 0.0))
                                .build(),
                        )
                        .with_children(&[{
                            camera = CameraBuilder::new(
                                BaseBuilder::new().with_local_transform(
                                    TransformBuilder::new()
                                        .with_local_position(Vector3::new(0.0, 0.0, -2.0))
                                        .build(),
                                ),
                            )
                                .with_z_far(48.0)
                                .with_skybox(create_skybox(resource_manager).await)
                                .build(graph);
                            camera
                        }]))
                        .build(graph);
                    hinge
                }]))
                .build(graph);

            Self {
                pivot,
                hinge,
                camera,
            }
        }
    }
}
