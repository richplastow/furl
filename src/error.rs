/// If true, `gl.get_error()` runs after most gl calls during App initialisation.  
/// This is the ‘safe’ way to start up, with a barely noticable performance hit.
/// @TODO is it true that it’s barely noticable?
/// @TODO if not ‘barely noticable’, then in production, after these checks have
/// passed for a given build, store that they passed in localStorage, and then
/// bypass the checks from then on — we know that device’s GPU runs that build.
pub const SLOWLY_GET_ERROR_COLD_PATH: bool = true;

/// Similar to the cold path, but used during Scene initialisation.
pub const SLOWLY_GET_ERROR_COOL_PATH: bool = true;

/// If true, `gl.get_error()` runs after most gl calls during state update.  
/// These generally follow user interactions which change what WebGL renders.  
/// Likely to add noticable jank/stalling, so best avoided in production code.  
/// Probably keep this set to `true` during development, but `false` during
/// performance optimisation and benchmarking.
#[allow(dead_code)]
pub const SLOWLY_GET_ERROR_TEPID_PATH: bool = true;

/// If true, `gl.get_error()` runs after most gl calls during the render phase.  
/// Will cause constant noticable jank/stalling, so only set to `true` while
/// actively debugging WebGL `render()` issues.
pub const SLOWLY_GET_ERROR_WARM_PATH: bool = false;

// @TODO check that other slow WebGL methods, eg `get_parameter()` and `finish()`
// aren’t being called — or at least, put them behind a feature flag, like
// SLOWLY_GET_PARAMETER_DURING_INIT


/// Each of the app’s possible errors has a unique code.
#[derive(Debug)]
pub enum ERROR {
    /// R11006 RENDERER ERROR: uniform location not found
    R11006,
    // /// R11279 RENDERER ERROR: failed to create buffer
    // R11279,
    /// R11331 RENDERER ERROR: error creating program
    R11331,
    /// R11418 RENDERER ERROR: error creating program object
    R11418,
    /// R11530 RENDERER ERROR: error compiling vertex shader
    R11530,
    /// R11572 RENDERER ERROR: error compiling fragment shader
    R11572,
    /// R11820 RENDERER ERROR: canvas_id not found
    R11820,
    /// R11833 RENDERER ERROR: canvas_id not a canvas element
    R11833,
    /// R11872 RENDERER ERROR: error creating shader
    R11872,
    /// R11982 RENDERER ERROR: unable to get shader info log
    R11982,

    /// R22860 RKCOLD ERROR: invalid enum MAX_VERTEX_ATTRIBS
    R22860,
    /// R22863 RKCOLD ERROR: MAX_VERTEX_ATTRIBS is too small
    R22863,
    /// R22870 RKCOLD ERROR: unable to get extension
    R22870,
}

/// Returns a description about an error.
/// Be aware of the RegExp below `<pre id="wasm-panic"></pre>` in index.html.
/// Usage: `oooops().expect(e(E::R12345))`
pub fn error_to_string(error: ERROR) -> &'static str {
    match error {
        ERROR::R11006 => "R11006 RENDERER ERROR: uniform location not found",
        // ERROR::R11279 => "R11279 RENDERER ERROR: failed to create buffer",
        ERROR::R11331 => "R11331 RENDERER ERROR: error creating program",
        ERROR::R11418 => "R11418 RENDERER ERROR: error creating program object",
        ERROR::R11530 => "R11530 RENDERER ERROR: error compiling vertex shader",
        ERROR::R11572 => "R11572 RENDERER ERROR: error compiling fragment shader",
        ERROR::R11820 => "R11820 RENDERER ERROR: canvas_id not found",
        ERROR::R11833 => "R11820 RENDERER ERROR: canvas_id not a canvas element",
        ERROR::R11872 => "R11872 RENDERER ERROR: error creating shader",
        ERROR::R11982 => "R11982 RENDERER ERROR: unable to get shader info log",

        ERROR::R22860 => "R22860 RKCOLD ERROR: invalid enum MAX_VERTEX_ATTRIBS",
        ERROR::R22863 => "R22863 RKCOLD ERROR: MAX_VERTEX_ATTRIBS is too small",
        ERROR::R22870 => "R22870 RKCOLD ERROR: unable to get extension",
    }
}
