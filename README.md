# Trapify

Draft of a trap-based survival game we are trying to make.

## Testing it out

Clone the repository locally and do
```
cargo run
```
[In theory](https://comfyengine.org/book/releasing/) it should also be possible to use `trunk serve`, but [WASM has FPS issues](https://github.com/darthdeus/comfy/issues/100).

UPDATE: `trunk serve` now works using a hack. Also, the game is available to play on [github-pages](https://cedeerwe.github.io/trapify/). Note that it is currently optimized for PC, not for mobile.

## TODO:

### v0.1
- [x] Create a tilemap
- [x] Have enemies randomly spawn and move through the map
- [x] Have enemies deal damage to player once they arrive to the end
- [x] Game over screen with restart button
- [x] Possibility to pause the game -- using button or "P" key
- [x] Create a UI for the enemy spawner for testing purposes
- [x] Make it possible to select a tile
- [x] Allow to build traps on the selected tiles
- [x] Introduce Simple trap -- deals X damage periodically on the given tile
- [x] Introduce DOT trap -- applies a damage over time effect
- [x] Make traps visually show when they trigger
- [x] Introduce gold -- traps cost gold to build and player receives gold for slaying enemies
- [x] Introduce Slow trap -- applies slow effect in an area
- [x] Count how long the player lasted and show it in the game over screen
- [x] Income based on current gold
- [x] Have enemies become progressively stronger
- [ ] Separate two game modes -- sandbox & normal
- [ ] Adjust parameters to be able to play the actual game
### v0.2
- [ ] Make individual traps upgradeable for gold
- [ ] Introduce global purchasable global updates for traps
- [ ] More traps: %HP dmg, wall, laser wall, shooters, buffers, one-time traps
### v0.3
- [ ] Traps layed in specific patterns can have special effects
- [ ] Configurable triggering of traps
### v0.4
- [ ] Introduce loot -- content which isn't available on every run

### Other stuff
- [x] Fix Wasm version to not have FPS ~1000 and be playable: HACKED
- [ ] Fix resolutions / UI to work on mobile web as well