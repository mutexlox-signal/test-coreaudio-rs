To reproduce:

```sh
$ git clone https://github.com/mozilla/cubeb-rs.git
$ git clone https://github.com/mutexlox-signal/test-coreaudio-rs.git
$ cd test-coreaudio-rs
# select the built-in MacBook Pro microphone as default in system settings
$ cargo run
# Sound may be very quiet
# select the any other microphone as default in system settings
$ cargo run
# Sound should be normal
```
