# FilmCraft Studio

An open-source, sovereign non-linear video editor (NLE) built in pure Rust, powered by the **Martensite** GPU-accelerated retained-mode GUI engine.

![FilmCraft Studio on Martensite](brag/demo.gif)

## Architecture

- **`crates/ui-martensite`**: Sovereign retained-mode NLE interface built on Martensite's multi-track timeline, J/K/L shuttle scrubbing, and audio meters.
- **`crates/engine`**: Core timeline compositing, frame decoding, and audio-video synchronization.

## Legal & Compliance Notice

FilmCraft is an independent open-source video editor. It is not sponsored, endorsed, or affiliated with Adobe Inc. Adobe, Premiere Pro, and Creative Cloud are trademarks of Adobe Inc. All timeline mechanics, J/K/L shuttle controls, and editing paradigms operate under 17 U.S.C. § 102(b) and *Lotus v. Borland*.

## License

Dual-licensed under MIT OR Apache-2.0.
