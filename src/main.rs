pub mod lib;

use lib::MyGame;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Umirag", "Fares Setbel")
    // Make a Context.
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}
