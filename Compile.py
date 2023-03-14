# Python version of my shit bat file
# Author    : NikC-
# Home repo : https://github.com/InsultingPros/KFCompileTool
# License   : https://www.gnu.org/licenses/gpl-3.0.en.html


#################################################################################
#                               IMPORTING
#################################################################################
import os
import shutil
from subprocess import run, CalledProcessError
import sys
from dataclasses import dataclass
from configparser import ConfigParser
from enum import Enum
from pathlib import Path
from typing import Any

#################################################################################
#                              'CONSTANTS'
#################################################################################
LINE_SEPARATOR: str = "\n######################################################\n"
SETTINGS_FILE: str = "CompileSettings.ini"
"""Settings file for this script, contains client-server directories and mods info"""
CMPL_CONFIG: str = "kfcompile.ini"
"""Game config that UCC.exe uses for compilation, contains EditPackages lines and the most minimal setup"""
REDIRECT_DIR_NAME: str = "Redirect"
"""Folder name for redirect files"""
IGNORE_LIST: list[str] = [".git", "*.md", "Docs", "LICENSE"]
"""Filter for files-directories, so we copy-paste only source files"""


class ERROR(Enum):
    NO_SETTINGS = 0
    NO_GLOBAL_SECTION = 1
    NO_LOCAL_SECTION = 2
    NO_UCC = 3
    WRONG_DIR_STYLE = 4
    COMPILATION_FAILED = 5


#################################################################################
#                                UTILITY
#################################################################################


@dataclass
class RuntimeVars:
    """Contains 'runtime' variables"""

    # Global
    mutatorName: str = "fallback mutatorName"
    dir_Compile: str = "fallback dir_Compile"
    dir_MoveTo: str = "fallback dir_MoveTo"
    dir_ReleaseOutput: str = "fallback dir_ReleaseOutput"
    dir_Classes: str = "fallback dir_Classes"
    # sections
    EditPackages: str = "fallback EditPackages"
    bICompileOutsideofKF: bool = False
    bAltDirectories: bool = False
    bMoveFiles: bool = False
    bCreateINT: bool = False
    bMakeRedirect: bool = False
    bMakeRelease: bool = False
    # paths to use
    path_source_files: Path = Path()
    path_compile_dir: Path = Path()
    path_compile_dir_sys: Path = Path()
    path_compiled_file_u: Path = Path()
    path_compiled_file_ucl: Path = Path()
    path_compiled_file_uz2: Path = Path()
    path_compiled_file_int: Path = Path()
    path_compilation_ini: Path = Path()
    path_garbage_file: Path = Path()
    path_release: Path = Path()
    path_move_to: Path = Path()

    def __str__(self):
        return (
            f"{LINE_SEPARATOR}"
            f"{SETTINGS_FILE}\n"
            f"mutatorName           = {self.mutatorName}\n"
            f"dir_Compile           = {self.dir_Compile}\n"
            f"dir_MoveTo            = {self.dir_MoveTo}\n"
            f"dir_ReleaseOutput     = {self.dir_ReleaseOutput}\n"
            f"dir_Classes           = {self.dir_Classes}\n"
            f"EditPackages          = {self.EditPackages}\n"
            f"bICompileOutsideofKF  = {self.bICompileOutsideofKF}\n"
            f"bAltDirectories       = {self.bAltDirectories}\n"
            f"bMoveFiles            = {self.bMoveFiles}\n"
            f"bCreateINT            = {self.bCreateINT}\n"
            f"bMakeRedirect         = {self.bMakeRedirect}\n"
            f"bMakeRelease          = {self.bMakeRelease}"
        )


