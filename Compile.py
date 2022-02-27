# Python version of my shit bat file
# check updates: https://github.com/InsultingPros/KFCompileTool


#################################################################################
#                               IMPORTING
#################################################################################
import os, shutil, sys, subprocess, stat
from configparser import ConfigParser

#################################################################################
#                              'CONSTANTS'
#################################################################################
_lineSeparator      :str       =   '######################################################'
_settingsFile       :str       =   'CompileSettings.ini'
_mincompfile        :str       =   'kfcompile.ini'
_redirectFolderName :str       =   'Redirect'
_bDebug             :bool      =   True
_list               :list[str] =   ['.git','*.md','Docs', 'LICENSE']

#################################################################################
#                                UTILITY
#################################################################################

# contains 'runtime' variables
class runtimeVars():
    # Global
    mutatorName            :str  =   'fallback mutatorName'
    dir_Compile            :str  =   'fallback dir_Compile'
    dir_MoveTo             :str  =   'fallback dir_MoveTo'
    dir_ReleaseOutput      :str  =   'fallback dir_ReleaseOutput'
    dir_Classes            :str  =   'fallback dir_Classes'
    # sections
    EditPackages           :str  =   'fallback EditPackages'
    bICompileOutsideofKF   :bool =   False
    bAltDirectories        :bool =   False
    bMoveFiles             :bool =   False
    bCreateINT             :bool =   False
    bMakeRedirect          :bool =   False
    bMakeRelease           :bool =   False
    # random package related
    pathSystem             :str  =   'fallback pathSystem'
    pathFileU              :str  =   'fallback pathFileU'
    pathFileUCL            :str  =   'fallback pathFileUCL'
    pathFileUZ2            :str  =   'fallback pathFileUZ2'
    pathFileINT            :str  =   'fallback pathFileINT'
    pathFileGarbage        :str  =   'fallback pathFileGarbage'
    # other
    pathMoveTo             :str  =   'fallback pathMoveTo'


# contains lists, dicts used to populate kfcompile.ini / CompileSettings.ini
class types():
    # CompileSettings.ini
    def_Global              =   {'mutatorName'          :   'TestMut',
                                 'dir_Compile'          :   r'D:\Games\SteamLibrary\steamapps\common\KillingFloor',
                                 'dir_MoveTo'           :   r'D:\Games\KF Dedicated Server',
                                 'dir_ReleaseOutput'    :   r'C:\Users\USER\Desktop\Mutators',
                                 'dir_Classes'          :   r'C:\Users\Shtoyan\Desktop\Projects'}

    def_Mod                 =   {'EditPackages'         :   'TestMutParent,TestMut',
                                 'bICompileOutsideofKF' :   False,
                                 'bAltDirectories'      :   False,
                                 'bMoveFiles'           :   False,
                                 'bCreateINT'           :   False,
                                 'bMakeRedirect'        :   False,
                                 'bMakeRelease'         :   False}

    # kfcompile.ini
    # [Editor.EditorEngine]
    def_EditPackages        =   ['Core', 'Engine', 'Fire', 'Editor', 'UnrealEd', 'IpDrv', 'UWeb', 'GamePlay',
                                 'UnrealGame', 'XGame', 'XInterface', 'XAdmin', 'XWebAdmin', 'GUI2K4', 'xVoting',
                                 'UTV2004c', 'UTV2004s', 'ROEffects', 'ROEngine', 'ROInterface', 'Old2k4', 'KFMod',
                                 'KFChar', 'KFGui', 'GoodKarma', 'KFMutators', 'KFStoryGame', 'KFStoryUI',
                                 'SideShowScript', 'FrightScript']

    # [Engine.Engine]
    # this setting is enough
    EngineDict              =   {'EditorEngine'         :   'Editor.EditorEngine'}

    # [Core.System]
    # this too
    SysDict                 =   { 'CacheRecordPath'     :   '../System/*.ucl'}

    def_paths               =   ['../System/*.u', '../Maps/*.rom', '../TestMaps/*.rom', '../Textures/*.utx',
                                 '../Sounds/*.uax', '../Music/*.umx', '../StaticMeshes/*.usx', '../Animations/*.ukx' ,
                                 '../Saves/*.uvx', '../Textures/Old2k4/*.utx', '../Sounds/Old2k4/*.uax',
                                 '../Music/Old2k4/*.umx', '../StaticMeshes/Old2k4/*.usx', '../Animations/Old2k4/*.ukx','../KarmaData/Old2k4/*.ka']

    def_Suppress            =   ['DevLoad', 'DevSave']


