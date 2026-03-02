// This file is part of UnHEIC
// Copyright © 2026 - Present Philipp Remy.
// This Source Code Form is subject to the terms of the GNU General Public
// License v3.0 only, if a copy of the MPL was not distributed with this file,
// You can obtain one at https://www.gnu.org/licenses/gpl-3.0.html.

#include <cstdint>
#include <ui_api.hxx>
#include <ui_api_types.hxx>
#include <globals.hxx>
#include <utils.hxx>

#include <libUnHEIC.hxx>

#include <libheif/heif.h>

#include <cstdlib>
#include <format>

/* BEGIN C linkage */

UNHEIC_START_CPP

UnHEICBridgeResult
heic_read_input_image(const char* path,
                      UnHEICCXXImageHandle& out_handle)
{
    cxx_debug("UnHEICBridgeResult read_input_heic_image(const char*, UnHEICCXXImageHandle*)");

    heif_image_handle* handle;
    heif_image* image;
    heif_error* error = nullptr;

    LibHEIFContext::instance().acquire_decoding_ctx([&](heif_context* ctx) {
        cxx_debug("Reading HEIF file from path with heif_context*");
        heif_error err = heif_context_read_from_file(ctx, path, nullptr /* Options currently don't exist */);
        if(err.code != heif_error_Ok)
            goto handle_err;

        cxx_debug("Retrieving primary image handle with heif_context*");
        err = heif_context_get_primary_image_handle(ctx, &handle);
        if(err.code != heif_error_Ok)
            goto handle_err;

        cxx_debug("Attempting to decode HEIF image...");
        err = heif_decode_image(handle, &image, heif_colorspace_RGB, heif_chroma_interleaved_RGBA, nullptr /* No modifications */);
        if(err.code != heif_error_Ok)
            goto handle_err;

        // Store output parameters
        out_handle.image_handle = handle;
        out_handle.image = image;

        return;

handle_err:
        error = static_cast<heif_error*>(malloc(sizeof(heif_error)));
        memcpy(static_cast<void*>(error), static_cast<void*>(&err), sizeof(heif_error));
        return;
    });

    if(error) {
        cxx_error(std::format("Error in read_input_heic_image: {} ({})", error->message, static_cast<int>(error->code)).c_str());
        free(error);
        return UnHEICBridgeResult::ReadingInputImageFailed;
    }

    cxx_debug("read_input_heic_image executed successfully");
    return UnHEICBridgeResult::Success;
}

UnHEICBridgeResult
heic_get_image_dimensions(const UnHEICCXXImageHandle& image_handle,
                          size_t& width,
                          size_t& height)
{
    UnHEICBridgeResult result = UnHEICBridgeResult::Success;

    int w = -1, h = -1;

    if(!image_handle.image_handle || !image_handle.image) {
        result = UnHEICBridgeResult::InvalidUnHEICCXXImageHandle;
        goto handle_err;
    }

    w = heif_image_get_width(image_handle.image, heif_channel::heif_channel_interleaved);
    h = heif_image_get_height(image_handle.image, heif_channel::heif_channel_interleaved);

    if(h == -1 || w == -1) {
        result = UnHEICBridgeResult::ImageNoInterleavedChannel;
        goto handle_err;
    }

    width = w;
    height = h;

    return result;

handle_err:
    cxx_error(std::format("Error in heic_image_dimensions: {}", result).c_str());
    return result;
}

UnHEICBridgeResult
heic_get_image_data_rgba(const UnHEICCXXImageHandle& image_handle,
                         const size_t& width,
                         const size_t& height,
                         size_t& stride,
                         const uint8_t*& ro_rgba_data)
{
    UnHEICBridgeResult result = UnHEICBridgeResult::Success;

    size_t w, h, s;
    const uint8_t* rgba_d = nullptr;

    if(!image_handle.image_handle || !image_handle.image) {
        result = UnHEICBridgeResult::InvalidUnHEICCXXImageHandle;
        goto handle_err;
    }

    result = heic_get_image_dimensions(image_handle, w, h);
    if(result != UnHEICBridgeResult::Success)
        goto handle_err;

    if(w != width || h != height) {
        result = UnHEICBridgeResult::InvalidImageDimensions;
        goto handle_err;
    }

    rgba_d = heif_image_get_plane_readonly2(image_handle.image, heif_channel::heif_channel_interleaved, &s);
    if(!rgba_d) {
        result = UnHEICBridgeResult::ImageNoInterleavedChannel;
        goto handle_err;
    }

    stride = s;
    ro_rgba_data = rgba_d;

    return result;

handle_err:
    cxx_error(std::format("Error in get_image_data_rgba: {}", result).c_str());
    return result;
}

UnHEICBridgeResult
heic_get_bytes_per_pixel(const UnHEICCXXImageHandle& image_handle,
                         size_t& nb_bytes)
{
    UnHEICBridgeResult result = UnHEICBridgeResult::Success;

    int bbp = -1;

    if(!image_handle.image_handle || !image_handle.image) {
        result = UnHEICBridgeResult::InvalidUnHEICCXXImageHandle;
        goto handle_err;
    }

    bbp = heif_image_get_bits_per_pixel(image_handle.image, heif_channel::heif_channel_interleaved);
    if(bbp == -1) {
        result = UnHEICBridgeResult::ImageNoInterleavedChannel;
        goto handle_err;
    }

    nb_bytes = bbp / 8;

    return result;

handle_err:
    cxx_error(std::format("Error in bytes_per_pixel: {}", result).c_str());
    return result;
}

UNHEIC_END_CPP

/* END C linkage */
/* BEGIN C++ linkage */
/* END C++ linkage */
