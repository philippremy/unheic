macro(ensure_dependencies)

    # We need the following packages:
    #
    # Decoding:
    # libde265 --> (libheif, HEIC decoder)
    # aom --> (libheif, AVIF decoder)
    # vvdec --> (libheif, VVC decoder)
    # jpeg-turbo --> (libheif, JPEG decoder)
    # openjpeg --> (libheif, JPEG-2000 decoder, HTJ2K decoder)
    # libheif --> (HEIC/HEIF decoder)
    #
    # Utils:
    # corrosion --> (import Rust crates)
    #
    # TODO:
    # openh264 --> (no CMake, libheif, AVC decoder)

    # Required for ExternalProject_Add
    include(ExternalProject)

    # Set a default value for the underlying build type
    # if the user did not specify one
    set(UNHEIC_DEPENDENCY_BUILD_TYPE)
    if(NOT DEFINED CMAKE_BUILD_TYPE)
        set(UNHEIC_DEPENDENCY_BUILD_TYPE "Debug")
    else()
        set(UNHEIC_DEPENDENCY_BUILD_TYPE ${CMAKE_BUILD_TYPE})
    endif()

    set(TOOLCHAIN_FLAG)
    if(DEFINED CMAKE_TOOLCHAIN_FILE)
        cmake_path(NORMAL_PATH CMAKE_TOOLCHAIN_FILE OUTPUT_VARIABLE TOOLCHAIN_PATH_NORMALIZED)
        set(TOOLCHAIN_FLAG "-DCMAKE_TOOLCHAIN_FILE=${TOOLCHAIN_PATH_NORMALIZED}")
    endif()

    set(OSX_ARCH_FLAG)
    if(DEFINED CMAKE_OSX_ARCHITECTURES)
        set(OSX_ARCH_FLAG "-DCMAKE_OSX_ARCHITECTURES=${CMAKE_OSX_ARCHITECTURES}")
    endif()

    # Shared CMake configure flags
    set(SHARED_CMAKE_FLAGS
        -DCMAKE_C_FLAGS="-w"
        -DCMAKE_CXX_FLAGS="-w"
        -DBUILD_SHARED_LIBS=OFF
        -DCMAKE_PREFIX_PATH=${CMAKE_BINARY_DIR}/ThirdParty/Artifacts
        -DCMAKE_BUILD_TYPE=${UNHEIC_DEPENDENCY_BUILD_TYPE}
        -DCMAKE_OSX_DEPLOYMENT_TARGET="13.3"
        ${TOOLCHAIN_FLAG}
        ${OSX_ARCH_FLAG}
    )

    ExternalProject_Add(
        libde265
        PREFIX ${CMAKE_BINARY_DIR}/ThirdParty
        TMP_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Temporaries/libde265
        STAMP_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Stamps/libde265
        LOG_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Logs/libde265
        DOWNLOAD_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Downloads/libde265
        SOURCE_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Sources/libde265
        BINARY_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Builds/libde265
        INSTALL_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Artifacts
        GIT_REPOSITORY "https://github.com/strukturag/libde265.git"
        GIT_TAG "v1.0.16"
        GIT_SHALLOW ON
        GIT_PROGRESS ON
        CMAKE_ARGS
            ${SHARED_CMAKE_FLAGS}

            -DENABLE_SDL=OFF
            -DBUILD_FRAMEWORK=OFF
            -DENABLE_DECODER=OFF
            -DENABLE_ENCODER=OFF
            -DBUILD_SHARED_LIBS=OFF

            -DCMAKE_INSTALL_PREFIX=<INSTALL_DIR>
    )

    ExternalProject_Add(
        aom
        PREFIX ${CMAKE_BINARY_DIR}/ThirdParty
        TMP_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Temporaries/aom
        STAMP_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Stamps/aom
        LOG_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Logs/aom
        DOWNLOAD_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Downloads/aom
        SOURCE_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Sources/aom
        BINARY_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Builds/aom
        INSTALL_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Artifacts
        GIT_REPOSITORY "https://aomedia.googlesource.com/aom"
        GIT_TAG "v3.13.1"
        GIT_SHALLOW ON
        GIT_PROGRESS ON
        CMAKE_ARGS
            ${SHARED_CMAKE_FLAGS}

            -DCONFIG_AV1_DECODER=1
            -DCONFIG_AV1_ENCODER=0
            -DCONFIG_LIBYUV=0
            -DCONFIG_PIC=1
            -DCONFIG_SHARED=0
            -DCONFIG_WEBM_IO=0
            -DSTATIC_LINK_JXL=1
            -DENABLE_DOCS=0
            -DENABLE_EXAMPLES=0
            -DENABLE_TESTS=0
            -DENABLE_TOOLS=0

            -DCMAKE_INSTALL_PREFIX=<INSTALL_DIR>
    )

    ExternalProject_Add(
        vvdec
        PREFIX ${CMAKE_BINARY_DIR}/ThirdParty
        TMP_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Temporaries/vvdec
        STAMP_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Stamps/vvdec
        LOG_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Logs/vvdec
        DOWNLOAD_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Downloads/vvdec
        SOURCE_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Sources/vvdec
        BINARY_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Builds/vvdec
        INSTALL_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Artifacts
        GIT_REPOSITORY "https://github.com/fraunhoferhhi/vvdec.git"
        GIT_TAG "v3.1.0"
        GIT_SHALLOW ON
        GIT_PROGRESS ON
        CMAKE_ARGS
            ${SHARED_CMAKE_FLAGS}

            -DVVDEC_ENABLE_BITSTREAM_DOWNLOAD=OFF
            -DVVDEC_ENABLE_LOCAL_BITSTREAM_DOWNLOAD=OFF
            -DVVDEC_INSTALL_VVDECAPP=OFF
            -DVVDEC_LIBRARY_ONLY=ON
            -DBUILD_SHARED_LIBS=OFF
            -DVVDEC_ENABLE_BUILD_TYPE_POSTFIX=OFF
            -DVVDEC_ENABLE_LINK_TIME_OPT=${CMAKE_INTERPROCEDURAL_OPTIMIZATION}
            -DVVDEC_ENABLE_WERROR=ON
            -DVVDEC_USE_ADDRESS_SANITIZER=OFF
            -DVVDEC_USE_THREAD_SANITIZER=OFF
            -DVVDEC_TOPLEVEL_OUTPUT_DIRS=OFF
            -DVVDEC_ENABLE_ITT=OFF

            -DCMAKE_INSTALL_PREFIX=<INSTALL_DIR>
    )

    ExternalProject_Add(
        jpeg-turbo
        PREFIX ${CMAKE_BINARY_DIR}/ThirdParty
        TMP_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Temporaries/jpeg-turbo
        STAMP_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Stamps/jpeg-turbo
        LOG_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Logs/jpeg-turbo
        DOWNLOAD_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Downloads/jpeg-turbo
        SOURCE_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Sources/jpeg-turbo
        BINARY_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Builds/jpeg-turbo
        INSTALL_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Artifacts
        GIT_REPOSITORY "https://github.com/libjpeg-turbo/libjpeg-turbo.git"
        GIT_TAG "3.1.3"
        GIT_SHALLOW ON
        GIT_PROGRESS ON
        CMAKE_ARGS
            ${SHARED_CMAKE_FLAGS}

            -DENABLE_SHARED=OFF
            -DENABLE_STATIC=ON
            -DWITH_ARITH_DEC=ON
            -DWITH_ARITH_ENC=ON
            -DWITH_SIMD=ON
            -DWITH_TURBOJPEG=ON
            -DWITH_TOOLS=OFF
            -DWITH_TESTS=OFF
            -DWITH_FUZZ=OFF
            -DWITH_JAVA=OFF

            -DCMAKE_INSTALL_PREFIX=<INSTALL_DIR>
    )

    ExternalProject_Add(
        openjpeg
        PREFIX ${CMAKE_BINARY_DIR}/ThirdParty
        TMP_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Temporaries/openjpeg
        STAMP_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Stamps/openjpeg
        LOG_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Logs/openjpeg
        DOWNLOAD_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Downloads/openjpeg
        SOURCE_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Sources/openjpeg
        BINARY_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Builds/openjpeg
        INSTALL_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Artifacts
        GIT_REPOSITORY "https://github.com/uclouvain/openjpeg.git"
        GIT_TAG "v2.5.4"
        GIT_SHALLOW ON
        GIT_PROGRESS ON
        CMAKE_ARGS
            ${SHARED_CMAKE_FLAGS}

            -DBUILD_DOC=OFF
            -DOPJ_USE_DSYMUTIL=OFF
            -DBUILD_SHARED_LIBS=OFF
            -DBUILD_STATIC_LIBS=ON
            -DBUILD_JPIP_SERVER=OFF
            -DBUILD_LUTS_GENERATOR=OFF
            -DBUILD_UNIT_TESTS=OFF
            -DBUILD_CODEC=OFF
            -DBUILD_JPIP=OFF
            -DBUILD_VIEWER=OFF
            -DBUILD_JAVA=OFF
            -DBUILD_THIRDPARTY=OFF
            -DBUILD_TESTING=OFF

            -DCMAKE_INSTALL_PREFIX=<INSTALL_DIR>
    )

    ExternalProject_Add(
        libheif
        PREFIX ${CMAKE_BINARY_DIR}/ThirdParty
        TMP_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Temporaries/libheif
        STAMP_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Stamps/libheif
        LOG_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Logs/libheif
        DOWNLOAD_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Downloads/libheif
        SOURCE_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Sources/libheif
        BINARY_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Builds/libheif
        INSTALL_DIR ${CMAKE_BINARY_DIR}/ThirdParty/Artifacts
        GIT_REPOSITORY "https://github.com/strukturag/libheif.git"
        GIT_TAG "v1.21.1"
        GIT_SHALLOW ON
        GIT_PROGRESS ON
        CMAKE_ARGS
            ${SHARED_CMAKE_FLAGS}
            -DCMAKE_C_FLAGS="-DLIBDE265_STATIC_BUILD"       # Fixes linking issues
            -DCMAKE_CXX_FLAGS="-DLIBDE265_STATIC_BUILD"     # Fixes linking issues

            -DENABLE_PLUGIN_LOADING=OFF
            -DBUILD_DOCUMENTATION=OFF
            -DBUILD_FRAMEWORK=OFF
            -DBUILD_TESTING=OFF
            -DENABLE_COVERAGE=OFF
            -DENABLE_EXPERIMENTAL_FEATURES=OFF
            -DENABLE_EXPERIMENTAL_MINI_FORMAT=OFF
            -DENABLE_MULTITHREADING_SUPPORT=ON
            -DENABLE_PARALLEL_TILE_DECODING=ON

            -DWITH_AOM_DECODER=ON
            -DWITH_AOM_DECODER_PLUGIN=OFF
            -DWITH_AOM_ENCODER=OFF
            -DWITH_AOM_ENCODER_PLUGIN=OFF

            -DWITH_DAV1D=OFF
            -DWITH_DAV1D_PLUGIN=OFF

            -DWITH_EXAMPLES=OFF
            -DWITH_EXAMPLE_HEIF_THUMB=OFF
            -DWITH_EXAMPLE_HEIF_VIEW=OFF

            -DWITH_FFMPEG_DECODER=OFF
            -DWITH_FFMPEG_DECODER_PLUGIN=OFF

            -DWITH_FUZZERS=OFF
            -DWITH_GDK_PIXBUF=OFF
            -DWITH_HEADER_COMPRESSION=OFF

            -DWITH_JPEG_DECODER=ON
            -DWITH_JPEG_DECODER_PLUGIN=OFF
            -DWITH_JPEG_ENCODER=OFF
            -DWITH_JPEG_ENCODER_PLUGIN=OFF

            -DWITH_KVAZAAR=OFF
            -DWITH_KVAZAAR_PLUGIN=OFF

            -DWITH_LIBDE265=ON
            -DWITH_LIBDE265_PLUGIN=OFF

            -DWITH_LIBSHARPYUV=OFF
            -DWITH_LIBSHARPYUV_INTERNAL=OFF

            -DWITH_OPENJPH_ENCODER=OFF
            -DWITH_OPENJPH_ENCODER_PLUGIN=OFF

            -DWITH_OpenH264_DECODER=OFF
            -DWITH_OpenH264_DECODER_PLUGIN=OFF

            -DWITH_OpenJPEG_DECODER=ON
            -DWITH_OpenJPEG_DECODER_PLUGIN=OFF
            -DWITH_OpenJPEG_ENCODER=OFF
            -DWITH_OpenJPEG_ENCODER_PLUGIN=OFF

            -DWITH_RAV1E=OFF
            -DWITH_RAV1E_PLUGIN=OFF

            -DWITH_REDUCED_VISIBILITY=ON

            -DWITH_SvtEnc=OFF
            -DWITH_SvtEnc_PLUGIN=OFF

            -DWITH_UNCOMPRESSED_CODEC=OFF

            -DWITH_UVG266=OFF
            -DWITH_UVG266_PLUGIN=OFF

            -DWITH_VVDEC=ON
            -DWITH_VVDEC_PLUGIN=OFF

            -DWITH_VVENC=OFF
            -DWITH_VVENC_PLUGIN=OFF

            -DWITH_WEBCODECS=OFF

            -DWITH_X264=OFF
            -DWITH_X264_PLUGIN=OFF

            -DWITH_X265=OFF
            -DWITH_X265_PLUGIN=OFF

            -DCMAKE_INSTALL_PREFIX=<INSTALL_DIR>

        DEPENDS libde265 aom vvdec jpeg-turbo openjpeg
    )

    if(DEFINED CMAKE_BUILD_TYPE)
        set(UNHEIC_USER_BUILD_TYPE "-DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}")
    endif()
    ExternalProject_Add(
        UnHEIC
        SOURCE_DIR ${CMAKE_SOURCE_DIR}
        TMP_DIR ${CMAKE_BINARY_DIR}
        STAMP_DIR ${CMAKE_BINARY_DIR}
        LOG_DIR ${CMAKE_BINARY_DIR}
        DOWNLOAD_DIR ${CMAKE_BINARY_DIR}
        BINARY_DIR ${CMAKE_BINARY_DIR}
        INSTALL_DIR ${CMAKE_PREFIX_DIR}
        CMAKE_ARGS
            -DCMAKE_PREFIX_PATH=${CMAKE_BINARY_DIR}/ThirdParty/Artifacts
            ${UNHEIC_USER_BUILD_TYPE}

            -D_UNHEIC_BUILD_INTERNAL_RECURSE=ON
            -DCMAKE_EXPORT_COMPILE_COMMANDS=ON

            -DCMAKE_INSTALL_PREFIX=<INSTALL_DIR>
        INSTALL_COMMAND ""
        DEPENDS libheif jpeg-turbo openjpeg
    )

endmacro()
