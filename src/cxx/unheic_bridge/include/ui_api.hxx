// This file is part of UnHEIC
// Copyright © 2026 - Present Philipp Remy.
// This Source Code Form is subject to the terms of the GNU General Public
// License v3.0 only, if a copy of the MPL was not distributed with this file,
// You can obtain one at https://www.gnu.org/licenses/gpl-3.0.html.

/**
  * This file contains the C API which can be called from Rust to use various
  * utility functions written in C++.
 */

#ifndef UNHEIC_UI_API_HXX
#define UNHEIC_UI_API_HXX

#include <ui_api_types.hxx>
#include <utils.hxx>

#include <cstddef>
#include <cstdint>

UNHEIC_START_CPP

/**
  * @brief Attempts to read an input HEIF/HEIC file, decode it and store it in
  *        the provided opaque handle
  *
  * @param [in] path The path to the HEIC/HEIF file
  * @param [out] out_handle A pre-allocated handle for storing the resulting
  *              image data. Only for use within C++ and therefore opaque to
  *              Rust.
  *
  * @note This function is safe to be called from any thread.
  *
  * @return A status code indicating whether the function executed successfully
 */
UnHEICBridgeResult
heic_read_input_image(const char* path,
                      UnHEICCXXImageHandle& out_handle);

/**
  * @brief Gets the image dimensions of a previously read HEIC/HEIF image
  *
  * @param [in] image_handle The handle to the previously read image
  * @param [out] width The width of the image in pixels
  * @param [out] height The height of the image in pixels
  *
  * @note This function is safe to be called from any thread.
  *
  * @return A status code indicating whether the function executed successfully
 */
UnHEICBridgeResult
heic_get_image_dimensions(const UnHEICCXXImageHandle& image_handle,
                          size_t& width,
                          size_t& height);

/**
  * @brief Gets the image RGBA data of a previously read HEIC/HEIF image as
  *        read-only bytes
  *
  * @param [in] image_handle The handle to the previously read image
  * @param [in] width The width of the image in pixels
  * @param [in] height The height of the image in pixels
  * @param [out] stride The stride of the image (i.e, row width in bytes)
  * @param [out] ro_rgba_data A pointer to the read-only RGBA image data
  *
  * @note This function is safe to be called from any thread.
  *
  * @warning This function only succeeds if the passed in width and height
  *          matches the actual image width and height. It serves as a a sanity
  *          check.
  *
  * @warning The ro_rgba_data has a a lifetime that matches the input
  *          const UnHEICCXXImageHandle& image_handle.
  *
  * @return A status code indicating whether the function executed successfully
 */
UnHEICBridgeResult
heic_get_image_data_rgba(const UnHEICCXXImageHandle& image_handle,
                         const size_t& width,
                         const size_t& height,
                         size_t& stride,
                         const uint8_t*& ro_rgba_data);

/**
 * @brief Returns the number of bytes required to store a single pixel of this
 *        input image
 *
 * @param [in] image_handle The handle to the previously read image
 * @param [out] nb_bytes The number of bytes per pixel
 *
 * @note This function is safe to be called from any thread.
 *
 * @return A status code indicating whether the function executed successfully
 */
UnHEICBridgeResult
heic_get_bytes_per_pixel(const UnHEICCXXImageHandle& image_handle,
                         size_t& nb_bytes);

UNHEIC_END_CPP

#endif
