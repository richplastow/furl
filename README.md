# furl

__Playing with WebGL's ANGLE_instanced_arrays, in a custom Rust-based parametric design app.__

<https://richplastow.com/furl>

<https://github.com/richplastow/furl>


## License

Standard MIT License. See the ‘LICENSE’ file for details.


## Install Build Dependencies

1. Check your node version:  
   `node --version # should be 'v14.16.1'`
2. Check your wasm-pack version:  
   `wasm-pack --version # should be 'wasm-pack 0.9.1'`
3. Check your (global) static-server version:  
   `static-server --version # should be 'static-server@2.2.1'`


## Develop and Build

To work on this project, run:  
`node develop.js`

The `develop.js` script will:  
1. Check that your current working directory is the root of the project
2. Copy the `LICENSE` and `VERSION` files from project root to ‘docs/’
3. Immediately build ‘src/frw.rs’ to ‘docs/lib/wasm/v0/’
4. Start watching ‘src/’
   — it triggers a build whenever anything changes
5. Start a server on http://localhost:9080/ and open a browser window

There’s no automatic browser refresh when code changes. You’ll need to
manually refresh browser to load changes.


## Screengrab Encoding

- Input the file produced by QuickTime Player screen recording
- `libx264` for mp4
- `-crf 12` to set quality, where 50 is lowest and 0 is lossless
- Resample 60fps to 30fps
- Trim a couple of seconds from the start of the recording
- Resample 2048x1252 to half size
- Output an mp4 file to ~/Work/2021-Work/Loop.Coop/frw-screengrabs

```bash
ffmpeg -i "Screen Recording 2021-XX-XX at XX.XX.XX.mov" \
-c:v libx264 \
-crf 12 \
-filter:v "fps=fps=30" \
-ss 2 \
-s 1024x626 \
-c:a copy 202XXXX-frw-xxxx.mp4
```


## Further Reading

__*WebGL Optimization - Instanced Drawing*__  
Code examples and live demos explaining WebGL’s ‘instanced drawing’ feature.  
‘Drawing Multiple Things’ on the same site may also be useful.  
Gregg Tavares, Nov 5th 2019  
<https://webglfundamentals.org/webgl/lessons/webgl-instanced-drawing.html>

__*Rendering 100k spheres, instantiating and draw calls*__  
An easy-to-follow explanation of instanced geometries and animating in shaders.  
Daniel Velasquez, March 8th 2020  
<https://velasquezdaniel.com/blog/rendering-100k-spheres-instantianing-and-draw-calls/>  
Here’s the final demo: <https://4oxuu.csb.app/>

__*Into Vertex Shaders*__  
Creating high-performance WebGL animation systems, by the author of Three.bas.  
Szenia Zadvornykh, June 18th 2017  
<https://medium.com/@Zadvorsky/into-vertex-shaders-594e6d8cd804>  
Here’s the Catmull-Rom Spline demo: <https://codepen.io/zadvorsky/full/brporW>

__*`ANGLE_instanced_arrays`*__  
MDN Web Docs for the extension which lets WebGL draw the same object many times.  
Not all WebGL extensions have good browser compatibility, but this one’s fine.  
MDN contributors, initial en-us content checkin 15 Sep 2020  
<https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays>

__*Three.bas — THREE Buffer Animation System*__  
Moves animation logic to the vertex shader, for THREE.js projects.  
Szenia Zadvornykh, initial commit 26 Oct 2015  
<https://github.com/zadvorsky/three.bas/wiki>

