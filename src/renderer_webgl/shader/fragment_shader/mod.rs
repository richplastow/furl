//! WebGL fragment shaders.

// We avoid Linux '/' vs Windows '\' incompatibility.

pub fn blue_frag() -> &'static str {
    include_str!("blue.frag")
}

pub fn red_frag() -> &'static str {
    include_str!("red.frag")
}

pub fn rainbow_frag() -> &'static str {
    include_str!("rainbow.frag")
}

pub fn passthru_frag() -> &'static str {
    include_str!("passthru.frag")
}
