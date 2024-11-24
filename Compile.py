# Advanced compilation script for KF1
# Author    : Shtoyan
# Home repo : https://github.com/InsultingPros/KFCompileTool
# License   : https://www.gnu.org/licenses/gpl-3.0.en.html

import sys
from configparser import ConfigParser
from dataclasses import dataclass
from enum import IntEnum, auto
from logging import DEBUG, Logger, basicConfig, getLogger
from pathlib import Path
from shutil import copy, copy2, copytree, ignore_patterns, rmtree
from stat import S_IREAD, S_IWRITE
from subprocess import CalledProcessError, check_call
from sys import argv
from sys import exit as s_exit
from typing import Any, Final, NoReturn

if getattr(sys, "frozen", False) and hasattr(sys, "_MEIPASS"):
    script_dir = Path(sys.executable).parent
else:
    script_dir = Path(__file__).parent

LINE_SEPARATOR: Final[str] = (
    "\n######################################################\n"
)
REDIRECT_DIR_NAME: Final[str] = "Redirect"
"""Folder name for redirect files"""
IGNORE_LIST: Final[list[str]] = [".git", "*.md", "Docs", "LICENSE"]
"""Filter for files-directories, so we copy-paste only source files"""

SETTINGS_FILE_NAME: Final[str] = "CompileSettings.ini"
"""Settings file for this script, contains client-server directories and mods info"""
SETTINGS_FILE_CONTENT: Final[
    str
] = r"""; Online Help : https://github.com/InsultingPros/KFCompileTool
[Global]
mutatorName=TestMut
dir_Compile=D:\Games\SteamLibrary\steamapps\common\KillingFloor
dir_MoveTo=D:\Games\KF Dedicated Server
dir_ReleaseOutput=C:\Users\USER\Desktop\Mutators
dir_Classes=C:\Users\USER\Desktop\Projects

[TestMut]
EditPackages=TestMut
bICompileOutsideofKF=False
bAltDirectories=False
bMoveFiles=False
bCreateINT=False
bMakeRedirect=False
bMakeRelease=False

[AnotherMutThatDependsOnTestMut]
EditPackages=TestMut,AnotherMutThatDependsOnTestMut
bICompileOutsideofKF=False
bAltDirectories=False
bMoveFiles=False
bCreateINT=False
bMakeRedirect=False
bMakeRelease=False
"""

COMPILATION_CONFIG_NAME: Final[str] = "kfcompile.ini"
"""Game config that UCC.exe uses for compilation.
Contains EditPackages lines and the most minimal setup"""
COMPILATION_CONFIG_CONTENT: Final[
    str
] = """;= WARNING! This file is generated for one time compilation!
[Engine.Engine]
EditorEngine=Editor.EditorEngine

[Core.System]
CacheRecordPath=../System/*.ucl
Paths=../System/*.u
Paths=../Maps/*.rom
Paths=../TestMaps/*.rom
Paths=../Textures/*.utx
Paths=../Sounds/*.uax
Paths=../Music/*.umx
Paths=../StaticMeshes/*.usx
Paths=../Animations/*.ukx
Paths=../Saves/*.uvx
Paths=../Textures/Old2k4/*.utx
Paths=../Sounds/Old2k4/*.uax
Paths=../Music/Old2k4/*.umx
Paths=../StaticMeshes/Old2k4/*.usx
Paths=../Animations/Old2k4/*.ukx
Paths=../KarmaData/Old2k4/*.ka
Suppress=DevLoad
Suppress=DevSave

[ROFirstRun]
ROFirstRun=1094

[Editor.EditorEngine]
EditPackages=Core
EditPackages=Engine
EditPackages=Fire
EditPackages=Editor
EditPackages=UnrealEd
EditPackages=IpDrv
EditPackages=UWeb
EditPackages=GamePlay
EditPackages=UnrealGame
EditPackages=XGame
EditPackages=XInterface
EditPackages=XAdmin
EditPackages=XWebAdmin
EditPackages=GUI2K4
EditPackages=xVoting
EditPackages=UTV2004c
EditPackages=UTV2004s
EditPackages=ROEffects
EditPackages=ROEngine
EditPackages=ROInterface
EditPackages=Old2k4
EditPackages=KFMod
EditPackages=KFChar
EditPackages=KFGui
EditPackages=GoodKarma
EditPackages=KFMutators
EditPackages=KFStoryGame
EditPackages=KFStoryUI
EditPackages=SideShowScript
EditPackages=FrightScript
"""


