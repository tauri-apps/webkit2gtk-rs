# Update Guide

1. Run `git submodule update --init` first to init the submodules
2. Run `cargo install --path gir` to install `gir`
3. Run `gir -o .` in both `.` and `./sys` to regenerate the binding files
4. Run `git apply fix.patch` to apply the patch for some incorrect bindings
5. Run `cargo fmt` in both `.` and `./sys` to format the files
