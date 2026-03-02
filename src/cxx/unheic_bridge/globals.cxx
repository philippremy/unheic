// This file is part of UnHEIC
// Copyright © 2026 - Present Philipp Remy.
// This Source Code Form is subject to the terms of the GNU General Public
// License v3.0 only, if a copy of the MPL was not distributed with this file,
// You can obtain one at https://www.gnu.org/licenses/gpl-3.0.html.

#include <globals.hxx>
#include <utils.hxx>

#include <libUnHEIC.hxx>

/* BEGIN C linkage */

UNHEIC_START_CPP
UNHEIC_END_CPP

/* END C LINKAGE */
/* BEGIN C++ linkage */

LibHEIFContext* LibHEIFContext::_this = nullptr;
std::mutex LibHEIFContext::_mutex;

LibHEIFContext& LibHEIFContext::instance() {
    _mutex.lock();

    if(!_this) {
        cxx_debug("Initializing static LibHEIFContext instance...");
        _this = new LibHEIFContext();
    }

    _mutex.unlock();

    return *_this;
}

LibHEIFContext::LibHEIFContext() {
    cxx_debug("LibHEIFContext::LibHEIFContext()");
    this->decoding_ctx = heif_context_alloc();
}

LibHEIFContext::~LibHEIFContext() {
    cxx_debug("LibHEIFContext::~LibHEIFContext()");
    if(this->decoding_ctx)
        heif_context_free(this->decoding_ctx);
}

void LibHEIFContext::acquire_decoding_ctx(std::function<void(heif_context*)> scope_function) {
    cxx_debug("LibHEIFContext: Calling into protected context (LibHEIFContext::acquire_decoding_ctx(std::function<void(heif_context*)>))");
    this->_mutex.lock();
    scope_function(this->decoding_ctx);
    this->_mutex.unlock();
}

/* END C++ linkage */
