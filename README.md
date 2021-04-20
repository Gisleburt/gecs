GECs - Gisleburt's Electronic Components
========================================

A collection of wrappers for electronic components using [embedded-hal] interfaces.

ToDo:
-----

- [ ] Come up with a better name / is this name ok?
- [ ] Think about how to make a common interface for things like displays. Current 8 segment interface doesn't
      work for things that need to be switched, eg a 4 digit 8 segment display.
- [ ] One project with features or multiple features?

[embedded-hal]: https://crates.io/crates/embedded-hal

Features:
---------

To use a component, in addition to including the library, you'll need to activate its feature. For example:

For example, to use the 5161bs LEDs:

```toml
[dependencies]
gecs = { version = "*", features = ["5161bs"] }
```

This will export the `DisplayLed5161bs` struct that will allow you to use the 5161bs.

### List of features and exports:

- 5161bs
  - DisplayLed5161bs, SevenSegmentDp, DigitalPinState
