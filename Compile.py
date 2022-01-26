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
_lineSeparator      =   '######################################################'
_settingsFile       =   'CompileSettings.ini'
_mincompfile        =   'kfcompile.ini'
_redirectFolderName =   'Redirect'
_bDebug             =   True
_list               =   ['.git','*.md','Docs', 'LICENSE']

#################################################################################
#                                UTILITY
#################################################################################

# contains 'runtime' variables
class runtimeVars():
    # Global
    mutatorName             =   'fallback mutatorName'
    dir_Compile             =   'fallback dir_Compile'
    dir_MoveTo              =   'fallback dir_MoveTo'
    dir_ReleaseOutput       =   'fallback dir_ReleaseOutput'
    dir_Classes             =   'fallback dir_Classes'
    # sections
    EditPackages            =   'fallback EditPackages'
    bICompileOutsideofKF    =   'fallback bICompileOutsideofKF'
    bAltDirectories         =   'fallback bAltDirectories'
    bMoveFiles              =   'fallback bMoveFiles'
    bCreateINT              =   'fallback bCreateINT'
    bMakeRedirect           =   'fallback bMakeRedirect'
    bMakeRelease            =   'fallback bMakeRelease'
    # random package related
    pathSystem              =   'fallback pathSystem'
    pathFileU               =   'fallback pathFileU'
    pathFileUCL             =   'fallback pathFileUCL'
    pathFileUZ2             =   'fallback pathFileUZ2'
    pathFileINT             =   'fallback pathFileINT'
    pathFileGarbage         =   'fallback pathFileGarbage'
    # other
    pathMoveTo              =   'fallback pathMoveTo'