# random utility functions
class utility():
    # post compilation / failure cleanup
    def cleanup(self):
        # remove steamapp_id.txt, its being created every time
        util.deleteCompileDirFiles(r.pathFileGarbage)

         # remove compfile, we don't need it
        util.deleteCompileDirFiles(_mincompfile)

        if r.bICompileOutsideofKF is True:
            self.dir_remove(os.path.join(r.dir_Compile, r.mutatorName))

    # check if compilation was successfull
    # let's just check if package.u is created or not
    def compilationFailed(self) -> bool:
        dir = self.getSysDir(r.dir_Compile)
        ufile = self.getModFileTypes(dir, 1)
        if not os.path.exists(os.path.join(dir, ufile)):
            return True
        return False

    # get file paths from type
    def getModFileTypes(self, dir: str, type: int) -> str:
        if type == 1:
            ext = '.u'
        elif type == 2:
            ext = '.ucl'
        elif type == 3:
            ext = '.u.uz2'
        elif type == 4:
            ext = '.int'
        return os.path.join(dir, r.mutatorName + ext)

    # get file names from type
    def getModFileName(self, type: int) -> str:
        if type == 1:
            return r.mutatorName + '.u'
        elif type == 2:
            return r.mutatorName + '.ucl'
        elif type == 3:
            return r.mutatorName + '.u.uz2'
        else:
            return 'fallback name + extension!'

    # https://docs.python.org/3/library/shutil.html#rmtree-example
    def remove_readonly(self, func, path, _):
        # Clear the readonly bit and reattempt the removal
        os.chmod(path, stat.S_IWRITE)
        func(path)

    # remove new created 'classes' folder on alternate dir style
    def dir_remove(self, dir: str):
        if os.path.exists(dir):
            shutil.rmtree(dir, onerror=self.remove_readonly)

    # check and delete the file
    def deleteCompileDirFiles(self, file):
        if os.path.isfile(os.path.join(r.pathSystem, file)):
            os.remove(os.path.join(r.pathSystem, file))

    # get system directory
    def getSysDir(self, basedir) -> str:
        return os.path.join(basedir, 'System')

    # return list from file content
    def getReadLines(self, filename) -> list:
        with open(filename, 'r') as f:
            contents = f.readlines()
        return contents

    def copyFile4System(self, src, dest):
        if os.path.isfile(src) is False:
            return
        shutil.copy(src, dest)
        print('> Copied:  ' + src + '  --->  ' + dest)

    # get / create redirect directory in selected directory
    def get_dirRedirect(self, dir: str) -> str:
        destdir = os.path.join(dir, _redirectFolderName)
        # check if path exist and create otherwise
        if not os.path.exists(destdir):
            os.makedirs(destdir)
        return destdir


