# a gemini guestbook

i wanted to write a small guestbook for my gemini capsule

## components

* `./test-server` : gmid(1) test server with chroot
* `./sign-guestbook` : rust cgi program for gemini to append to guestbook
* `./blank-guestbook.gmi` : blank guestbook template

### test server

a test server using gmid(1). since static rust binaries are not available on openbsd (to my knowledge), i wrote `./server/setup-rust.sh` to copy necessary shared-objects into the gemroot. running `set-up-everything.sh` should set up everything. run `serve.sh` as root to start the server.

### sign guestbook

a rust cgi program for gemini to prompt and append a message to the guestbook. it reads a message from the environment variable `QUERY_STRING`, set by gmid(1). note that the path to the guestbook is hardcoded (see `const GUESTBOOK` in `main.rs`). copy the compiled program into your cgi-bin folder.

### blank guestbook

a blank guestbook template. make sure the link to `sign-guestbook` is correct.