class Types:
    """Contains lists, dicts used to populate kfcompile.ini / CompileSettings.ini"""

    # CompileSettings.ini
    def_Global: dict[str, str] = {
        "mutatorName": "TestMut",
        "dir_Compile": r"D:\Games\SteamLibrary\steamapps\common\KillingFloor",
        "dir_MoveTo": r"D:\Games\KF Dedicated Server",
        "dir_ReleaseOutput": r"C:\Users\USER\Desktop\Mutators",
        "dir_Classes": r"C:\Users\USER\Desktop\Projects",
    }

    def_Mod: dict[str, Any] = {
        "EditPackages": "TestMutParent,TestMut",
        "bICompileOutsideofKF": False,
        "bAltDirectories": False,
        "bMoveFiles": False,
        "bCreateINT": False,
        "bMakeRedirect": False,
        "bMakeRelease": False,
    }

    # kfcompile.ini
    # [Editor.EditorEngine]
    def_EditPackages: list[str] = [
        "Core",
        "Engine",
        "Fire",
        "Editor",
        "UnrealEd",
        "IpDrv",
        "UWeb",
        "GamePlay",
        "UnrealGame",
        "XGame",
        "XInterface",
        "XAdmin",
        "XWebAdmin",
        "GUI2K4",
        "xVoting",
        "UTV2004c",
        "UTV2004s",
        "ROEffects",
        "ROEngine",
        "ROInterface",
        "Old2k4",
        "KFMod",
        "KFChar",
        "KFGui",
        "GoodKarma",
        "KFMutators",
        "KFStoryGame",
        "KFStoryUI",
        "SideShowScript",
        "FrightScript",
    ]

    # [Engine.Engine]
    # this setting is enough
    EngineDict: dict[str, str] = {"EditorEngine": "Editor.EditorEngine"}

    # [Core.System]
    # this too
    SysDict: dict[str, str] = {"CacheRecordPath": "../System/*.ucl"}

    def_paths: list[str] = [
        "../System/*.u",
        "../Maps/*.rom",
        "../TestMaps/*.rom",
        "../Textures/*.utx",
        "../Sounds/*.uax",
        "../Music/*.umx",
        "../StaticMeshes/*.usx",
        "../Animations/*.ukx",
        "../Saves/*.uvx",
        "../Textures/Old2k4/*.utx",
        "../Sounds/Old2k4/*.uax",
        "../Music/Old2k4/*.umx",
        "../StaticMeshes/Old2k4/*.usx",
        "../Animations/Old2k4/*.ukx",
        "../KarmaData/Old2k4/*.ka",
    ]

    def_Suppress: list[str] = ["DevLoad", "DevSave"]


def safe_delete_file(input_path: Path) -> None:
    """Check and delete the file"""
    if input_path.exists() and input_path.is_file():
        try:
            input_path.unlink()
        except PermissionError as e:
            sys.exit("Failed to delete the file: " + str(e))


def copy_file(source_path: Path, destination_path: Path) -> None:
    if not source_path.is_file():
        return
    try:
        shutil.copy(source_path, destination_path)
        print("> Copied:  ", source_path, "  --->  ", destination_path)
    except PermissionError as e:
        sys.exit("Failed to copy the file: " + str(e))


# post compilation / failure cleanup
def cleanup_files() -> None:
    # remove garbage-temporary files
    safe_delete_file(r.path_garbage_file)
    safe_delete_file(r.path_compilation_ini)

    if r.bICompileOutsideofKF:
        safe_delete_dir(r.path_compile_dir.joinpath(r.mutatorName))


# https://docs.python.org/3/library/shutil.html#rmtree-example
def remove_readonly(func, path, _) -> None:
    """Clear the readonly bit and reattempt the removal"""
    Path(path).chmod(0o0200)
    func(path)


def safe_delete_dir(input_path: Path) -> None:
    """remove new created 'classes' folder on alternate dir style"""
    if input_path.exists():
        shutil.rmtree(input_path, onerror=remove_readonly)


def create_def_compile_ini(destination_path: Path) -> None:
    """Create DEFAULT config file if none found"""

    def write_line_to_config(text: str) -> None:
        """Write single line to file"""
        with open(destination_path, "a") as f:
            f.writelines([text + "\n"])

    def write_list_to_config(key: str, input_list: list[str]) -> None:
        """Add lines at the end of the file"""
        with open(destination_path, "a") as f:
            for x in input_list:
                f.writelines([key + "=" + x + "\n"])

    def write_dict_to_config(input_dict: dict[str, str]) -> None:
        """write key-value from dictionary"""
        with open(destination_path, "a") as f:
            for k, v in input_dict.items():
                f.writelines([k + "=" + v + "\n"])

    # SECTION 1
    write_line_to_config("[Editor.EditorEngine]")

    write_list_to_config("EditPackages", Types.def_EditPackages)
    write_line_to_config("\n")

    # SECTION 2
    write_line_to_config("[Engine.Engine]")
    write_dict_to_config(Types.EngineDict)
    write_line_to_config("\n")

    # SECTION 3
    write_line_to_config("[Core.System]")
    write_dict_to_config(Types.SysDict)

    write_list_to_config("Paths", Types.def_paths)
    write_list_to_config("Suppress", Types.def_Suppress)
    write_line_to_config("\n")

    # SECTION 4
    # if we don't add this section, we will get some other garbage being written
    write_line_to_config("[ROFirstRun]")
    write_line_to_config("ROFirstRun=1094\n")

    # shutil.move(CMPL_CONFIG, destination_path)


def print_separator_box(msg: str) -> None:
    """Print nice message box"""
    print(LINE_SEPARATOR, msg, LINE_SEPARATOR)


