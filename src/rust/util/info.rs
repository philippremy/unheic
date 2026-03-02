use log::info;

use crate::resource::info::{
    UNHEIC_APP_NAME, UNHEIC_BUILD_COMPILER, UNHEIC_BUILD_OPT_LEVEL, UNHEIC_BUILD_PROFILE,
    UNHEIC_BUILD_TARGET_CPU_ARCH, UNHEIC_BUILD_TARGET_CPU_FEATURES, UNHEIC_BUILD_TARGET_OS,
    UNHEIC_BUILD_TARGET_TRIPLE, UNHEIC_VERSION,
};

#[rustfmt::skip]
pub(crate) fn log_app_info() {
    info!("Application:         {}", UNHEIC_APP_NAME);
    info!("Version:             {}", UNHEIC_VERSION);
    info!("Build Profile:       {}", UNHEIC_BUILD_PROFILE);
    info!("Optimization Level:  {}", UNHEIC_BUILD_OPT_LEVEL);
    info!("OS:                  {}", UNHEIC_BUILD_TARGET_OS);
    info!("CPU:                 {} ({})", UNHEIC_BUILD_TARGET_CPU_ARCH, UNHEIC_BUILD_TARGET_CPU_FEATURES);
    info!("Target Triple:       {}", UNHEIC_BUILD_TARGET_TRIPLE);
    info!("Compiler:            {}", UNHEIC_BUILD_COMPILER);
}
