# SigmaOS Screen Recorder

This crate models recorder states and configuration. `start()`, `pause()`,
`resume()`, and `stop()` change in-memory state; stopping returns the configured
output path. It does not capture frames or audio, encode video, or write an
output file. The frame counter therefore does not represent captured frames.

The inline tests cover state transitions only, not media capture or encoding.

## License

GPL-3.0-or-later. See [`LICENSE`](LICENSE) and [`Cargo.toml`](Cargo.toml).
