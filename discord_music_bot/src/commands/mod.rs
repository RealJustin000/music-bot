
pub mod join;
pub mod play;

use serenity::framework::standard::macros::group;

#[group]
#[commands(join, play)]
pub struct General;
