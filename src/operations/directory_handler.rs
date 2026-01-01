use crate::{
    RuntimeVariables, SourcesCopied, errors::CompileToolErrors,
    utils::conflict_resolver::source_folder_conflict_resolver,
};

/// _
/// # Errors
/// _
pub fn handle_alt_style_directories(input: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    // if we don't use alternate style or copy-paste files from somewhere else - do nothing.
    if !input.mod_settings.alt_directories || input.mod_settings.sources_are_somewhere_else {
        return Ok(());
    }

    let classes_folder = &input.paths.compile_dir_sources.join("Classes");

    if classes_folder.exists() {
        std::fs::remove_dir_all(classes_folder)?;
    }

    Ok(())
}

/// _
/// # Errors
/// _
pub fn remove_copied_sources(input: &mut RuntimeVariables) -> Result<(), CompileToolErrors> {
    if input.sources_copied_state == SourcesCopied::FromExternalDir {
        std::fs::remove_dir_all(&input.paths.compile_dir_sources)?;
    }

    if input.sources_copied_state == SourcesCopied::Ignored {
        return Ok(());
    }

    if input.mod_settings.sources_are_somewhere_else
        && input.paths.compile_dir_sources.try_exists()?
    {
        source_folder_conflict_resolver(input);
    }

    Ok(())
}