# class for working with config file
class configHelper(utility, types):
    # create DEFAULT config file if none found
    def create_settingsFile(self, dir):
        config = ConfigParser()
        # save the case
        config.optionxform = str

        config['Global']  = self.def_Global
        config['TestMut'] = self.def_Mod

        with open(dir, 'w') as configfile:
            config.write(configfile, space_around_delimiters=False)

        print('>>> WARING: ' + _settingsFile + ' was created in same directory. PLEASE go and edit it to fit your neeeds.')

    # create DEFAULT config file if none found
    def create_defMincompfile(self, sys_dir):
        # print(sys_dir)
        # make sure we don't have old files
        os.path.join(os.getcwd(), _mincompfile)

        # write single line
        def write_string2config(text: str):
            with open(_mincompfile, 'a') as f:
                f.writelines([text + '\n'])

        # add lines at the end of the file
        def write_list2config(key, list):
            with open(_mincompfile, 'a') as f:
                for x in list:
                    f.writelines([key + '=' + x + '\n'])

        # write key-value from dictionary
        def write_dict2config(dict):
            with open(_mincompfile, 'a') as f:
                for k, v in dict.items():
                    f.writelines([k + '=' + v + '\n'])

        # SECTION 1
        write_string2config('[Editor.EditorEngine]')

        write_list2config('EditPackages', self.def_EditPackages)
        write_string2config('\n')

        # SECTION 2
        write_string2config('[Engine.Engine]')
        write_dict2config(self.EngineDict)
        write_string2config('\n')

        # SECTION 3
        write_string2config('[Core.System]')
        write_dict2config(self.SysDict)

        write_list2config('Paths', self.def_paths)
        write_list2config('Suppress', self.def_Suppress)
        write_string2config('\n')

        # SECTION 4
        # if we don't add this section, we will get some other garbage being written
        write_string2config('[ROFirstRun]')
        write_string2config('ROFirstRun=1094\n')

        shutil.move(_mincompfile, sys_dir)


class Debug():
    # stop right here
    def catchError(self, err: int):
        prefix :str = '>>> TERMINATION WARNING: '
        if err == 0:
            print(prefix + 'CompileSettings.ini was not found.')
        elif err == 1:
            print(prefix + 'Global section not found in CompileSettings.ini.')
        elif err == 2:
            print(prefix + r.mutatorName + ' section not found in CompileSettings.ini')
        elif err == 3:
            print(prefix + 'UCC.exe was not found in compile directory. Install SDK and retry!')
        elif err == 4:
            print(prefix + 'Alternative Directory is True, but `sources` folder NOT FOUND!')
        elif err == 5:
            print(prefix + 'Compilation FAILED!')
        os.system('pause')
        exit()

    # nice message box
    def print_separatorBox(self, msg: str):
        print('\n' + _lineSeparator + '\n')
        print(msg)
        print('\n' + _lineSeparator + '\n')

    # DEBUG / info
    def debug_Logs(self):
        if _bDebug is True:
            # type some information
            print(_lineSeparator + '\n')
            print(_settingsFile)
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

# read config file and define all variables
def initSettings():
    # self directory
    dirScript = os.path.dirname(os.path.realpath(__file__))
    dirSettingsIni = os.path.join(dirScript, _settingsFile)
    # check if settings.ini exists in same directory
    if os.path.isfile(dirSettingsIni) is False:
        cfghlp.create_settingsFile(dirSettingsIni)
        dbg.catchError(0)

    config = ConfigParser()
    config.read(dirSettingsIni)
    # get global section and set main vars
    if config.has_section('Global') is False:
        dbg.catchError(1)

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
    if config.has_section(r.mutatorName) is False:
        dbg.catchError(2)

    r.EditPackages          =   config[r.mutatorName]['EditPackages']
    r.bICompileOutsideofKF  =   config[r.mutatorName].getboolean('bICompileOutsideofKF')
    r.bAltDirectories       =   config[r.mutatorName].getboolean('bAltDirectories')
    r.bMoveFiles            =   config[r.mutatorName].getboolean('bMoveFiles')
    r.bCreateINT            =   config[r.mutatorName].getboolean('bCreateINT')
    r.bMakeRedirect         =   config[r.mutatorName].getboolean('bMakeRedirect')
    r.bMakeRelease          =   config[r.mutatorName].getboolean('bMakeRelease')

    # RANDOM
    r.pathSystem            =   util.getSysDir(r.dir_Compile)
    r.pathFileU             =   util.getModFileTypes(r.pathSystem, 1)
    r.pathFileUCL           =   util.getModFileTypes(r.pathSystem, 2)
    r.pathFileUZ2           =   util.getModFileTypes(r.pathSystem, 3)
    r.pathFileINT           =   util.getModFileTypes(r.pathSystem, 4)
    r.pathFileGarbage       =   os.path.join(r.pathSystem, 'steam_appid.txt')

    # make sure there are no old files
    util.deleteCompileDirFiles(_mincompfile)

    # update editPackages and create the kf.ini
    types.def_EditPackages.extend(r.EditPackages.split(','))
    cfghlp.create_defMincompfile(r.pathSystem)


