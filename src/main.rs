mod control;
mod editor;
mod game;
mod model;
mod render;

use geng::prelude::*;

#[derive(clap::Parser)]
struct Opts {
    #[clap(flatten)]
    geng: geng::CliArgs,
}

#[derive(geng::asset::Load)]
pub struct Assets {}

fn main() {
    logger::init();
    geng::setup_panic_handler();

    let opts: Opts = clap::Parser::parse();

    let mut options = geng::ContextOptions::default();
    options.window.title = "Untitled Parkour Game".to_owned();
    options.window.vsync = true;
    options.with_cli(&opts.geng);

    Geng::run_with(&options, |geng| async move {
        let assets: Assets =
            geng::asset::Load::load(geng.asset_manager(), &run_dir().join("assets"), &())
                .await
                .expect("Failed to load assets");
        let assets = Rc::new(assets);
        let state = game::Game::new(&geng, &assets);
        geng.run_state(state).await;
    });
}
