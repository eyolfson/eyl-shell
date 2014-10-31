# Eyl Shell

A basic Weston desktop shell client.

## Notes

I forked the Weston code for the initial commit, with no understanding how it
works. This is a toy project to see how Weston and Wayland work. I redid the
build system in CMake and stripped away anything that didn't seem releated to
the desktop shell client. I will likely write additional code in C++14. It seems
Weston is almost in a usable state, except on HiDPI displays. That's likely the
first thing I'll hack around on.

There is a proof of concept tiling style shell for Weston [here]
(http://lists.freedesktop.org/archives/wayland-devel/2014-May/014980.html).
