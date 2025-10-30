# Tennis for Two (Bevy prototype)

> Heads up: this description was drafted with help from an AI assistant.

Hey future me (and any curious friends or recruiters 👋). This repo is my place to poke at a Bevy-powered remake of the old "Tennis for Two" idea. It's casual, it's experimental, and it's where I stash notes so I remember what on earth I was trying to do the next time I open the project.

## What's working

* Booting into a simple title sequence that fades through a few placeholder app states (Title → Main Menu → Game → Pause → End).
* A handful of helper systems and resources (`Clock`, `Phase`, `Config`, `globals::lerp`, `despawn_with`) that keep the UI flow moving.

## How to run it

```bash
cargo run
```

The first build will fetch Bevy and friends, so give it a moment. After that you should land right in the title sequence window.

## Notes to myself

* Title screen bits live in `src/screens/title/` if I want to tweak timings or layout.
* Fonts hang out in `assets/fonts`. Anything vibe-y and oscilloscope-esque goes here.
* There's a `draw_coordinates` gizmo system in `src/main.rs` that's currently commented out—flip it on when spatial debugging is needed.

## Loose plans

* Swap the placeholder state swaps for a real menu that responds to keyboard/gamepad input.
* Start roughing in paddle control, ball physics, and net collisions.
* Play with shaders or post-processing to get that soft oscilloscope glow.
* Once gameplay exists, think about local two-player polish and basic scoring.

## License

MIT, see [LICENSE](LICENSE). Asset licenses stay with their original owners.
