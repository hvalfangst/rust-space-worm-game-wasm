# Refactor Rust Project for WASM Support

Systematically convert this Rust project to support WebAssembly deployment while maintaining desktop compatibility.

## Analysis Phase

1. **Analyze Current Project Structure**
   - Examine `Cargo.toml` dependencies (especially `rodio` and other desktop-only crates)
   - Identify platform-specific code (file I/O, audio, input handling)
   - Review game logic that can be shared between platforms
   - Check for any native system calls or desktop-only APIs

2. **Identify Refactoring Scope**
   - List all modules that need WASM adaptation
   - Identify shared game logic that can remain unchanged
   - Plan directory structure for platform-specific code

## Cargo Configuration

3. **Update Cargo.toml for Dual Platform Support**
   - Add `crate-type = ["cdylib", "rlib"]` for WASM support
   - Create conditional dependencies for desktop vs WASM
   - Add WASM-specific dependencies (wasm-bindgen, js-sys, web-sys)
   - Configure build targets for both platforms

## Audio System Refactoring

4. **Replace Rodio with Platform-Agnostic Audio**
   - Create audio trait for abstraction
   - Implement Web Audio API for WASM target
   - Keep rodio implementation for desktop target
   - Convert preloaded audio system to work with both platforms
   - Update MusicId and SfxId enums to work with WASM

## Platform Abstraction Layer

5. **Create Platform-Specific Modules**
   - Create `src/platform/` directory structure
   - Implement conditional compilation for desktop vs WASM
   - Abstract input handling for web events vs desktop input
   - Abstract file loading (fetch API vs filesystem)

## WASM Entry Point

6. **Create WASM-Specific Entry Point**
   - Create `src/lib.rs` as WASM entry point
   - Implement `#[wasm_bindgen]` exports for game functions
   - Set up game loop compatible with `requestAnimationFrame`
   - Add WASM-specific initialization and cleanup

## Web Deployment Structure

7. **Set Up Web Deployment Files**
   - Create `www/` directory for web assets
   - Generate `index.html` with canvas and WASM loading
   - Create JavaScript glue code for WASM interaction
   - Set up asset loading for web deployment

## Build Scripts and Tooling

8. **Create Build Scripts**
   - `scripts/build-wasm.sh` for WebAssembly builds
   - `scripts/build-desktop.sh` for native builds  
   - `scripts/serve.sh` for local development server
   - `scripts/deploy.sh` for web deployment

## Game State Management

9. **Refactor Game State for Platform Independence**
   - Ensure game logic is platform-agnostic
   - Update audio calls to use the new abstracted interface
   - Handle timing differences between platforms
   - Manage asset paths for both file system and web URLs

## Error Handling and Compatibility

10. **Add Robust Error Handling**
    - Handle WASM-specific errors (JsValue)
    - Add fallbacks for unsupported browser features
    - Implement graceful degradation for audio/input
    - Add debugging utilities for both platforms

## Testing and Validation

11. **Create Testing Strategy**
    - Verify desktop build still works
    - Test WASM build in multiple browsers
    - Validate audio playback in web environment
    - Check performance on various devices

## Documentation Updates

12. **Update Documentation**
    - Update README.md with build instructions for both platforms
    - Document platform differences and limitations
    - Add deployment instructions for web hosting
    - Include troubleshooting guide for common issues

## Implementation Guidelines

- **Preserve existing game logic** - only modify platform-specific code
- **Use feature flags** for conditional compilation (`#[cfg(target_arch = "wasm32")]`)
- **Maintain API compatibility** - same function signatures across platforms
- **Optimize for web** - consider bundle size and loading performance
- **Test incrementally** - ensure each change doesn't break existing functionality

## File Structure Target

```
src/
├── lib.rs              # WASM entry point
├── main.rs             # Desktop entry point  
├── game/               # Shared game logic (unchanged)
│   ├── mod.rs
│   ├── state.rs
│   ├── player.rs
│   └── world.rs
├── audio/              # Platform-abstracted audio
│   ├── mod.rs          # Common interface
│   ├── desktop.rs      # Rodio implementation
│   └── web.rs          # Web Audio implementation
├── input/              # Platform-abstracted input
│   ├── mod.rs
│   ├── desktop.rs
│   └── web.rs
└── platform/           # Other platform-specific code
    ├── mod.rs
    ├── desktop.rs
    └── web.rs
www/                    # Web deployment
├── index.html
├── style.css
├── pkg/                # Generated WASM files
└── assets/             # Web-optimized assets
scripts/                # Build automation
├── build-wasm.sh
├── build-desktop.sh
├── serve.sh
└── deploy.sh
```

## Success Criteria

- ✅ Desktop build continues to work unchanged
- ✅ WASM build compiles without errors
- ✅ Game runs in web browser with full functionality
- ✅ Audio system works on both platforms
- ✅ Build scripts automate the process
- ✅ Web deployment is ready for hosting

Execute this refactoring systematically, testing each phase before proceeding to the next. Prioritize maintaining the working desktop version while adding WASM support incrementally.