def compileMe():
    # delete files before compilation start, since UCC is ghei
    util.deleteCompileDirFiles(r.pathFileU)
    util.deleteCompileDirFiles(r.pathFileUCL)

    srcdir = os.path.join(r.dir_Classes, r.mutatorName)
    destdir = os.path.join(r.dir_Compile, r.mutatorName)
    # if our mod files are in other directory, just copy-paste everything from there

    # if mod folder is outside, delete old dir and copy-paste new one
    if r.bICompileOutsideofKF is True:
        util.dir_remove(destdir)
        shutil.copytree(srcdir, destdir, copy_function=shutil.copy, ignore=shutil.ignore_patterns(*_list))

    # if we use alternative directory style, we need to do some work
    if r.bAltDirectories is True:
        sources = os.path.join(srcdir, 'sources')
        if os.path.exists(sources) is False:
            dbg.catchError(4)

        classes = os.path.join(destdir, 'Classes')
        util.dir_remove(classes)
        os.makedirs(classes)
        # now copy everything!
        for path, subdirs, files in os.walk(sources):
            for name in files:
                filename = os.path.join(path, name)
                shutil.copy2(filename, classes)

    dbg.print_separatorBox('COMPILING: ' + r.mutatorName)

    ucc = os.path.join(r.pathSystem, 'UCC.exe')
    # check if we have UCC
    if os.path.isfile(ucc) is False:
        dbg.catchError(3)

    # start the actual compilation! FINALLY!!!
    subprocess.run([ucc, 'make', 'ini=' + _mincompfile, '-EXPORTCACHE'])

    # we failed here, cleanup and shut down
    if util.compilationFailed() is True:
        util.cleanup()
        dbg.catchError(5)

    # create INT files
    if r.bCreateINT is True:
        dbg.print_separatorBox('Creating INT file!')
        os.chdir(r.pathSystem)
        subprocess.run(['ucc', 'dumpint', r.pathFileU])

    # create UZ2 files
    if r.bMakeRedirect is True:
        dbg.print_separatorBox('Creating UZ2 file!')
        os.chdir(r.pathSystem)
        subprocess.run(['ucc', 'compress', r.pathFileU])

    # cleanup!
    util.cleanup()


def handle_Files():
    # announce
    dbg.print_separatorBox('MOVING FILES')

    # get System dir
    sys = util.getSysDir(r.dir_Compile)
    # get file paths
    dir_uFile   = util.getModFileTypes(sys, 1)
    dir_uclFile = util.getModFileTypes(sys, 2)
    dir_uz2file = util.getModFileTypes(sys, 3)
    dir_intFile = util.getModFileTypes(sys, 4)

    # do we want files being moved to desired client / server directory?
    if r.bMoveFiles is True:
        print('>>> Moving files to CLIENT directory.\n')
        dest = util.getSysDir(r.dir_MoveTo)
        util.copyFile4System(dir_uFile,   dest)
        util.copyFile4System(dir_uclFile, dest)
        util.copyFile4System(dir_intFile, dest)

    if r.bMakeRelease is True:
        print('>>> Moving files to output directory.\n')
        x = os.path.join(r.dir_ReleaseOutput, r.mutatorName)
        # if 'Redirect' folder doesn't exist, create it
        if not os.path.exists(x):
            os.makedirs(x)
        # copy files
        util.copyFile4System(dir_uFile,   x)
        util.copyFile4System(dir_uclFile, x)

        if r.bMakeRedirect is True:
            y = os.path.join(x, _redirectFolderName)
            if not os.path.exists(y):
                os.makedirs(y)
            # copy files
            util.copyFile4System(dir_uz2file, y)

    # remove the file from system after everything else is done
    if r.bMakeRedirect is True:
        print('\n>>> Moving redirect file to redirect directory.\n')
        util.copyFile4System(dir_uz2file, r.dir_Compile + '/' + _redirectFolderName)
        util.deleteCompileDirFiles(dir_uz2file)

    print('\n')
    # press any key to close
    os.system('pause')


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
dbg.debug_Logs()
# compile!
compileMe()
handle_Files()