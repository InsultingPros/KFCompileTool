use crate::constants::config_files::{COMPILATION_CONFIG_NAME, COMPILATION_CONFIG_TEMPLATE};
use crate::{RuntimeVariables, errors::CompileToolErrors};
use std::fmt::Write as _;

pub trait KFConfig {
    /// Creates temporary kf.ini for compilation and adds required `Editpackages`.
    /// # Errors
    /// _
    fn create_kf_config(&self) -> Result<(), CompileToolErrors>;
    /// Remove our temporary compilation `kf.ini`.
    /// # Errors
    /// _
    fn remove_kf_config(&self) -> Result<(), CompileToolErrors>;
}

impl KFConfig for RuntimeVariables {
    fn create_kf_config(&self) -> Result<(), CompileToolErrors> {
        let mut new_content: String = COMPILATION_CONFIG_TEMPLATE.to_string();

        for package in &self.mod_settings.edit_packages {
            // dbg!(package);
            writeln!(&mut new_content, "EditPackages={package}")?;
        }

        std::fs::write(
            self.paths.compile_dir_system.join(COMPILATION_CONFIG_NAME),
            &new_content,
        )?;

        Ok(())
    }

    fn remove_kf_config(&self) -> Result<(), CompileToolErrors> {
        if self.paths.temp_kf_ini.try_exists()? {
            std::fs::remove_file(&self.paths.temp_kf_ini)?;
        }

        Ok(())
    }
}