def throw_error(err: ERROR):
    """Throw human-readable error message."""
    prefix: str = ">>> TERMINATION WARNING: "
    match err:
        case ERROR.NO_SETTINGS:
            print(
                prefix
                + SETTINGS_FILE
                + """ was not found. We created a new file for you, in the same directory.
                    PLEASE go and edit it to fit your needs."""
            )
        case ERROR.NO_GLOBAL_SECTION:
            print(
                prefix
                + """Global section not found in CompileSettings.ini.
                    PLEASE go and fill it manually"""
            )
        case ERROR.NO_LOCAL_SECTION:
            print(
                prefix
                + r.mutatorName
                + """ section not found in CompileSettings.ini.
                    PLEASE go and fill it manually"""
            )
        case ERROR.NO_UCC:
            print(
                prefix
                + """UCC.exe was not found in compile directory.
                    PLEASE Install SDK / check your directories in Global section."""
            )
        case ERROR.WRONG_DIR_STYLE:
            print(
                prefix
                + "Alternative Directory is True, but `sources` folder NOT FOUND!"
            )
        case ERROR.COMPILATION_FAILED:
            print(prefix + "Compilation FAILED!")
        case _:
            print(prefix + "undefined error code!")

    input("Press any key to close.")
    exit()


#################################################################################
#                                FUNCTIONS
#################################################################################


def init_settings() -> None:
    """Read config file and define all variables."""

    def create_settings_file(input_dir) -> None:
        """Create DEFAULT config file if none found"""
        config = ConfigParser()
        # save the case
        config.optionxform = str

        config["Global"] = Types.def_Global
        config["TestMut"] = Types.def_Mod

        with open(input_dir, "w") as configfile:
            config.write(configfile, space_around_delimiters=False)

    # self directory
    dir_script: str = os.path.dirname(os.path.realpath(__file__))
    dir_settings_ini: str = os.path.join(dir_script, SETTINGS_FILE)
    # check if settings.ini exists in same directory
    if not Path(dir_settings_ini).is_file():
        create_settings_file(dir_settings_ini)
        throw_error(ERROR.NO_SETTINGS)

    config = ConfigParser()
    config.read(dir_settings_ini)
    # get global section and set main vars
    if not config.has_section("Global"):
        throw_error(ERROR.NO_GLOBAL_SECTION)

    # GLOBAL
    # accept cmdline arguments
    if len(sys.argv) == 1:
        r.mutatorName = config["Global"]["mutatorName"]
    else:
        r.mutatorName = sys.argv[1]

    r.dir_Compile = config["Global"]["dir_Compile"]
    r.dir_MoveTo = config["Global"]["dir_MoveTo"]
    r.dir_ReleaseOutput = config["Global"]["dir_ReleaseOutput"]
    r.dir_Classes = config["Global"]["dir_Classes"]

    # SECTIONS
    # check if exist
    if not config.has_section(r.mutatorName):
        throw_error(ERROR.NO_LOCAL_SECTION)

    r.EditPackages = config[r.mutatorName]["EditPackages"]
    r.bICompileOutsideofKF = config[r.mutatorName].getboolean("bICompileOutsideofKF")
    r.bAltDirectories = config[r.mutatorName].getboolean("bAltDirectories")
    r.bMoveFiles = config[r.mutatorName].getboolean("bMoveFiles")
    r.bCreateINT = config[r.mutatorName].getboolean("bCreateINT")
    r.bMakeRedirect = config[r.mutatorName].getboolean("bMakeRedirect")
    r.bMakeRelease = config[r.mutatorName].getboolean("bMakeRelease")

    # paths to dirs
    r.path_source_files = Path(r.dir_Classes)
    r.path_compile_dir = Path(r.dir_Compile)
    r.path_compile_dir_sys = r.path_compile_dir.joinpath("System")
    r.path_release = Path(r.dir_ReleaseOutput)
    r.path_move_to = Path(r.dir_MoveTo)
    # paths to files
    r.path_garbage_file = r.path_compile_dir_sys.joinpath("steam_appid.txt")
    r.path_compilation_ini = r.path_compile_dir_sys.joinpath(CMPL_CONFIG)
    r.path_compiled_file_u = r.path_compile_dir_sys.joinpath(
        "{}.{}".format(r.mutatorName, "u")
    )
    r.path_compiled_file_ucl = r.path_compile_dir_sys.joinpath(
        "{}.{}".format(r.mutatorName, "ucl")
    )
    r.path_compiled_file_uz2 = r.path_compile_dir_sys.joinpath(
        "{}.{}".format(r.mutatorName, "u.uz2")
    )
    r.path_compiled_file_int = r.path_compile_dir_sys.joinpath(
        "{}.{}".format(r.mutatorName, "int")
    )

    # make sure there are no old files
    safe_delete_file(r.path_compilation_ini)

    # update editPackages and create the kf.ini
    Types.def_EditPackages.extend(r.EditPackages.split(","))
    create_def_compile_ini(r.path_compilation_ini)


