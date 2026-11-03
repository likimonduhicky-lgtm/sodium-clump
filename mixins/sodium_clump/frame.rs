// HEAD adapter for the client frame hook.
// Called every client frame – ideal place to push visual option state into the game.

use crate::game::sodium_clump;

pub fn on_frame(game: &mut /* crate::game::Game<'_> */ ()) {
    // The real signature is &mut Game<'_>.
    // We keep this file thin so the heavy logic stays in runtime.rs (testable).
    let _ = game;
    sodium_clump::on_client_frame();
}
