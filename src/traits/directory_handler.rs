use crate::{
    RuntimeVariables, SourcesCopied, errors::CompileToolErrors,
    utility::source_folder_conflict_resolver,
};

impl RuntimeVariables {
    /// _
    /// # Errors
    /// _
    pub fn handle_alt_style_directories(&self) -> Result<(), CompileToolErrors> {
        // if we don't use alternate style or copy-paste files from somewhere else - do nothing.
        if !self.mod_settings.alt_directories || self.mod_settings.sources_are_somewhere_else {
            return Ok(());
        }

        let classes_folder = &self.paths.compile_dir_sources.join("Classes");

        if classes_folder.exists() {
            std::fs::remove_dir_all(classes_folder)?;
        }

        Ok(())
    }

    /// _
    /// # Errors
    /// _
    pub fn remove_copied_sources(&mut self) -> Result<(), CompileToolErrors> {
        if self.sources_copied_state == SourcesCopied::FromExternalDir {
            std::fs::remove_dir_all(&self.paths.compile_dir_sources)?;
        }

        if self.sources_copied_state == SourcesCopied::Ignored {
            return Ok(());
        }

        if self.mod_settings.sources_are_somewhere_else
            && self.paths.compile_dir_sources.try_exists()?
        {
            source_folder_conflict_resolver(self);
        }

        Ok(())
    }
}
