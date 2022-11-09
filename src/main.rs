extern crate fyrox;
#[cfg(test)]
use crate::{level::Level, player::Player};
use fyrox::{
    core::{color::Color, futures::executor::block_on, pool::Handle},
    engine::{resource_manager::ResourceManager, executor::Executor},
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    plugin::{Plugin, PluginConstructor, PluginContext},
    scene::{Scene},
};

#[cfg(test)]
mod level;
#[cfg(test)]
mod player;

struct Player;
impl Player {
    async fn new(_: ResourceManager, _: &mut Scene) -> Self { Self }
}

struct Level;
impl Level {
    async fn new(_: ResourceManager, _: &mut Scene) -> Self { Self }
}

struct Game {
    scene: Handle<Scene>,
    level: Level,
    player: Player,
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

        let player = block_on(Player::new(context.resource_manager.clone(), &mut scene));

        Self {
            player,
            level: block_on(Level::new(context.resource_manager.clone(), &mut scene)),
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

