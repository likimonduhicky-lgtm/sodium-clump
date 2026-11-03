// HEAD adapter for break_block
// Compiled into the game by horizon-mixins.

use crate::game::sodium_clump;

pub fn on_break(pos: impl std::fmt::Debug) {
    sodium_clump::on_block_break_attempt(format!("{:?}", pos));
}