# contains lists, dicts used to populate kfcompile.ini / CompileSettings.ini
class types():
    # CompileSettings.ini
    def_Global              =   {'mutatorName'          :   'TestMut',
                                 'dir_Compile'          :   r'D:\Games\SteamLibrary\steamapps\common\KillingFloor',
                                 'dir_MoveTo'           :   r'D:\Games\KF Dedicated Server',
                                 'dir_ReleaseOutput'    :   r'C:\Users\USER\Desktop\Mutators',
                                 'dir_Classes'          :   r'C:\Users\Shtoyan\Desktop\Projects'}

    def_Mod                 =   {'EditPackages'         :   'TestMutParent,TestMut',
                                 'bICompileOutsideofKF' :   'False',
                                 'bAltDirectories'      :   'False',
                                 'bMoveFiles'           :   'False',
                                 'bCreateINT'           :   'False',
                                 'bMakeRedirect'        :   'False',
                                 'bMakeRelease'         :   'False'}

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
        # remove this garbage, its being created every time
        util.deleteCompileDirFiles(r.pathFileGarbage)

         # remove compfile, we don't need it
        util.deleteCompileDirFiles(_mincompfile)

        if r.bICompileOutsideofKF == 'True':
            # set write permissions, just in case
            os.chmod(os.path.join(r.dir_Compile, r.mutatorName), stat.S_IWRITE)
            self.dir_remove(os.path.join(r.dir_Compile, r.mutatorName))


    # check if compilation was successfull
    # let's just check if package.u is created or not
    def compilationFailed(self):
        dir = self.getSysDir(r.dir_Compile)
        ufile = self.getModFileTypes(dir, 1)
        if not os.path.exists(os.path.join(dir, ufile)):
            return True
        return False

    # get file paths from type
    def getModFileTypes(self, dir, type: int):
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
    
    # remove new created 'classes' folder on alternate dir style
    def dir_remove(self, dir):
        if os.path.exists(dir):
            shutil.rmtree(dir, ignore_errors=True)

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
        print(dest)

    # get / create redirect directory in selected directory
    def get_dirRedirect(self, dir):
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

        config['Global'] =  self.def_Global
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
        def write_string2config(text):
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
    def stopMe(self, a: int):
        e = '>>> TERMINATION WARNING: '
        if a == 0:
            print(e + 'CompileSettings.ini was not found.')
        elif a == 1:
            print(e + 'Global section not found in CompileSettings.ini.')
        elif a == 2:
            print(e + r.mutatorName + ' section not found in CompileSettings.ini')
        elif a == 3:
            print(e + 'UCC.exe was not found in compile directory. Install SDK and retry!')
        elif a == 4:
            print(e + 'Alternative Directory is True, but `sources` folder NOT FOUND!')
        elif a == 5:
            print(e + 'Compilation FAILED!')
        os.system('pause')
        exit()

    # nice message box
    def print_separatorBox(self, msg):
        print('\n' + _lineSeparator + '\n')
        print(msg + '\n')
        print(_lineSeparator + '\n')

    # DEBUG / info
    def debug_Logs(self):
        if bool(_bDebug) is True:
            # type some information
            print(_lineSeparator + '\n')
            print(_settingsFile)
            #  global
            print('mutatorName          : ' + r.mutatorName)
            print('dir_Compile          : ' + r.dir_Compile)
            print('dir_MoveTo           : ' + r.dir_MoveTo)
            print('dir_ReleaseOutput    : ' + r.dir_ReleaseOutput)
            print('dir_Classes          : ' + r.dir_Classes)
            print('\n')
            # sections
            print('EditPackages         : ' + r.EditPackages)
            print('bICompileOutsideofKF : ' + r.bICompileOutsideofKF)
            print('bAltDirectories      : ' + r.bAltDirectories)
            print('bMoveFiles           : ' + r.bMoveFiles)
            print('bCreateINT           : ' + r.bCreateINT)
            print('bMakeRedirect        : ' + r.bMakeRedirect)
            print('bMakeRelease         : ' + r.bMakeRelease)

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
        dbg.stopMe(0)

    config = ConfigParser()
    config.read(dirSettingsIni)
    # get global section and set main vars
    if config.has_section('Global') is False:
        dbg.stopMe(1)

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
        dbg.stopMe(2)

    r.EditPackages          =   config[r.mutatorName]['EditPackages']
    r.bICompileOutsideofKF  =   config[r.mutatorName]['bICompileOutsideofKF']
    r.bAltDirectories       =   config[r.mutatorName]['bAltDirectories']
    r.bMoveFiles            =   config[r.mutatorName]['bMoveFiles']
    r.bCreateINT            =   config[r.mutatorName]['bCreateINT']
    r.bMakeRedirect         =   config[r.mutatorName]['bMakeRedirect']
    r.bMakeRelease          =   config[r.mutatorName]['bMakeRelease']

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
    if r.bICompileOutsideofKF == 'True':
        util.dir_remove(destdir)
        shutil.copytree(srcdir, destdir, copy_function=shutil.copy, ignore=shutil.ignore_patterns(*_list))

    # if we use alternative directory style, we need to do some work
    if r.bAltDirectories == 'True':
        sources = os.path.join(srcdir, 'sources')
        if os.path.exists(sources) is False:
            dbg.stopMe(4)

        classes = os.path.join(destdir, 'Classes')
        shutil.rmtree(classes, ignore_errors=True)
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
        dbg.stopMe(3)

    # start the actual compilation! FINALLY!!!
    subprocess.run([ucc, 'make', 'ini=' + _mincompfile, '-EXPORTCACHE'])

    # we failed here, cleanup and shut down
    if util.compilationFailed() is True:
        util.cleanup()
        dbg.stopMe(5)

    # create INT files
    if r.bCreateINT == 'True':
        dbg.print_separatorBox('Creating INT file!')
        os.system(ucc + ' dumpint ' + r.mutatorName + '.u')

    # create UZ2 files
    if r.bMakeRedirect == 'True':
        dbg.print_separatorBox('Creating UZ2 file!')
        os.system(ucc + ' Compress ' + r.mutatorName + '.u')

    # cleanup!
    util.cleanup()


def handle_Files():
    # announce
    dbg.print_separatorBox('MOVING FILES')

    # get System dir
    sys = util.getSysDir(r.dir_Compile)
    # get file paths
    dir_uFile = util.getModFileTypes(sys, 1)
    dir_uclFile = util.getModFileTypes(sys, 2)
    dir_uz2file = util.getModFileTypes(sys, 3)

    # do we want files being moved to desired client / server directory?
    if r.bMoveFiles == 'True':
        dest = util.getSysDir(r.dir_MoveTo)
        util.copyFile4System(dir_uFile, dest)
        util.copyFile4System(dir_uclFile, dest)
        print('>>> Moving files to CLIENT directory.')

    if r.bMakeRelease == 'True':
        x = os.path.join(r.dir_ReleaseOutput, r.mutatorName)
        # if 'Redirect' folder doesn't exist, create it
        if not os.path.exists(x):
            os.makedirs(x)
        # copy files
        util.copyFile4System(dir_uFile, x)
        util.copyFile4System(dir_uclFile, x)

        if r.bMakeRedirect == 'True':
            y = os.path.join(x, _redirectFolderName)
            if not os.path.exists(y):
                os.makedirs(y)
            # copy files
            util.copyFile4System(dir_uz2file, y)

    # remove the file from system after everything else is done
    if r.bMakeRedirect == 'True':
        util.deleteCompileDirFiles(dir_uz2file)

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