class ERROR(IntEnum):
    NO_SETTINGS = auto()
    NO_GLOBAL_SECTION = auto()
    NO_LOCAL_SECTION = auto()
    NO_UCC = auto()
    WRONG_DIR_STYLE = auto()
    COMPILATION_FAILED = auto()
    DUPLICATE_FILES = auto()
    NO_COMPILE_DIR = auto()


LOG: Logger = getLogger(Path(__file__).stem)
BASIC_FORMAT = "[%(levelname)s]:[%(filename)s:%(lineno)s - %(funcName)s()] %(message)s"
basicConfig(format=BASIC_FORMAT)
LOG.setLevel(DEBUG)

#################################################################################
#                                UTILITY
#################################################################################


@dataclass(slots=True)
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
    path_ucc: Path = Path()
    path_compile_dir_sys: Path = Path()
    path_compiled_file_u: Path = Path()
    path_compiled_file_ucl: Path = Path()
    path_compiled_file_uz2: Path = Path()
    path_compiled_file_int: Path = Path()
    path_compilation_ini: Path = Path()
    path_garbage_file: Path = Path()
    path_release: Path = Path()
    path_move_to: Path = Path()

    def __str__(self) -> str:
        return (
            f"\n{SETTINGS_FILE_NAME}\n"
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


def safe_delete_file(input_path: Path) -> None:
    """Check and delete the file"""
    if input_path.exists() and input_path.is_file():
        try:
            input_path.chmod(S_IWRITE)
            input_path.unlink()
        except PermissionError as e:
            s_exit("Failed to delete the file: " + str(e))


def copy_file(source_path: Path, destination_path: Path) -> None:
    if not source_path.is_file():
        return
    try:
        copy(source_path, destination_path)
        print(f"> Copied:  {source_path}  --->  {destination_path}")
    except Exception as e:
        s_exit("Failed to copy the file: " + str(e))


# post compilation / failure cleanup
def cleanup_files() -> None:
    # remove garbage-temporary files
    safe_delete_file(r.path_garbage_file)
    safe_delete_file(r.path_compilation_ini)

    # remove folder with all sources from compile directory
    if r.bICompileOutsideofKF:
        safe_delete_dir(r.path_compile_dir.joinpath(r.mutatorName))

    # remove classes folder, we use alternative file organization method
    if r.bAltDirectories:
        safe_delete_dir(r.path_compile_dir.joinpath(r.mutatorName).joinpath("Classes"))


def safe_delete_dir(input_path: Path) -> None:
    """Safe delete directories."""

    # https://docs.python.org/3/library/shutil.html#rmtree-example
    def remove_readonly(func: Any, path: Any, _: Any) -> None:
        """Clear the readonly bit and reattempt the removal"""
        Path(path).chmod(S_IWRITE)
        func(path)

    if input_path.exists():
        rmtree(input_path, onerror=remove_readonly)


def print_separator_box(msg: str) -> None:
    """Print nice message box"""
    print(f"{LINE_SEPARATOR} {msg} {LINE_SEPARATOR}")


def throw_error(err: ERROR) -> NoReturn:
    """Throw human-readable error message."""
    match err:
        case ERROR.NO_SETTINGS:
            LOG.error(
                (
                    f"'{SETTINGS_FILE_NAME}' was not found."
                    " We created a new file for you, in the same directory."
                    " PLEASE go and edit it to fit your needs."
                )
            )
        case ERROR.NO_GLOBAL_SECTION:
            LOG.error(
                (
                    f"`Global` section not found in '{SETTINGS_FILE_NAME}'."
                    " PLEASE go and fill it manually."
                )
            )
        case ERROR.NO_LOCAL_SECTION:
            LOG.error(
                (
                    f"'{r.mutatorName}' section not found in '{SETTINGS_FILE_NAME}'."
                    " PLEASE go and fill it manually."
                )
            )
        case ERROR.NO_UCC:
            LOG.error(
                (
                    f"`UCC.exe` was not found in `{r.path_compile_dir}`."
                    " Install SDK / check your compile directory in `Global` section."
                )
            )
        case ERROR.WRONG_DIR_STYLE:
            LOG.error("Alternative Directory is True, but `sources` folder NOT FOUND!")
        case ERROR.COMPILATION_FAILED:
            LOG.error("Compilation FAILED!")
        case ERROR.DUPLICATE_FILES:
            LOG.error("Remove duplicates from your `sources` folder!")
        case ERROR.NO_COMPILE_DIR:
            LOG.error(f"Your compile directory (`{r.path_compile_dir}`) doesn't exist!")

    cleanup_files()
    s_exit()


#################################################################################
#                                FUNCTIONS
#################################################################################


def init_settings() -> None:
    """Read config file and define all variables."""

    def create_compilation_ini(
        destination_path: Path, edit_packages_list: list[str]
    ) -> None:
        """Create DEFAULT config file if none found"""
        with open(destination_path, "a") as f:
            f.write(COMPILATION_CONFIG_CONTENT)

            for package in edit_packages_list:
                f.writelines(f"EditPackages={package}\n")

    def create_steam_appid(destination_path: Path) -> None:
        """Create `steam_appid.txt` with edited value. So when you compile steam won't
        notify your friends that you are playing the KF.

        Thanks Alice for the hint!"""
        if destination_path.exists():
            destination_path.chmod(S_IWRITE)
        with open(destination_path, "w") as f:
            f.write("3")
        destination_path.chmod(S_IREAD)

    def create_default_settings_file(input_dir: Path) -> None:
        with open(input_dir, "w") as f:
            f.write(SETTINGS_FILE_CONTENT)

    path_settings_ini: Path = script_dir.joinpath(SETTINGS_FILE_NAME)
    # check if settings.ini exists in same directory
    if not path_settings_ini.is_file():
        create_default_settings_file(path_settings_ini)
        throw_error(ERROR.NO_SETTINGS)

    config: ConfigParser = ConfigParser()
    config.read(path_settings_ini)
    # get global section and set main vars
    if not config.has_section("Global"):
        throw_error(ERROR.NO_GLOBAL_SECTION)

    # GLOBAL
    # accept cmdline arguments
    if len(argv) == 1:
        r.mutatorName = config["Global"]["mutatorName"]
    else:
        r.mutatorName = argv[1]

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
    if not r.path_compile_dir.exists():
        throw_error(ERROR.NO_COMPILE_DIR)
    r.path_compile_dir_sys = r.path_compile_dir.joinpath("System")
    r.path_ucc = r.path_compile_dir_sys.joinpath("UCC.exe")
    # check if we have UCC
    if not r.path_ucc.is_file():
        throw_error(ERROR.NO_UCC)
    r.path_release = Path(r.dir_ReleaseOutput)
    r.path_move_to = Path(r.dir_MoveTo)
    # paths to files
    r.path_garbage_file = r.path_compile_dir_sys.joinpath("steam_appid.txt")
    r.path_compilation_ini = r.path_compile_dir_sys.joinpath(COMPILATION_CONFIG_NAME)
    path_compiled_file_name: Path = r.path_compile_dir_sys.joinpath(r.mutatorName)
    r.path_compiled_file_u = path_compiled_file_name.with_suffix(".u")
    r.path_compiled_file_ucl = path_compiled_file_name.with_suffix(".ucl")
    r.path_compiled_file_uz2 = path_compiled_file_name.with_suffix(".u.uz2")
    r.path_compiled_file_int = path_compiled_file_name.with_suffix(".int")

    # make sure there are no old files
    safe_delete_file(r.path_compilation_ini)

    # update editPackages and create the kf.ini
    create_compilation_ini(r.path_compilation_ini, r.EditPackages.split(","))
    create_steam_appid(r.path_garbage_file)


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
        copytree(
            dir_source,
            dir_destination,
            copy_function=copy,
            ignore=ignore_patterns(*IGNORE_LIST),
        )

    # if we use alternative directory style, we need to do some work
    if r.bAltDirectories:
        path_sources: Path = dir_source.joinpath("sources")
        if not path_sources.exists():
            throw_error(ERROR.WRONG_DIR_STYLE)

        path_classes: Path = dir_destination.joinpath("Classes")
        safe_delete_dir(path_classes)
        path_classes.mkdir()

        file_name_list: list[str] = []
        # quick check for duplicate files (same name)!
        for file in path_sources.rglob("*.uc"):
            file_name_list.append(file.name)
        list_duplicate: list[str] = [
            item for item in set(file_name_list) if file_name_list.count(item) > 1
        ]

        if list_duplicate:
            LOG.error(f"Duplicated files {list_duplicate}!")
            throw_error(ERROR.DUPLICATE_FILES)

        # now copy `*.uc` files from `sources` to `classes`, so UCC can process them
        for file in path_sources.rglob("*.uc"):
            try:
                copy2(file, path_classes)
            except Exception as e:
                LOG.error("Failed to copy the file: ", str(e))

    print_separator_box("COMPILING: " + r.mutatorName)

    # start the actual compilation! FINALLY!!!
    try:
        check_call(
            executable=r.path_ucc,
            args=["ucc", "make", "ini=" + COMPILATION_CONFIG_NAME, "-EXPORTCACHE"],
        )
    except CalledProcessError as e:
        LOG.error(str(e))
        throw_error(ERROR.COMPILATION_FAILED)

    # create INT files
    if r.bCreateINT:
        print_separator_box("Creating INT file!")
        try:
            check_call(
                executable=r.path_ucc,
                args=["ucc", "dumpint", r.path_compiled_file_u],
            )
        except CalledProcessError as e:
            LOG.error(str(e))

    # create UZ2 files
    if r.bMakeRedirect:
        print_separator_box("Creating UZ2 file!")
        try:
            check_call(
                executable=r.path_ucc,
                args=["ucc", "compress", r.path_compiled_file_u],
            )
        except CalledProcessError as e:
            LOG.error(str(e))

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
            LOG.error("Failed to move compiled files: " + str(e))

    if r.bMakeRelease:
        try:
            print(">>> Moving files to output directory.")
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
            LOG.error("Failed to create a redirect file in release folder: " + str(e))

    # remove the file from system after everything else is done
    if r.bMakeRedirect:
        try:
            print("\n>>> Moving redirect file to redirect directory.\n")
            copy_file(
                r.path_compiled_file_uz2, r.path_compile_dir.joinpath(REDIRECT_DIR_NAME)
            )
            safe_delete_file(r.path_compiled_file_uz2)
        except Exception as e:
            LOG.error("Failed to create a redirect file: " + str(e))


#################################################################################
#                              FUNCTION CALLS
#################################################################################

r: RuntimeVars


def main() -> None:
    global r

    try:
        r = RuntimeVars()
        # check if we have all configs and everything is fine
        # then assign global vars
        init_settings()
        # useful logs, if you want em
        print(r)
        # compile!
        compile_me()
        handle_files()
    except KeyboardInterrupt:
        LOG.info("Terminated by Ctrl - C")
    except Exception as e:
        LOG.critical(str(e))
    finally:
        input("\nPress any key to continue.\n")


if __name__ == "__main__":
    main()
