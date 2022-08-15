# Python version of my shit bat file
# check updates: https://github.com/InsultingPros/KFCompileTool
# linted by http://mypy-lang.org/


#################################################################################
#                               IMPORTING
#################################################################################
import os, shutil, sys, subprocess
from configparser import ConfigParser
from pathlib import Path

#################################################################################
#                              'CONSTANTS'
#################################################################################
LINE_SEPARATOR:    str       = '######################################################'
SETTINGS_FILE:     str       = 'CompileSettings.ini'
"""Settings file for this script, contains client-server directories and mods info"""
CMPL_CONFIG:       str       = 'kfcompile.ini'
"""Game config that UCC.exe uses for compilation, contains EditPackages lines and the most minimal setup"""
REDIRECT_DIR_NAME: str       = 'Redirect'
"""Folder name for redirect files"""
IGNORE_LIST:       list[str] = ['.git','*.md','Docs', 'LICENSE']
"""Filter for files-directories, so we copy-paste only source files"""

#################################################################################
#                                UTILITY
#################################################################################

class runtimeVars():
    """Сontains 'runtime' variables"""
    # Global
    mutatorName:            str  =   'fallback mutatorName'
    dir_Compile:            str  =   'fallback dir_Compile'
    dir_MoveTo:             str  =   'fallback dir_MoveTo'
    dir_ReleaseOutput:      str  =   'fallback dir_ReleaseOutput'
    dir_Classes:            str  =   'fallback dir_Classes'
    # sections
    EditPackages:           str  =   'fallback EditPackages'
    bICompileOutsideofKF:   bool =   False
    bAltDirectories:        bool =   False
    bMoveFiles:             bool =   False
    bCreateINT:             bool =   False
    bMakeRedirect:          bool =   False
    bMakeRelease:           bool =   False
    # random package related
    pathCmpSystem:             str  =   'fallback pathCmpSystem'
    pathFileU:              str  =   'fallback pathFileU'
    pathFileUCL:            str  =   'fallback pathFileUCL'
    pathFileUZ2:            str  =   'fallback pathFileUZ2'
    pathFileINT:            str  =   'fallback pathFileINT'
    pathFileGarbage:        str  =   'fallback pathFileGarbage'
    # other
    pathMoveTo:             str  =   'fallback pathMoveTo'


class types():
    """Contains lists, dicts used to populate kfcompile.ini / CompileSettings.ini"""
    # CompileSettings.ini
    def_Global: dict[str, str] = {'mutatorName'          :   'TestMut',
                                  'dir_Compile'          :   r'D:\Games\SteamLibrary\steamapps\common\KillingFloor',
                                  'dir_MoveTo'           :   r'D:\Games\KF Dedicated Server',
                                  'dir_ReleaseOutput'    :   r'C:\Users\USER\Desktop\Mutators',
                                  'dir_Classes'          :   r'C:\Users\Shtoyan\Desktop\Projects'}

    def_Mod                   = {'EditPackages'         :   'TestMutParent,TestMut',
                                 'bICompileOutsideofKF' :   False,
                                 'bAltDirectories'      :   False,
                                 'bMoveFiles'           :   False,
                                 'bCreateINT'           :   False,
                                 'bMakeRedirect'        :   False,
                                 'bMakeRelease'         :   False}

    # kfcompile.ini
    # [Editor.EditorEngine]
    def_EditPackages: list[str] = ['Core', 'Engine', 'Fire', 'Editor', 'UnrealEd', 'IpDrv', 'UWeb', 'GamePlay',
                                   'UnrealGame', 'XGame', 'XInterface', 'XAdmin', 'XWebAdmin', 'GUI2K4', 'xVoting',
                                   'UTV2004c', 'UTV2004s', 'ROEffects', 'ROEngine', 'ROInterface', 'Old2k4', 'KFMod',
                                   'KFChar', 'KFGui', 'GoodKarma', 'KFMutators', 'KFStoryGame', 'KFStoryUI',
                                   'SideShowScript', 'FrightScript']

    # [Engine.Engine]
    # this setting is enough
    EngineDict: dict[str, str] = {'EditorEngine'    : 'Editor.EditorEngine'}

    # [Core.System]
    # this too
    SysDict: dict[str, str]    = {'CacheRecordPath' : '../System/*.ucl'}

    def_paths: list[str]    = ['../System/*.u', '../Maps/*.rom', '../TestMaps/*.rom', '../Textures/*.utx',
                               '../Sounds/*.uax', '../Music/*.umx', '../StaticMeshes/*.usx', '../Animations/*.ukx' ,
                               '../Saves/*.uvx', '../Textures/Old2k4/*.utx', '../Sounds/Old2k4/*.uax',
                               '../Music/Old2k4/*.umx', '../StaticMeshes/Old2k4/*.usx', '../Animations/Old2k4/*.ukx','../KarmaData/Old2k4/*.ka']

    def_Suppress: list[str] = ['DevLoad', 'DevSave']


