# Space Worm - WASM Edition

A classic Snake-inspired game written in Rust and compiled to WebAssembly for web browsers. 

The player controls a space worm that grows longer as it eats food while avoiding collisions with itself. At scores of 1000, 3000, 6000, and 10000 points, players are eligible to choose special perks that enhance gameplay.

**🌐 Play Online:** [https://hvalfangst.github.io/rust-space-worm-game-wasm/](https://hvalfangst.github.io/rust-space-worm-game-wasm/)

## 🎮 Cross-Platform Gaming

This game is designed to run seamlessly across all modern web browsers in Desktop and mobile environments. It detects the platform and adjusts controls accordingly:
- **Desktop**: Uses keyboard keys WASD for movement and spacebar for perks
- **Mobile**: Touch controls with on-screen buttons for movement and perks



## 🛠️ Technology Stack

### Core Technologies
- **Rust**: Game logic and core systems
- **WebAssembly (WASM)**: High-performance web execution
- **JavaScript**: Web integration and audio management
- **HTML5 Canvas**: Graphics rendering
- **Web Audio API**: Sound and music playback

### Dependencies
```toml
[dependencies]
image = { version = "0.24", default-features = false, features = ["png", "jpeg"] }
rand = "0.8"
wasm-bindgen = "0.2.100"
js-sys = "0.3.77"
wasm-bindgen-futures = "0.4.50"
console_error_panic_hook = "0.1"
getrandom = { version = "0.2", features = ["js"] }

[dependencies.web-sys]
version = "0.3.77"
features = [
  "console", "CanvasRenderingContext2d", "Document", "Element",
  "HtmlCanvasElement", "HtmlElement", "HtmlImageElement", "Window",
  "AudioContext", "AudioDestinationNode", "AudioBuffer",
  "AudioBufferSourceNode", "GainNode", "AudioParam",
  "CssStyleDeclaration", "Response", "RequestInit", "RequestMode",
  "Request", "KeyboardEvent", "MouseEvent", "EventTarget",
  "Event", "ImageData"
]
```

## 🚀 Development Setup

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) for WASM compilation
- [Python](https://www.python.org/downloads/) (for local development server)

### Local Development

To start the game locally, use the [start-game](start-game.sh) script,
which will build the project and serve it using a local HTTP server. Do note that one
must pass the `build` argument to aforementioned script in order to actually build it:

```bash
./start-game.sh build
```


The default behavior is to just serve the already built WASM, which comes in handy if changes are only present in the
[index](www/index.html) file.

Once served, one
can play the game in your favourite browser at http://localhost:3000/www. 







## 🔄 CI/CD & Deployment

### GitHub Actions Workflow
Automated deployment to **GitHub Pages** using  script [deploy.yml](.github/workflows/deploy.yml), 
which does the following on pushes to main:
1. Sets up the Rust toolchain with the `wasm32-unknown-unknown` target
2. Installs `wasm-pack` for WASM compilation
3. Builds the project using `cargo build` and `wasm-pack`
4. Copies assets and creates a `.nojekyll` file
5. Deploys to GitHub Pages using the official actions


## 🎨 Assets

All game assets are original creations by me, [Hichael](https://www.youtube.com/watch?v=BSDYR7CT1Ic).

### Visual Assets
- **Sprites**: Created with [Aseprite](https://www.aseprite.org/)
- **Snake Parts**: Head, body, tail with animation frames
- **Environment**: Animated starfield background, rotating planet
- **UI Elements**: Game over screen, perk selection interface

### Audio Assets
- **Music**: Composed in [Ableton Live](https://www.ableton.com/)
- **Sound Effects**: Custom-created audio for game events
- **Files**: `eat.mp3`, `new_perk.mp3`, `music_0.mp3`


## 📜 License

This project is open source. Feel free to use, modify, and distribute according to your needs.

