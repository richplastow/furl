//! WebGL vertex shaders.

// We avoid Linux '/' vs Windows '\' incompatibility.

pub fn box_vert() -> &'static str {
    include_str!("box.vert")
}

pub fn cactus_vert() -> &'static str {
    include_str!("cactus.vert")
}

pub fn guides_vert() -> &'static str {
    include_str!("guides.vert")
}

pub fn furl_basic_vert() -> &'static str {
    include_str!("furl_basic.vert")
}