class utility():
    """Random utility functions"""
    # post compilation / failure cleanup
    def cleanup(self) -> None:
        # remove steamapp_id.txt, its being created every time
        util.deleteCompileDirFiles(r.pathFileGarbage)

         # remove compfile, we don't need it
        util.deleteCompileDirFiles(CMPL_CONFIG)

        if r.bICompileOutsideofKF:
            self.dir_remove(os.path.join(r.dir_Compile, r.mutatorName))

    def getModFileTypes(self, dir: str, type: int) -> str:
        """Get file paths from type."""
        ext: str
        match type:
            case 1:
                ext = '.u'
            case 2:
                ext = '.ucl'
            case 3:
                ext = '.u.uz2'
            case 4:
                ext = '.int'
        return os.path.join(dir, r.mutatorName + ext)

    def getModFileName(self, type: int) -> str:
        """get file names from type"""
        match type:
            case 1:
                return r.mutatorName + '.u'
            case 2:
                return r.mutatorName + '.ucl'
            case 3:
                return r.mutatorName + '.u.uz2'
            case _:
                return 'fallback name + extension!'

    # https://docs.python.org/3/library/shutil.html#rmtree-example
    def remove_readonly(self, func, path, _) -> None:
        """Clear the readonly bit and reattempt the removal"""
        Path(path).chmod(0o0200)
        func(path)

    def dir_remove(self, dir: str) -> None:
        """remove new created 'classes' folder on alternate dir style"""
        if Path(dir).exists():
            shutil.rmtree(dir, onerror=self.remove_readonly)

    def deleteCompileDirFiles(self, file: str) -> None:
        """Check and delete the file"""
        if Path(os.path.join(r.pathCmpSystem, file)).is_file():
            Path(os.path.join(r.pathCmpSystem, file)).unlink()

    def getSysDir(self, basedir: str) -> str:
        """Get system directory"""
        return os.path.join(basedir, 'System')

    def copyFile4System(self, src, dest) -> None:
        if not Path(src).is_file():
            return
        shutil.copy(src, dest)
        print('> Copied:  ' + src + '  --->  ' + dest)

    def get_dirRedirect(self, dir: str) -> str:
        """Get / create redirect directory in selected directory"""
        destdir = os.path.join(dir, REDIRECT_DIR_NAME)
        # check if path exist and create otherwise
        if not Path(destdir).exists():
            Path(destdir).mkdir()
        return destdir


class configHelper(utility, types):
    """Class for working with config file"""

    def create_settingsFile(self, dir):
        """Create DEFAULT config file if none found"""
        config = ConfigParser()
        # save the case
        config.optionxform = str

        config['Global']  = self.def_Global
        config['TestMut'] = self.def_Mod

        with open(dir, 'w') as configfile:
            config.write(configfile, space_around_delimiters=False)

    def create_defMincompfile(self, sys_dir) -> None:
        """Create DEFAULT config file if none found"""
        # print(sys_dir)
        # make sure we don't have old files
        os.path.join(os.path.dirname(os.path.realpath(__file__)), CMPL_CONFIG)

        def wStrToConfig(text: str) -> None:
            """Write single line to file"""
            with open(CMPL_CONFIG, 'a') as f:
                f.writelines([text + '\n'])

        def wListToConfig(key: str, list: list[str]) -> None:
            """Add lines at the end of the file"""
            with open(CMPL_CONFIG, 'a') as f:
                for x in list:
                    f.writelines([key + '=' + x + '\n'])

        def wDictToConfig(dict: dict[str, str]) -> None:
            """write key-value from dictionary"""
            with open(CMPL_CONFIG, 'a') as f:
                for k, v in dict.items():
                    f.writelines([k + '=' + v + '\n'])

        # SECTION 1
        wStrToConfig('[Editor.EditorEngine]')

        wListToConfig('EditPackages', self.def_EditPackages)
        wStrToConfig('\n')

        # SECTION 2
        wStrToConfig('[Engine.Engine]')
        wDictToConfig(self.EngineDict)
        wStrToConfig('\n')

        # SECTION 3
        wStrToConfig('[Core.System]')
        wDictToConfig(self.SysDict)

        wListToConfig('Paths', self.def_paths)
        wListToConfig('Suppress', self.def_Suppress)
        wStrToConfig('\n')

        # SECTION 4
        # if we don't add this section, we will get some other garbage being written
        wStrToConfig('[ROFirstRun]')
        wStrToConfig('ROFirstRun=1094\n')

        shutil.move(CMPL_CONFIG, sys_dir)