def compile_me() -> None:
    # delete old files before compilation start, since UCC is ghei
    safe_delete_file(r.path_compiled_file_u)
    safe_delete_file(r.path_compiled_file_ucl)
    safe_delete_file(r.path_compiled_file_int)

    dir_source: Path = r.path_source_files.joinpath(r.mutatorName)
    dir_destination: Path = r.path_compile_dir.joinpath(r.mutatorName)
    # if our mod files are in other directory, just copy-paste everything from there

    # if mod folder is outside, delete old dir and copy-paste new one
    if r.bICompileOutsideofKF:
        safe_delete_dir(dir_destination)
        shutil.copytree(
            dir_source,
            dir_destination,
            copy_function=shutil.copy,
            ignore=shutil.ignore_patterns(*IGNORE_LIST),
        )

    # if we use alternative directory style, we need to do some work
    if r.bAltDirectories:
        sources: Path = dir_source.joinpath("sources")
        if not sources.exists():
            throw_error(ERROR.WRONG_DIR_STYLE)

        classes: Path = dir_destination.joinpath("Classes")
        safe_delete_dir(classes)
        Path(classes).mkdir()
        # now copy everything!
        for path, subdir, files in os.walk(sources):
            for name in files:
                filename = os.path.join(path, name)
                shutil.copy2(filename, classes)

    print_separator_box("COMPILING: " + r.mutatorName)

    ucc: Path = r.path_compile_dir_sys.joinpath("UCC.exe")
    # check if we have UCC
    if not ucc.is_file():
        throw_error(ERROR.NO_UCC)

    # start the actual compilation! FINALLY!!!
    try:
        run([ucc, "make", "ini=" + CMPL_CONFIG, "-EXPORTCACHE"], check=True)
    except CalledProcessError as e:
        print(str(e))
        cleanup_files()
        throw_error(ERROR.COMPILATION_FAILED)

    # create INT files
    if r.bCreateINT:
        try:
            print_separator_box("Creating INT file!")
            os.chdir(r.path_compile_dir_sys)
            run(["ucc", "dumpint", r.path_compiled_file_u], check=True)
        except CalledProcessError as e:
            print(str(e))

    # create UZ2 files
    if r.bMakeRedirect:
        try:
            print_separator_box("Creating UZ2 file!")
            os.chdir(r.path_compile_dir_sys)
            run(["ucc", "compress", r.path_compiled_file_u], check=True)
        except CalledProcessError as e:
            print(str(e))

    # cleanup!
    cleanup_files()


def handle_files() -> None:
    # announce
    print_separator_box("MOVING FILES")

    # do we want files being moved to desired client / server directory?
    if r.bMoveFiles:
        try:
            print(">>> Moving files to CLIENT directory.\n")
            destination: Path = r.path_move_to.joinpath("System")
            copy_file(r.path_compiled_file_u, destination)
            copy_file(r.path_compiled_file_ucl, destination)
            copy_file(r.path_compiled_file_int, destination)
        except Exception as e:
            print("Failed to move compiled files: " + str(e))

    if r.bMakeRelease:
        try:
            print(">>> Moving files to output directory.\n")
            path_release: Path = r.path_release.joinpath(r.mutatorName)
            # cleanup old files at first
            safe_delete_dir(path_release)
            # if 'Redirect' folder doesn't exist, create it
            if not path_release.exists():
                path_release.mkdir()
            # copy files
            copy_file(r.path_compiled_file_u, path_release)
            copy_file(r.path_compiled_file_ucl, path_release)
            copy_file(r.path_compiled_file_int, path_release)

            if r.bMakeRedirect:
                path_redirect: Path = path_release.joinpath(REDIRECT_DIR_NAME)
                if not path_redirect.exists():
                    path_redirect.mkdir()
                # copy files
                copy_file(r.path_compiled_file_uz2, path_redirect)
        except Exception as e:
            print("Failed to create a redirect file in release folder: " + str(e))

    # remove the file from system after everything else is done
    if r.bMakeRedirect:
        try:
            print("\n>>> Moving redirect file to redirect directory.\n")
            copy_file(
                r.path_compiled_file_uz2, r.path_compile_dir.joinpath(REDIRECT_DIR_NAME)
            )
            safe_delete_file(r.path_compiled_file_uz2)
        except Exception as e:
            print("Failed to create a redirect file: " + str(e))


#################################################################################
#                              FUNCTION CALLS
#################################################################################

r: RuntimeVars


def main() -> None:
    global r

    r = RuntimeVars()
    # check if we have all configs and everything is fine
    # then assign global vars
    init_settings()
    # useful logs, if you want em
    print(r)
    # compile!
    compile_me()
    handle_files()

    # exit the script, everything is done
    input("\n" + "Press any key to continue.")


if __name__ == "__main__":
    main()
