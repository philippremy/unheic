// This file is part of UnHEIC
// Copyright © 2026 - Present Philipp Remy.
// This Source Code Form is subject to the terms of the GNU General Public
// License v3.0 only, if a copy of the MPL was not distributed with this file,
// You can obtain one at https://www.gnu.org/licenses/gpl-3.0.html.

#ifndef UNHEIC_UI_API_TYPES_INL
#define UNHEIC_UI_API_TYPES_INL

#include <format>

/* BEGIN C LINKAGE */

UNHEIC_START_CPP
UNHEIC_END_CPP

/* END C LINKAGE */
/* BEGIN C++ LINKAGE */

template <>
struct std::formatter<UnHEICBridgeResult> : std::formatter<std::string_view>
{
    auto format(UnHEICBridgeResult r, format_context& ctx) const
    {
        std::string_view name = "Unknown";

        switch (r) {
            case UnHEICBridgeResult::Success:                         name = "Success"; break;
            case UnHEICBridgeResult::ReadingInputImageFailed:         name = "ReadingInputImageFailed"; break;
            case UnHEICBridgeResult::InvalidUnHEICCXXImageHandle:     name = "InvalidUnHEICCXXImageHandle"; break;
            case UnHEICBridgeResult::ImageNoInterleavedChannel:       name = "ImageNoInterleavedChannel"; break;
            case UnHEICBridgeResult::InvalidImageDimensions:          name = "InvalidImageDimensions"; break;
        }

        return std::formatter<std::string_view>::format(name, ctx);
    }
};

/* END C++ LINKAGE */

#endif
