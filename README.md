To reproduce:

```sh
$ git clone https://github.com/mozilla/cubeb-rs.git
$ git clone https://github.com/mutexlox-signal/test-coreaudio-rs.git
$ cd test-coreaudio-rs
# select the built-in MacBook Pro microphone as default in system settings
# Talk, observing the indicated sound levels in system settings.
$ cargo run
# Sound may be very quiet, and system settings may show very little sound input
$
# talk again after program finishes -- sound levels in system settings should
# be normal.
# select any other microphone as the default in system settings
$ cargo run
# Sound should be normal -- audible in headphones/speakers and indicated at
# normal volumes in system settings.
```
