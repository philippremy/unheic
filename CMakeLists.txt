#
# UnHEIC - An easy to use GUI utility for converting HEIC/HEIF images
# GPLv3-only
#

cmake_minimum_required(VERSION 3.25)

# To be set before the first project call
set(CMAKE_OSX_DEPLOYMENT_TARGET "13.3")

# Enable PIC
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

project(
    UnHEIC
    VERSION "0.0.1"
    LANGUAGES ASM C CXX
)

if(WIN32)
    enable_language(RC)
endif()

# Set C++ standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Include Helpers
list(APPEND CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/src/cmake)
include(ConditionalOption)

# Project options
conditional_option(
    UNHEIC_ENABLE_LTO "Enable Link-Time-Optimization (LTO)"
    IF CMAKE_BUILD_TYPE STREQUAL "Release" OR CMAKE_BUILD_TYPE STREQUAL "RelWithDebInfo"
    THEN ON ELSE OFF
)
conditional_option(
     UNHEIC_WIN_NOCONSOLE "Link as a GUI application on Windows (no console)"
     IF CMAKE_BUILD_TYPE STREQUAL "Release"
     THEN ON ELSE OFF
 )

# Always export compile_commands.json
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Handle options
if(UNHEIC_ENABLE_LTO)
    include(CheckIPOSupported)
    check_ipo_supported(
        RESULT UNHEIC_CAN_LTO
        OUTPUT UNHEIC_WHY_NOT_LTO
        LANGUAGES CXX
    )
    if(NOT UNHEIC_CAN_LTO)
        if(DEFINED UNHEIC_WHY_NOT_LTO)
            message(WARNING "[UnHEIC] Requested LTO, but the compiler/linker does not support it: ${UNHEIC_WHY_NOT_LTO}")
        else()
            message(WARNING "[UnHEIC] Requested LTO, but the compiler/linker does not support it.")
        endif()
    else()
        set(CMAKE_INTERPROCEDURAL_OPTIMIZATION ON)
        message(STATUS "[UnHEIC] LTO enabled.")
    endif()
endif()

if(UNHEIC_WIN_NOCONSOLE)
    set(CMAKE_WIN32_EXECUTABLE ON)
endif()

# Fetch all dependencies (we call ourselves recursively with the new prefix
# path)
if(NOT _UNHEIC_BUILD_INTERNAL_RECURSE)
    include(HandleDependencies)
    ensure_dependencies()
    return()
endif()

# Corrosion
include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/philippremy/corrosion.git
    GIT_TAG "b0cf23e0993b1fd8b1a809bf56d6884be07042c0"
)
FetchContent_MakeAvailable(
    Corrosion
)

# Find dependencies
find_package(libde265 REQUIRED)
find_package(vvdec 3.1.0 REQUIRED)
find_package(AOM REQUIRED)
find_package(JPEG REQUIRED)
find_package(openjpeg REQUIRED)
find_package(libheif REQUIRED)

# Add bridge library
add_library(UnHEICBridge STATIC
    ${CMAKE_SOURCE_DIR}/src/cxx/unheic_bridge/include/globals.hxx
    ${CMAKE_SOURCE_DIR}/src/cxx/unheic_bridge/include/ui_api.hxx
    ${CMAKE_SOURCE_DIR}/src/cxx/unheic_bridge/include/ui_api_types.hxx
    ${CMAKE_SOURCE_DIR}/src/cxx/unheic_bridge/include/utils.hxx
    ${CMAKE_SOURCE_DIR}/src/cxx/unheic_bridge/globals.cxx
    ${CMAKE_SOURCE_DIR}/src/cxx/unheic_bridge/ui_api.cxx
    ${CMAKE_SOURCE_DIR}/src/cxx/unheic_bridge/ui_api_types.cxx
)
target_include_directories(UnHEICBridge PUBLIC ${CMAKE_SOURCE_DIR}/src/cxx/unheic_bridge/include)
target_link_libraries(UnHEICBridge PUBLIC heif)
# Add the generated output directory for cbindgen headers
# THIS ONLY APPLIES TO THE LIBRARY!
# It expects that the Rust library will eventually be linked into the binary!
target_include_directories(UnHEICBridge
    PRIVATE ${CMAKE_BINARY_DIR}/corrosion_generated/cbindgen/unheic/include
)
target_compile_options(UnHEICBridge PRIVATE
  $<$<CXX_COMPILER_ID:MSVC>:/W4 /WX>
  $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:-Wall -Wextra -Wpedantic -Werror>
)

