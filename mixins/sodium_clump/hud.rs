// HEAD adapter for the HUD hook.
// Called when the HUD is drawn – good place for FPS / coordinate overlays.

use crate::game::sodium_clump;

pub fn on_hud(game: &mut /* crate::game::Game<'_> */ ()) {
    let _ = game;
    sodium_clump::on_hud_draw();
}
