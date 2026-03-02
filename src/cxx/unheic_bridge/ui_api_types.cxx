// This file is part of UnHEIC
// Copyright © 2026 - Present Philipp Remy.
// This Source Code Form is subject to the terms of the GNU General Public
// License v3.0 only, if a copy of the MPL was not distributed with this file,
// You can obtain one at https://www.gnu.org/licenses/gpl-3.0.html.

#include <ui_api_types.hxx>
#include <utils.hxx>

#include <libUnHEIC.hxx>

#include <libheif/heif.h>

/* BEGIN C linkage */

UNHEIC_START_CPP
UNHEIC_END_CPP

/* END C linkage */
/* BEGIN C++ linkage */

UnHEICCXXImageHandle::UnHEICCXXImageHandle() {
    cxx_debug("UnHEICCXXImageHandle::UnHEICCXXImageHandle()");
    this->image_handle = nullptr;
    this ->image = nullptr;
}

UnHEICCXXImageHandle::~UnHEICCXXImageHandle() {
    cxx_debug("UnHEICCXXImageHandle::~UnHEICCXXImageHandle()");
    if(this->image)
        heif_image_release(this->image);
    if(this->image_handle)
        heif_image_handle_release(this->image_handle);
}

/* END C++ linkage */
