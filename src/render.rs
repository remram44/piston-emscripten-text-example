//! Rendering code, using Piston.

use graphics::{self, Context, Graphics, Transformed};
use graphics::character::CharacterCache;
use std::fmt::Debug;

pub fn render<G, C, E>(
    context: Context,
    gl: &mut G,
    glyph_cache: &mut C,
) where
    G: graphics::Graphics,
    E: Debug,
    C: CharacterCache<Texture = <G as Graphics>::Texture, Error = E> + Sized,
{
    graphics::clear([0.0, 0.0, 0.1, 1.0], gl);

    graphics::line(
        [0.0, 1.0, 0.0, 1.0],
        2.0,
        [200.0, 100.0, 600.0, 500.0],
        context.transform,
        gl,
    );
    graphics::line(
        [0.0, 0.0, 1.0, 1.0],
        2.0,
        [600.0, 100.0, 200.0, 500.0],
        context.transform,
        gl,
    );
    graphics::text(
        [1.0, 0.0, 0.0, 1.0],
        24,
        "Game Over!",
        glyph_cache,
        context.transform.trans(150.0, 250.0).scale(5.0, 5.0),
        gl,
    ).unwrap();
}
