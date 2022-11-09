#![allow(unused)]
fn main() {
    extern crate fyrox;

    #[cfg(test)]
    use crate::player::camera::CameraController;

    // Import everything we need for the tutorial.
    use fyrox::{
        animation::{
            machine::{Machine, Parameter, PoseNode, State, Transition},
            Animation,
        },
        core::{
            algebra::{UnitQuaternion, Vector3},
            pool::Handle,
        },
        engine::resource_manager::ResourceManager,
        event::{DeviceEvent, ElementState, KeyboardInput, VirtualKeyCode},
        resource::model::Model,
        scene::{
            base::BaseBuilder, collider::ColliderBuilder, collider::ColliderShape,
            graph::physics::CoefficientCombineRule, node::Node, rigidbody::RigidBodyBuilder,
            transform::TransformBuilder, Scene, graph::Graph
        },
    };

    #[cfg(test)]
    mod camera;

    struct CameraController;
    impl CameraController {
        async fn new(_: &mut Graph, _: ResourceManager) -> Self { Self }
    }

    pub struct Player {
        model: Handle<Node>,
        camera_controller: CameraController,
    }

    impl Player {
        pub async fn new(resource_manager: ResourceManager, scene: &mut Scene) -> Self {
            // Load paladin 3D model and create its instance in the scene.
            let model = resource_manager
                .request_model("data/models/paladin/paladin.fbx")
                .await
                .unwrap()
                .instantiate_geometry(scene);

            scene.graph[model]
                .local_transform_mut()
                // Move the model a bit down because its center is at model's feet
                // and we'd get floating model without this offset.
                .set_position(Vector3::new(0.0, -0.75, 0.0))
                // Scale down paladin's model because it is too big.
                .set_scale(Vector3::new(0.02, 0.02, 0.02));

            Self {
                model,

                // As a final stage create camera controller.
                camera_controller: CameraController::new(&mut scene.graph, resource_manager).await,
            }
        }
    }
}
