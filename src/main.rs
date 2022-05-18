use geng::prelude::*;

mod game;
mod logic;
mod model;
mod physics;
mod render;

use game::*;
use model::*;
use render::Render;

#[derive(geng::Assets)]
pub struct Assets {}

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Untitled Parkour Game".to_owned(),
        ..default()
    });

    geng.window().lock_cursor();

    geng::run(
        &geng,
        geng::LoadingScreen::new(
            &geng,
            geng::EmptyLoadingScreen,
            {
                let geng = geng.clone();
                async move {
                    let assets = <Assets as geng::LoadAsset>::load(&geng, &static_path())
                        .await
                        .expect("Failed to load assets");
                    (assets,)
                }
            },
            {
                let geng = geng.clone();
                move |(assets,)| {
                    let assets = Rc::new(assets);
                    Game::new(&geng, &assets)
                }
            },
        ),
    )
}
