To reproduce:

First you'll need to ensure you have `cmake` installed to build some
dependencies.

```sh
$ git clone https://github.com/mutexlox-signal/test-coreaudio-rs.git
$ cd test-coreaudio-rs
# select the speakers you are testing in system settings.
$ cargo run
# You should hear a tone playing for approximately 5 seconds
```