class Debug():
    def throwError(self, err: int):
        """Throw human readable error message."""
        prefix :str = '>>> TERMINATION WARNING: '
        match err:
            case 0:
                print(prefix + SETTINGS_FILE + ' was not found. We created a new file for you, in the same directory.')
                print('>>> PLEASE go and edit it to fit your neeeds.')
            case 1:
                print(prefix + 'Global section not found in CompileSettings.ini.')
            case 2:
                print(prefix + r.mutatorName + ' section not found in CompileSettings.ini')
            case 3:
                print(prefix + 'UCC.exe was not found in compile directory. Install SDK and retry!')
            case 4:
                print(prefix + 'Alternative Directory is True, but `sources` folder NOT FOUND!')
            case 5:
                print(prefix + 'Compilation FAILED!')
            case _:
                print(prefix + 'undefined error code!')
        input('Press any key to continue.')
        exit()

    def print_separatorBox(self, msg: str) -> None:
        """Print nice message box"""
        print('\n' + LINE_SEPARATOR + '\n')
        print(msg)
        print('\n' + LINE_SEPARATOR + '\n')

    def printSettings(self) -> None:
        """Print Settings file contents."""
        print(LINE_SEPARATOR + '\n')
        print(SETTINGS_FILE + '\n')
        #  global
        print('mutatorName          :', r.mutatorName)
        print('dir_Compile          :', r.dir_Compile)
        print('dir_MoveTo           :', r.dir_MoveTo)
        print('dir_ReleaseOutput    :', r.dir_ReleaseOutput)
        print('dir_Classes          :', r.dir_Classes, '\n')
        # sections
        print('EditPackages         :', r.EditPackages)
        print('bICompileOutsideofKF :', r.bICompileOutsideofKF)
        print('bAltDirectories      :', r.bAltDirectories)
        print('bMoveFiles           :', r.bMoveFiles)
        print('bCreateINT           :', r.bCreateINT)
        print('bMakeRedirect        :', r.bMakeRedirect)
        print('bMakeRelease         :', r.bMakeRelease)

#################################################################################
#                                FUNCTIONS
#################################################################################

def initSettings() -> None:
    """Read config file and define all variables."""
    # self directory
    dirScript: str = os.path.dirname(os.path.realpath(__file__))
    dirSettingsIni: str = os.path.join(dirScript, SETTINGS_FILE)
    # check if settings.ini exists in same directory
    if not Path(dirSettingsIni).is_file():
        cfghlp.create_settingsFile(dirSettingsIni)
        dbg.throwError(0)

    config = ConfigParser()
    config.read(dirSettingsIni)
    # get global section and set main vars
    if not config.has_section('Global'):
        dbg.throwError(1)

    # GLOBAL
    # accept cmdline arguments
    if len(sys.argv) == 1:
        r.mutatorName       =   config['Global']['mutatorName']
    else:
        r.mutatorName       =   sys.argv[1]

    r.dir_Compile           =   config['Global']['dir_Compile']
    r.dir_MoveTo            =   config['Global']['dir_MoveTo']
    r.dir_ReleaseOutput     =   config['Global']['dir_ReleaseOutput']
    r.dir_Classes           =   config['Global']['dir_Classes']

    # SECTIONS
    # check if exist
    if not config.has_section(r.mutatorName):
        dbg.throwError(2)

    r.EditPackages          =   config[r.mutatorName]['EditPackages']
    r.bICompileOutsideofKF  =   config[r.mutatorName].getboolean('bICompileOutsideofKF')
    r.bAltDirectories       =   config[r.mutatorName].getboolean('bAltDirectories')
    r.bMoveFiles            =   config[r.mutatorName].getboolean('bMoveFiles')
    r.bCreateINT            =   config[r.mutatorName].getboolean('bCreateINT')
    r.bMakeRedirect         =   config[r.mutatorName].getboolean('bMakeRedirect')
    r.bMakeRelease          =   config[r.mutatorName].getboolean('bMakeRelease')

    # RANDOM
    r.pathCmpSystem         =   util.getSysDir(r.dir_Compile)
    r.pathFileU             =   util.getModFileTypes(r.pathCmpSystem, 1)
    r.pathFileUCL           =   util.getModFileTypes(r.pathCmpSystem, 2)
    r.pathFileUZ2           =   util.getModFileTypes(r.pathCmpSystem, 3)
    r.pathFileINT           =   util.getModFileTypes(r.pathCmpSystem, 4)
    r.pathFileGarbage       =   os.path.join(r.pathCmpSystem, 'steam_appid.txt')

    # make sure there are no old files
    util.deleteCompileDirFiles(CMPL_CONFIG)

    # update editPackages and create the kf.ini
    types.def_EditPackages.extend(r.EditPackages.split(','))
    cfghlp.create_defMincompfile(r.pathCmpSystem)


