// This file is part of UnHEIC
// Copyright © 2026 - Present Philipp Remy.
// This Source Code Form is subject to the terms of the GNU General Public
// License v3.0 only, if a copy of the MPL was not distributed with this file,
// You can obtain one at https://www.gnu.org/licenses/gpl-3.0.html.

#ifndef UNHEIC_GLOBALS_HXX
#define UNHEIC_GLOBALS_HXX

/*
 * This file includes global variables
 * Everything must be synchronized with mutex
 */

#include <utils.hxx>

#include <libheif/heif.h>

#include <functional>
#include <mutex>

UNHEIC_START_CPP

class LibHEIFContext final {

public:
    static LibHEIFContext& instance();

    LibHEIFContext(LibHEIFContext&&) = delete;
    LibHEIFContext(LibHEIFContext&) = delete;
    LibHEIFContext operator=(LibHEIFContext&&) = delete;
    LibHEIFContext operator=(LibHEIFContext&) = delete;

    void acquire_decoding_ctx(std::function<void(heif_context*)> scope_function);

private:
    LibHEIFContext();
    ~LibHEIFContext();

    static LibHEIFContext* _this;
    static std::mutex _mutex;

    heif_context* decoding_ctx;
};

UNHEIC_END_CPP

#endif
