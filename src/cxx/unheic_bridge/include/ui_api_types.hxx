// This file is part of UnHEIC
// Copyright © 2026 - Present Philipp Remy.
// This Source Code Form is subject to the terms of the GNU General Public
// License v3.0 only, if a copy of the MPL was not distributed with this file,
// You can obtain one at https://www.gnu.org/licenses/gpl-3.0.html.

/**
  * This file contains the C API which can be called from Rust to use various
  * utility functions written in C++.
 */

#ifndef UNHEIC_UI_API_TYPES_HXX
#define UNHEIC_UI_API_TYPES_HXX

#include <utils.hxx>

/* BEGIN C LINKAGE */

/* Any foreign types must be forward declared here */

struct heif_image_handle;
struct heif_image;

UNHEIC_START_CPP

/**
  * @brief The Result type the API returns for giving feedback about the success
  * or error of a called function
 */
typedef enum UnHEICBridgeResult {
    Success = 1 << 0,                               /// The function finished executing successfully
    ReadingInputImageFailed = 1 << 1,               /// The HEIC/HEIF input could not be read
    InvalidUnHEICCXXImageHandle = 1 << 2,           /// The UnHEICCXXImageHandle is invalid or not initialized
    ImageNoInterleavedChannel = 1 << 3,             /// The HEIC/HEIF image has no interleaved channel to read from
    InvalidImageDimensions = 1 << 4,                /// The passed input width and height did not match the actual image dimensions
} UnHEICBridgeResult;

/**
  * @brief An opaque handle for Rust to temporarily take ownership of data
  * which is handled by C++
  *
  * @warning This type needs a manual Rust Drop implementation, because it has
  * a C++ destructor.
 */
typedef struct UnHEICCXXImageHandle {
    heif_image_handle* image_handle;        /// The internal libheif image handle. Needs to be released on destruction.
    heif_image* image;                      /// The internal libheif image. Needs to be released on destruction.
    UnHEICCXXImageHandle();                 /// The constructor which initializes an empty handle
    ~UnHEICCXXImageHandle();                /// The destructor which frees any associated data
} UnHEICCXXImageHandle;

UNHEIC_END_CPP

/* END C LINKAGE */
/* BEGIN C++ LINKAGE */

/* END C++ LINKAGE */

/* Inline definitions */
#include <ui_api_types.inl>

#endif
