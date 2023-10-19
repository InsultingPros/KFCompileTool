use crate::constants::COMPILATION_CONFIG_NAME;
use crate::types::{CompiledPaths, RuntimeVariables, SectionGlobal, SectionLocal};
use std::path::PathBuf;

pub fn create_runtime_vars(
    global_section: SectionGlobal,
    local_section: SectionLocal,
) -> RuntimeVariables {
    let compile_path: PathBuf = PathBuf::from(global_section.dir_compile.clone());

    RuntimeVariables {
        mutator_name: global_section.package_name.clone(),
        i_compile_outsideof_kf: local_section.compile_outsideof_kf,
        alt_directories: local_section.alt_directories,
        move_files: local_section.move_files,
        create_int: local_section.create_int,
        make_redirect: local_section.make_redirect,
        make_release: local_section.make_release,
        path_source_files: PathBuf::from(global_section.dir_classes.clone()),
        compiled_paths: CompiledPaths {
            path_compile_dir: compile_path.clone(),
            path_compile_system: compile_path.join("System"),
            path_compilation_ini: compile_path.join(format!("System\\{COMPILATION_CONFIG_NAME}")),
            path_ucc: compile_path.join("System\\UCC.exe"),
            path_copied_sources: {
                if global_section.dir_compile == global_section.dir_classes {
                    None
                } else {
                    Some(compile_path.join(global_section.package_name.clone()))
                }
            },
            compiled_file_u: compile_path
                .join(format!("System\\{}.u", global_section.package_name)),
            compiled_file_ucl: compile_path
                .join(format!("System\\{}.ucl", global_section.package_name)),
            compiled_file_int: {
                if local_section.create_int {
                    Some(compile_path.join(format!("System\\{}.int", global_section.package_name)))
                } else {
                    None
                }
            },
            compiled_file_uz2: {
                if local_section.make_redirect {
                    Some(
                        compile_path
                            .join(format!("Redirect\\{}.u.uz2", global_section.package_name)),
                    )
                } else {
                    None
                }
            },
            steam_app_id_hack: compile_path.join("System\\steam_appid.txt"),
        },
        copy_path: {
            if local_section.move_files {
                global_section.dir_move_to.map(PathBuf::from)
            } else {
                None
            }
        },
        release_path: {
            if local_section.make_release {
                global_section.dir_release_output.map(PathBuf::from)
            } else {
                None
            }
        },
    }
}
