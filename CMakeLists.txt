cmake_minimum_required(VERSION 3.0.0 FATAL_ERROR)

project(EylShell)

find_package(PkgConfig)
pkg_check_modules(CAIRO REQUIRED cairo)
pkg_check_modules(GLIB REQUIRED glib-2.0)
pkg_check_modules(PIXMAN REQUIRED pixman-1)
pkg_check_modules(PNG REQUIRED libpng)
pkg_check_modules(WAYLAND_CLIENT REQUIRED wayland-client)
pkg_check_modules(WAYLAND_CURSOR REQUIRED wayland-cursor)
pkg_check_modules(WAYLAND_SCANNER REQUIRED wayland-scanner)
pkg_check_modules(WEBP REQUIRED libwebp)
pkg_check_modules(XKB_COMMON REQUIRED xkbcommon)
find_package(JPEG)

execute_process(
  COMMAND ${PKG_CONFIG_EXECUTABLE}
  wayland-scanner --variable=wayland_scanner
  OUTPUT_VARIABLE WAYLAND_SCANNER_EXECUTABLE
  OUTPUT_STRIP_TRAILING_WHITESPACE
)
macro(wayland_scanner_generate _filename)
    message(STATUS "${WAYLAND_SCANNER_EXECUTABLE}: ${_filename}.xml")
    execute_process(
      COMMAND ${WAYLAND_SCANNER_EXECUTABLE} client-header
      INPUT_FILE src/protocol/${_filename}.xml
      OUTPUT_FILE src/protocol/${_filename}-client-protocol.h
    )
    execute_process(
      COMMAND ${WAYLAND_SCANNER_EXECUTABLE} code
      INPUT_FILE src/protocol/${_filename}.xml
      OUTPUT_FILE src/protocol/${_filename}-client-protocol.c
    )
endmacro()
wayland_scanner_generate(desktop-shell)
wayland_scanner_generate(xdg-shell)
wayland_scanner_generate(text-cursor-position)
wayland_scanner_generate(workspaces)

include(CheckIncludeFiles)
check_include_files(execinfo.h HAVE_EXECINFO_H)
include(CheckFunctionExists)
check_function_exists(mkostemp HAVE_MKOSTEMP)
check_function_exists(posix_fallocate HAVE_POSIX_FALLOCATE)
check_function_exists(strchrnul HAVE_STRCHRNUL)
configure_file(src/client/config.h.in src/client/config.h)

add_definitions(
  -DDATADIR="/usr/share"
  -DBINDIR="/usr/bin"
  -DLIBEXECDIR="/usr/lib/weston"
  -D_GNU_SOURCE
  -DUSE_RESIZE_POOL
)
include_directories(
  ${CAIRO_INCLUDE_DIRS}
  ${GLIB_INCLUDE_DIRS}
)
add_executable(eyl-shell-client
  src/client/desktop-shell.c
  src/client/window.c
  src/client/frame.c
  src/client/image-loader.c
  src/client/os-compatibility.c
  src/client/config-parser.c
  src/client/cairo-util.c
  src/protocol/desktop-shell-client-protocol.c
  src/protocol/text-cursor-position-client-protocol.c
  src/protocol/xdg-shell-client-protocol.c
  src/protocol/workspaces-client-protocol.c
)
target_link_libraries(eyl-shell-client
  ${WAYLAND_CLIENT_LIBRARIES}
  ${XKB_COMMON_LIBRARIES}
  ${WAYLAND_CURSOR_LIBRARIES}
  ${PIXMAN_LIBRARIES}
  ${CAIRO_LIBRARIES}
  ${PNG_LIBRARIES}
  ${WEBP_LIBRARIES}
  ${JPEG_LIBRARIES}
  m
)

INSTALL(TARGETS eyl-shell-client
  RUNTIME DESTINATION bin
)
