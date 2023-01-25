use crate::constants;
use crate::errors::CompileToolErrors;
use crate::types::{SectionGlobal, SectionLocal};
// use anyhow::Context;
use configparser::ini::Ini;
use std::fmt::Write;
use std::fs;
use std::path::Path;

/// check if our settings file exists in the same directory
pub fn internal_config_exists() -> Result<(), CompileToolErrors> {
    if !Path::new(constants::INTERNAL_CONFIG_NAME).exists() {
        println!(
            "{} DOESNT exist! Creating it!",
            constants::INTERNAL_CONFIG_NAME
        );
        fs::write(
            constants::INTERNAL_CONFIG_NAME,
            constants::INTERNAL_CONFIG_TEMPLATE,
        )?;
    }

    Ok(())
}

/// create kfcompile.ini and add required Editpackages
pub fn create_kf_config(packages: &[String]) -> Result<(), CompileToolErrors> {
    let mut new_content: String = constants::COMPILATION_CONFIG_TEMPLATE.to_string();

    packages.iter().for_each(|package: &String| {
        println!("{package}");
        let _ = writeln!(&mut new_content, "EditPackages={}", package)
            .map_err(CompileToolErrors::WriteError);
    });

    fs::write(constants::COMPILATION_CONFIG_NAME, &new_content)?;

    Ok(())
}

/// Parse internal config file and get the variables
// todo: remove panics!
pub fn parse_internal_config() -> Result<(SectionGlobal, SectionLocal), String> {
    let mut my_config: Ini = Ini::new();
    my_config
        .load(constants::INTERNAL_CONFIG_NAME)
        .unwrap_or_else(|_| {
            panic!(
                "Could not load `{}!` Check file read access.",
                constants::INTERNAL_CONFIG_NAME
            )
        });

    let result_global: SectionGlobal = SectionGlobal {
        // these variables are important
        package_name: my_config
            .get("global", "mutatorName")
            .expect("'mutatorName' isn't specified in the config, aborting!"),
        dir_compile: my_config
            .get("global", "dir_Compile")
            .expect("'dir_Compile' path isn't specified in the config, aborting!"),
        dir_classes: my_config
            .get("global", "dir_Classes")
            .expect("'dir_Classes' path isn't specified in the config, aborting!"),
        // all these are optional
        dir_move_to: my_config.get("global", "dir_MoveTo"),
        dir_release_output: my_config.get("global", "dir_ReleaseOutput"),
    };

    let result_local: SectionLocal = SectionLocal {
        // this one is important
        edit_packages: my_config
            .get(&result_global.package_name, "EditPackages")
            .expect("`EditPackages` variable is empty, aborting!"),
        // everything else is optional and can be set to default (false) values on fail
        compile_outsideof_kf: my_config
            .getbool(&result_global.package_name, "bICompileOutsideofKF")?
            .unwrap_or_default(),
        alt_directories: my_config
            .getbool(&result_global.package_name, "bAltDirectories")?
            .unwrap_or_default(),
        move_files: my_config
            .getbool(&result_global.package_name, "bMoveFiles")?
            .unwrap_or_default(),
        create_int: my_config
            .getbool(&result_global.package_name, "bCreateINT")?
            .unwrap_or_default(),
        make_redirect: my_config
            .getbool(&result_global.package_name, "bMakeRedirect")?
            .unwrap_or_default(),
        make_release: my_config
            .getbool(&result_global.package_name, "bMakeRelease")?
            .unwrap_or_default(),
    };

    Ok((result_global, result_local))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_config_exists() {
        match internal_config_exists() {
            Ok(_) => {}
            Err(e) => {
                println!("Error happened! {}", e)
            }
        };
    }

    #[test]
    fn test_create_kf_config() {
        let arg: &[String; 2] = &["Package1".to_string(), "Package2".to_string()];
        match create_kf_config(arg) {
            Ok(_) => {}
            Err(e) => {
                println!("Error happened! {}", e)
            }
        };
    }

    #[test]
    fn test_parse_internal_config() {
        match parse_internal_config() {
            Ok(result) => {
                println!("{:?}", result)
            }
            Err(e) => {
                println!("Error happened! {}", e)
            }
        };
    }
}
