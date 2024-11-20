# PreciseFlex 400/3400 Series SDK (_Unofficial_)

This was written as a personal project to get more familiar with the Rust language. Yes, it could have been done far more easily in Python but where's the fun in that?

## Testing
Currently, all tcs_client.rs and pflex.rs tests require a physical robot. Tests require the ROBOT_IP env variable to be set prior running tests.

Here's how to run the _pflex_test::check_vitals_ test:
```bash
ROBOT_IP=10.5.2.12 cargo test -p pflex-module-rs check_vitals
```

The remaining tests within pflex_test.rs still have to be properly tested and developed upon.
