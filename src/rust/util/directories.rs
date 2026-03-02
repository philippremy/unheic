use std::path::PathBuf;

use directories::ProjectDirs;

use crate::{
    UnHEICError, UtilityError,
    resource::info::{UNHEIC_APP_LOCALE, UNHEIC_APP_NAME, UNHEIC_APP_ORGANIZATION},
};

#[inline]
fn project_dirs() -> crate::Result<ProjectDirs> {
    ProjectDirs::from(UNHEIC_APP_LOCALE, UNHEIC_APP_ORGANIZATION, UNHEIC_APP_NAME).ok_or(
        UnHEICError::UtilityError(UtilityError::HomeDirectoryNotFound),
    )
}

#[inline]
pub(crate) fn log_dir() -> crate::Result<PathBuf> {
    let base_dirs = project_dirs()?;
    Ok(base_dirs.data_local_dir().join("Logs"))
}