def compileMe() -> None:
    # delete files before compilation start, since UCC is ghei
    util.deleteCompileDirFiles(r.pathFileU)
    util.deleteCompileDirFiles(r.pathFileUCL)

    srcdir: str = os.path.join(r.dir_Classes, r.mutatorName)
    destdir: str = os.path.join(r.dir_Compile, r.mutatorName)
    # if our mod files are in other directory, just copy-paste everything from there

    # if mod folder is outside, delete old dir and copy-paste new one
    if r.bICompileOutsideofKF:
        util.dir_remove(destdir)
        shutil.copytree(srcdir, destdir, copy_function=shutil.copy, ignore=shutil.ignore_patterns(*IGNORE_LIST))

    # if we use alternative directory style, we need to do some work
    if r.bAltDirectories:
        sources: str = os.path.join(srcdir, 'sources')
        if not Path(sources).exists():
            dbg.throwError(4)

        classes: str = os.path.join(destdir, 'Classes')
        util.dir_remove(classes)
        Path(classes).mkdir()
        # now copy everything!
        for path, subdirs, files in os.walk(sources):
            for name in files:
                filename = os.path.join(path, name)
                shutil.copy2(filename, classes)

    dbg.print_separatorBox('COMPILING: ' + r.mutatorName)

    ucc: str = os.path.join(r.pathCmpSystem, 'UCC.exe')
    # check if we have UCC
    if not Path(ucc).is_file():
        dbg.throwError(3)

    # start the actual compilation! FINALLY!!!
    subprocess.run([ucc, 'make', 'ini=' + CMPL_CONFIG, '-EXPORTCACHE'])

    # let's just check if package.u is created or not
    # else we failed -> cleanup and shut down
    if not Path(os.path.join(r.pathCmpSystem, r.pathFileU)).exists():
        util.cleanup()
        dbg.throwError(5)

    # create INT files
    if r.bCreateINT:
        dbg.print_separatorBox('Creating INT file!')
        os.chdir(r.pathCmpSystem)
        subprocess.run(['ucc', 'dumpint', r.pathFileU])

    # create UZ2 files
    if r.bMakeRedirect:
        dbg.print_separatorBox('Creating UZ2 file!')
        os.chdir(r.pathCmpSystem)
        subprocess.run(['ucc', 'compress', r.pathFileU])

    # cleanup!
    util.cleanup()


def handle_Files() -> None:
    # announce
    dbg.print_separatorBox('MOVING FILES')

    # do we want files being moved to desired client / server directory?
    if r.bMoveFiles:
        print('>>> Moving files to CLIENT directory.\n')
        dest: str = util.getSysDir(r.dir_MoveTo)
        util.copyFile4System(r.pathFileU,   dest)
        util.copyFile4System(r.pathFileUCL, dest)
        util.copyFile4System(r.pathFileINT, dest)

    if r.bMakeRelease:
        print('>>> Moving files to output directory.\n')
        x: str = os.path.join(r.dir_ReleaseOutput, r.mutatorName)
        # if 'Redirect' folder doesn't exist, create it
        if not Path(x).exists():
            Path(x).mkdir()
        # copy files
        util.copyFile4System(r.pathFileU,   x)
        util.copyFile4System(r.pathFileUCL, x)

        if r.bMakeRedirect:
            y = os.path.join(x, REDIRECT_DIR_NAME)
            if not Path(y).exists():
                Path(y).mkdir()
            # copy files
            util.copyFile4System(r.pathFileUZ2, y)

    # remove the file from system after everything else is done
    if r.bMakeRedirect:
        print('\n>>> Moving redirect file to redirect directory.\n')
        util.copyFile4System(r.pathFileUZ2, r.dir_Compile + '/' + REDIRECT_DIR_NAME)
        util.deleteCompileDirFiles(r.pathFileUZ2)

    # exit the script, everything is done
    input('\n' + 'Press any key to continue.')


#################################################################################
#                              FUNCTION CALLS
#################################################################################

util = utility()
dbg = Debug()
cfghlp = configHelper()
r = runtimeVars()

# check if we have all configs and everything is fine
# then assign global vars
initSettings()
# useful logs, if you want em
dbg.printSettings()
# compile!
compileMe()
handle_Files()