# Import unheic from Rust
corrosion_import_crate(
    MANIFEST_PATH ${CMAKE_SOURCE_DIR}/Cargo.toml
    CRATE_TYPES "staticlib"
    IMPORTED_CRATES UNHEIC_RUST_LIBS
)

# Create bindings from Rust
corrosion_experimental_cbindgen(
    TARGET unheic
    HEADER_NAME libUnHEIC.hxx
)

# Define main executable
add_executable(UnHEIC
    ${CMAKE_SOURCE_DIR}/src/cxx/main.cxx
)
# Link to required targets
target_link_libraries(UnHEIC
    PRIVATE ${UNHEIC_RUST_LIBS}
    PRIVATE UnHEICBridge
)
# Add the generated output directory for cbindgen headers
target_include_directories(UnHEIC
    PRIVATE ${CMAKE_BINARY_DIR}/corrosion_generated/cbindgen/unheic/include
)
target_compile_options(UnHEICBridge PRIVATE
  $<$<CXX_COMPILER_ID:MSVC>:/W4 /WX>
  $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:-Wall -Wextra -Wpedantic -Werror>
)

# Platform specific linking
if(APPLE)
    target_link_libraries(UnHEIC
        PRIVATE
            "-framework AppKit"
            "-framework Carbon"
            "-framework CoreFoundation"
            "-framework CoreGraphics"
            "-framework CoreVideo"
            "-framework Foundation"
            "-framework Metal"
            "-framework QuartzCore"
            "-framework Security"
    )
else(WIN32)
    target_sources(UnHEIC PRIVATE ${CMAKE_SOURCE_DIR}/res/windows/UnHEIC.rc)
    set_source_files_properties(${CMAKE_SOURCE_DIR}/res/windows/UnHEIC.rc PROPERTIES LANGUAGE RC)
endif()

# Create Bundles
if(APPLE)
    set(APPICON_MACOS ${CMAKE_SOURCE_DIR}/res/icons/app_icons/AppIcon.icns)
    target_sources(UnHEIC PRIVATE ${APPICON_MACOS})
    set_source_files_properties(${APPICON_MACOS} PROPERTIES MACOSX_PACKAGE_LOCATION "Resources")
    set_target_properties(UnHEIC PROPERTIES MACOSX_BUNDLE ON)
    set_target_properties(UnHEIC PROPERTIES
        BUNDLE ON
        MACOSX_BUNDLE_EXECUTABLE_NAME UnHEIC
        MACOSX_BUNDLE_BUNDLE_NAME UnHEIC
        MACOSX_BUNDLE_LONG_VERSION_STRING "${CMAKE_PROJECT_VERSION}"
        MACOSX_BUNDLE_SHORT_VERSION_STRING "${CMAKE_PROJECT_VERSION}"
        MACOSX_BUNDLE_BUNDLE_VERSION "${CMAKE_PROJECT_VERSION}"
        MACOSX_BUNDLE_INFO_STRING "A handy GUI for converting HEIC/HEIF images to other formats"
        MACOSX_BUNDLE_GUI_IDENTIFIER "de.philippremy.UnHEIC"
        MACOSX_BUNDLE_COPYRIGHT "© Philipp Remy 2026 - Present. Licensed under GPLv3-only."
        MACOSX_BUNDLE_ICON_FILE AppIcon
        MACOSX_BUNDLE_INFO_PLIST ${CMAKE_SOURCE_DIR}/src/cmake/templates/Info.plist.in
    )
endif()
