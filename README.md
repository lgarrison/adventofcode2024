# Advent of Code 2024
Keeping it 🦀

## Usage
To make a new day:
1) `$ cd src; cp -r template dayXX`
1) Put the input in `src/dayXX/input.txt`
1) Add a new `[[bin]]` section to `Cargo.toml`

To run:
- `$ cargo run --bin dayXX`

To run fast:
- `$ cargo run --release --bin dayXX`

## Flatiron VS Code Configuration Note
Getting the `rust-analyzer` VS Code extension to recognize the rust installed in the modules was a huge pain. The nicest solution would be if VS Code would allow you to set per-workspace environment variables (or even source an environment setup script) so that the extension host can pick up the right `PATH`. But if such a mechanism exists, I can't find it.

However, the rust extension does allow you to set arbitrary environment variables, so we can use it to modify the `PATH`. It's not such a bad solution, but the rust devs rightly point out that it's fragile to have to set the rust environment for the analyzer separately from where one sets it up for compilation.

My `settings.json` now has an entry like:
```
    "rust-analyzer.server.extraEnv": {
        "PATH": "/mnt/sw/nix/store/2grrzm5kq84sxpxsfwmnzl60p53l1hn6-rust-1.70.0/bin"
    }
```
