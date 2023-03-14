# Python version of my shit bat file
# Author    : NikC-
# Home repo : https://github.com/InsultingPros/KFCompileTool
# License   : https://www.gnu.org/licenses/gpl-3.0.en.html


#################################################################################
#                               IMPORTING
#################################################################################
import os
import shutil
import subprocess
import sys
from dataclasses import dataclass
from configparser import ConfigParser
from enum import Enum
from pathlib import Path

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
    # random package related
    pathCmpSystem: str = "fallback pathCmpSystem"
    pathFileU: str = "fallback pathFileU"
    pathFileUCL: str = "fallback pathFileUCL"
    pathFileUZ2: str = "fallback pathFileUZ2"
    pathFileINT: str = "fallback pathFileINT"
    pathFileGarbage: str = "fallback pathFileGarbage"
    # other
    pathMoveTo: str = "fallback pathMoveTo"

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

    def_Mod = {
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


class Utility:
    """Random utility functions"""

    # post compilation / failure cleanup
    def cleanup(self) -> None:
        # remove steamapp_id.txt, its being created every time
        util.delete_compile_dir_files(r.pathFileGarbage)

        # remove compfile, we don't need it
        util.delete_compile_dir_files(CMPL_CONFIG)

        if r.bICompileOutsideofKF:
            self.dir_remove(os.path.join(r.dir_Compile, r.mutatorName))

    def get_mod_file_types(self, dir_base_name: str, file_type: int) -> str:
        """Get file paths from type."""
        match file_type:
            case 1:
                ext = ".u"
            case 2:
                ext = ".ucl"
            case 3:
                ext = ".u.uz2"
            case 4:
                ext = ".int"
            case _:
                ext = "BAD EXTENSION"
        return os.path.join(dir_base_name, r.mutatorName + ext)

    def get_mod_file_name(self, file_type: int) -> str:
        """get file names from type"""
        match file_type:
            case 1:
                return r.mutatorName + ".u"
            case 2:
                return r.mutatorName + ".ucl"
            case 3:
                return r.mutatorName + ".u.uz2"
            case _:
                return "fallback name + extension!"

    # https://docs.python.org/3/library/shutil.html#rmtree-example
    def remove_readonly(self, func, path, _) -> None:
        """Clear the readonly bit and reattempt the removal"""
        Path(path).chmod(0o0200)
        func(path)

    def dir_remove(self, input_dir: str) -> None:
        """remove new created 'classes' folder on alternate dir style"""
        if Path(input_dir).exists():
            shutil.rmtree(input_dir, onerror=self.remove_readonly)

    def delete_compile_dir_files(self, file: str) -> None:
        """Check and delete the file"""
        if Path(os.path.join(r.pathCmpSystem, file)).is_file():
            try:
                Path(os.path.join(r.pathCmpSystem, file)).unlink()
            except PermissionError as e:
                sys.exit("Failed to delete the file: " + str(e))

    def get_system_dir(self, dir_base: str) -> str:
        """Get system directory"""
        return os.path.join(dir_base, "System")

    def copy_file(self, dir_source, dir_destination) -> None:
        if not Path(dir_source).is_file():
            return
        shutil.copy(dir_source, dir_destination)
        print("> Copied:  " + dir_source + "  --->  " + dir_destination)

    def get_dir_redirect(self, dir_input: str) -> str:
        """Get / create redirect directory in selected directory"""
        dir_destination = os.path.join(dir_input, REDIRECT_DIR_NAME)
        # check if path exist and create otherwise
        if not Path(dir_destination).exists():
            Path(dir_destination).mkdir()
        return dir_destination


class ConfigHelper(Utility, Types):
    """Class for working with config file"""

    def create_settings_file(self, input_dir):
        """Create DEFAULT config file if none found"""
        config = ConfigParser()
        # save the case
        config.optionxform = str

        config["Global"] = self.def_Global
        config["TestMut"] = self.def_Mod

        with open(input_dir, "w") as configfile:
            config.write(configfile, space_around_delimiters=False)

    def create_def_compile_ini(self, input_dir) -> None:
        """Create DEFAULT config file if none found"""
        # print(sys_dir)
        # make sure we don't have old files
        os.path.join(os.path.dirname(os.path.realpath(__file__)), CMPL_CONFIG)

        def write_line_to_config(text: str) -> None:
            """Write single line to file"""
            with open(CMPL_CONFIG, "a") as f:
                f.writelines([text + "\n"])

        def write_list_to_config(key: str, input_list: list[str]) -> None:
            """Add lines at the end of the file"""
            with open(CMPL_CONFIG, "a") as f:
                for x in input_list:
                    f.writelines([key + "=" + x + "\n"])

        def write_dict_to_config(input_dict: dict[str, str]) -> None:
            """write key-value from dictionary"""
            with open(CMPL_CONFIG, "a") as f:
                for k, v in input_dict.items():
                    f.writelines([k + "=" + v + "\n"])

        # SECTION 1
        write_line_to_config("[Editor.EditorEngine]")

        write_list_to_config("EditPackages", self.def_EditPackages)
        write_line_to_config("\n")

        # SECTION 2
        write_line_to_config("[Engine.Engine]")
        write_dict_to_config(self.EngineDict)
        write_line_to_config("\n")

        # SECTION 3
        write_line_to_config("[Core.System]")
        write_dict_to_config(self.SysDict)

        write_list_to_config("Paths", self.def_paths)
        write_list_to_config("Suppress", self.def_Suppress)
        write_line_to_config("\n")

        # SECTION 4
        # if we don't add this section, we will get some other garbage being written
        write_line_to_config("[ROFirstRun]")
        write_line_to_config("ROFirstRun=1094\n")

        shutil.move(CMPL_CONFIG, input_dir)


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
    # self directory
    dir_script: str = os.path.dirname(os.path.realpath(__file__))
    dir_settings_ini: str = os.path.join(dir_script, SETTINGS_FILE)
    # check if settings.ini exists in same directory
    if not Path(dir_settings_ini).is_file():
        cfghlp.create_settings_file(dir_settings_ini)
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

    # RANDOM
    r.pathCmpSystem = util.get_system_dir(r.dir_Compile)
    r.pathFileU = util.get_mod_file_types(r.pathCmpSystem, 1)
    r.pathFileUCL = util.get_mod_file_types(r.pathCmpSystem, 2)
    r.pathFileUZ2 = util.get_mod_file_types(r.pathCmpSystem, 3)
    r.pathFileINT = util.get_mod_file_types(r.pathCmpSystem, 4)
    r.pathFileGarbage = os.path.join(r.pathCmpSystem, "steam_appid.txt")

    # make sure there are no old files
    util.delete_compile_dir_files(CMPL_CONFIG)

    # update editPackages and create the kf.ini
    Types.def_EditPackages.extend(r.EditPackages.split(","))
    cfghlp.create_def_compile_ini(r.pathCmpSystem)


def compile_me() -> None:
    # delete files before compilation start, since UCC is ghei
    util.delete_compile_dir_files(r.pathFileU)
    util.delete_compile_dir_files(r.pathFileUCL)

    dir_source: str = os.path.join(r.dir_Classes, r.mutatorName)
    dir_destination: str = os.path.join(r.dir_Compile, r.mutatorName)
    # if our mod files are in other directory, just copy-paste everything from there

    # if mod folder is outside, delete old dir and copy-paste new one
    if r.bICompileOutsideofKF:
        util.dir_remove(dir_destination)
        shutil.copytree(
            dir_source,
            dir_destination,
            copy_function=shutil.copy,
            ignore=shutil.ignore_patterns(*IGNORE_LIST),
        )

    # if we use alternative directory style, we need to do some work
    if r.bAltDirectories:
        sources: str = os.path.join(dir_source, "sources")
        if not Path(sources).exists():
            throw_error(ERROR.WRONG_DIR_STYLE)

        classes: str = os.path.join(dir_destination, "Classes")
        util.dir_remove(classes)
        Path(classes).mkdir()
        # now copy everything!
        for path, subdir, files in os.walk(sources):
            for name in files:
                filename = os.path.join(path, name)
                shutil.copy2(filename, classes)

    print_separator_box("COMPILING: " + r.mutatorName)

    ucc: str = os.path.join(r.pathCmpSystem, "UCC.exe")
    # check if we have UCC
    if not Path(ucc).is_file():
        throw_error(ERROR.NO_UCC)

    # start the actual compilation! FINALLY!!!
    try:
        subprocess.run([ucc, "make", "ini=" + CMPL_CONFIG, "-EXPORTCACHE"], check=True)
    except subprocess.CalledProcessError as e:
        print(str(e))
        util.cleanup()
        throw_error(ERROR.COMPILATION_FAILED)

    # create INT files
    if r.bCreateINT:
        try:
            print_separator_box("Creating INT file!")
            os.chdir(r.pathCmpSystem)
            subprocess.run(["ucc", "dumpint", r.pathFileU], check=True)
        except subprocess.CalledProcessError as e:
            print(str(e))

    # create UZ2 files
    if r.bMakeRedirect:
        try:
            print_separator_box("Creating UZ2 file!")
            os.chdir(r.pathCmpSystem)
            subprocess.run(["ucc", "compress", r.pathFileU], check=True)
        except subprocess.CalledProcessError as e:
            print(str(e))

    # cleanup!
    util.cleanup()


def handle_files() -> None:
    # announce
    print_separator_box("MOVING FILES")

    # do we want files being moved to desired client / server directory?
    if r.bMoveFiles:
        try:
            print(">>> Moving files to CLIENT directory.\n")
            dir_destination: str = util.get_system_dir(r.dir_MoveTo)
            util.copy_file(r.pathFileU, dir_destination)
            util.copy_file(r.pathFileUCL, dir_destination)
            util.copy_file(r.pathFileINT, dir_destination)
        except Exception as e:
            print("Failed to move compiled files: " + str(e))

    if r.bMakeRelease:
        try:
            print(">>> Moving files to output directory.\n")
            x: str = os.path.join(r.dir_ReleaseOutput, r.mutatorName)
            # if 'Redirect' folder doesn't exist, create it
            if not Path(x).exists():
                Path(x).mkdir()
            # copy files
            util.copy_file(r.pathFileU, x)
            util.copy_file(r.pathFileUCL, x)
        except Exception as e:
            print("Failed to create a release file: " + str(e))

        if r.bMakeRedirect:
            try:
                y = os.path.join(x, REDIRECT_DIR_NAME)
                if not Path(y).exists():
                    Path(y).mkdir()
                # copy files
                util.copy_file(r.pathFileUZ2, y)
            except Exception as e:
                print("Failed to create a redirect file in release folder: " + str(e))

    # remove the file from system after everything else is done
    if r.bMakeRedirect:
        try:
            print("\n>>> Moving redirect file to redirect directory.\n")
            util.copy_file(r.pathFileUZ2, r.dir_Compile + "/" + REDIRECT_DIR_NAME)
            util.delete_compile_dir_files(r.pathFileUZ2)
        except Exception as e:
            print("Failed to create a redirect file: " + str(e))


util: Utility
cfghlp: ConfigHelper
r: RuntimeVars


def init_classes() -> None:
    global util
    global cfghlp
    global r

    util = Utility()
    cfghlp = ConfigHelper()
    r = RuntimeVars()


#################################################################################
#                              FUNCTION CALLS
#################################################################################


def main() -> None:
    init_classes()
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
