// Helps re-exporting for rustfmt
pub(crate) use crate::resource::info::_info::*;

#[rustfmt::skip]
#[allow(unused)]
pub(crate) mod _info {

    // App Info
    pub(crate) static UNHEIC_APP_NAME: &str = "UnHEIC";
    pub(crate) static UNHEIC_APP_LOCALE: &str = "de";
    pub(crate) static UNHEIC_APP_ORGANIZATION: &str = "philippremy";
    pub(crate) static UNHEIC_VERSION: &str = build_info::format!("{}", $.crate_info.version);
    pub(crate) static UNHEIC_VERSION_MAJOR: &str = build_info::format!("{}", $.crate_info.version.major);
    pub(crate) static UNHEIC_VERSION_MINOR: &str = build_info::format!("{}", $.crate_info.version.minor);
    pub(crate) static UNHEIC_VERSION_PATCH: &str = build_info::format!("{}", $.crate_info.version.patch);
    pub(crate) static UNHEIC_LICENSE_ID: &str = build_info::format!("{}", $.crate_info.license);
    pub(crate) static UNHEIC_AUTHORS: &str = build_info::format!("{}", $.crate_info.authors);

    // Build Info
    pub(crate) static UNHEIC_BUILD_PROFILE: &str = build_info::format!("{}", $.profile);
    pub(crate) static UNHEIC_BUILD_OPT_LEVEL: &str = build_info::format!("{}", $.optimization_level);
    pub(crate) static UNHEIC_BUILD_TARGET_TRIPLE: &str = build_info::format!("{}", $.target.triple);
    pub(crate) static UNHEIC_BUILD_TARGET_OS: &str = build_info::format!("{}", $.target.os);
    pub(crate) static UNHEIC_BUILD_TARGET_CPU_ARCH: &str = build_info::format!("{}", $.target.cpu.arch);
    pub(crate) static UNHEIC_BUILD_TARGET_CPU_FEATURES: &str = build_info::format!("{}", $.target.cpu.features);
    pub(crate) static UNHEIC_BUILD_COMPILER: &str = build_info::format!("{}", $.compiler);

}
