extern crate fyrox;
use fyrox::{
    core::{color::Color, futures::executor::block_on, pool::Handle},
    engine::executor::Executor,
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    plugin::{Plugin, PluginConstructor, PluginContext},
    scene::{Scene},
};

struct Game {
    scene: Handle<Scene>,
}

struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn create_instance(&self, _: Handle<Scene>, context: PluginContext) -> Box<dyn Plugin> {
        Box::new(Game::new(context))
    }
}

impl Game {
    fn new(context: PluginContext) -> Self {
        let mut scene = Scene::new();

        scene.ambient_lighting_color = Color::opaque(150, 150, 150);

        Self {
            scene: context.scenes.add(scene),
        }
    }
}

impl Plugin for Game {
    fn update(&mut self, context: &mut PluginContext, _: &mut ControlFlow) {

    }

    fn on_os_event(
        &mut self,
        event: &Event<()>,
        _context: PluginContext,
        _control_flow: &mut ControlFlow,
    ) {

    }
}

fn main() {
    let mut executor = Executor::new();
    executor.add_plugin_constructor(GameConstructor);
    executor.get_window().set_title("RPG");
    executor.run();
}
