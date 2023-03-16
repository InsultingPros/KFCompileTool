[`CompileSettings.ini`]: CompileSettings.ini

# KF Compile Tool

[![GitHub all releases](https://img.shields.io/github/downloads/InsultingPros/KFCompileTool/total)](https://github.com/InsultingPros/KFCompileTool/releases)

Killing Floor 1 advanced compilation script. Allows you to save hours of your life, automating many file operations and config editing.

## Features

- Automates compilation process, while allowing you to keep your sources in a directory unrelated to game/server files.
- Supports hierarchical project directory structure, instead of keeping all files in one dump.
- Supports compiling mods that depend on other mods.
- Automatically copies compiled packages into your server's System directory.
- Automatically creates compressed files.
- Automatically generates localization files.
- Automatically generates release directory that contains all files that are necessary for a public release.

## Installation and Setup

Download [files](https://github.com/InsultingPros/KFCompileTool/releases) (put them anywhere, any directory is fine) and edit [`CompileSettings.ini`]:

**Global section**, fill all directories:

- **dir_Compile** - path to root directory where actual compilation will take place: it can be either Killing Floor game client with installed SDK or a dedicated server.

```ini
# for this example we compile in our server
dir_Compile=D:\Games\KF Dedicated Server
```

- **dir_MoveTo** - path to root directory of either Killing Floor game client or dedicated server, where compiled files must be moved. If this isn't needed set **bMoveFiles** to false instead.

```ini
# and move compiled files to our game client
dir_MoveTo=D:\Games\SteamLibrary\steamapps\common\KillingFloor
```

- **dir_ReleaseOutput** - path to directory, where script will prepare all the files necessary for public mod release. If you this isn't needed set **bMakeRelease** to false instead.

```ini
# any easy to access directory
dir_ReleaseOutput=D:\ReleaseMutators
```

- **dir_Classes** - directory with your projects (usually people use same directory as **dir_Compile** for that). For example, if you have two projects *SimpleProject* and *ComplexProject*, it can look something like this:

```ini
<dir_Classes>
├── SimpleProject
│   └── Classes
│       └── MyMutator.uc
└── ComplexProject
    └── Classes
        ├── MainMutator.uc
        ├── BaseWeapon.uc
        ├── CoolWeapon.uc
        ├── LameWeapon.uc
        ├── BaseZed.uc
        ├── UltraClot.uc
        └── MegaCrawler.uc
```

- **mutatorName** - name of the mod to compile by default when the script is called. In the above example it would be either *SimpleProject* or *ComplexProject*.

**Mod sections** allow you to describe how to compile each individual mod (compilation of particular is invoked by `python .\Compile.py <mod_name>` command):

- **EditPackages** - your mod's package name. If you have dependencies add them all at once separated with comma:

```ini
[MyMod]
EditPackages=MyModParent1,MyModParent2,MyMod
```

- **bICompileOutsideofKF** - if your mod folder is outside of **dir_Compile**.

> **Warning** if **bICompileOutsideofKF** is set to true, it will wipe the mod folder before and after compilation step in **dir_Compile**. This is intentional, to keep everything clean and separated.

- **bAltDirectories** - Set this flag to true to use **sources** directory instead of **Classes** to contain your script files. This flag also allows you to use sub-directories to organize your source files:

```ini
ComplexProject
└── source
    ├── MainMutator.uc
    ├── weapons
    │   ├── BaseWeapon.uc
    │   ├── CoolWeapon.uc
    │   └── LameWeapon.uc
    └── enemies
        ├── BaseZed.uc
        ├── UltraClot.uc
        └── MegaCrawler.uc
```

- **bMoveFiles** and bMakeRelease - were described above.
- **bCreateINT** - set this to *true* to automatically create localization files.
- **bMakeRedirect** - set this to *true* to automatically compress compiled files.

If you feel lost after this description - check the [`CompileSettings.ini`] (it's as simple as possible) and try to compile something. Terminal will warn you about errors / missed lines.

## Requirements

- [Python >3.10.x](https://www.python.org/).
- OS - Windows.
