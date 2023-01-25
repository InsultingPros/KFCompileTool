# KF Compile Tool

[release_badge]: <https://img.shields.io/github/downloads/InsultingPros/KFCompileTool/total?style=for-the-badge>
[build_badge]: https://img.shields.io/github/actions/workflow/status/InsultingPros/KFCompileTool/build.yml?style=for-the-badge

[![build_badge]](https://github.com/InsultingPros/KFCompileTool/actions/workflows/build.yml) [![release_badge]](https://github.com/InsultingPros/KFCompileTool/releases)

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

1. Download the latest [release](https://github.com/InsultingPros/KFCompileTool/releases/latest).
2. Run the executable once to generate the default `kf_compile_tool.ini`.
3. Start editing the config file.

### Global Section

You need to fill all directories:

- **dir_Compile** - path to root directory where actual compilation will take place: it can be either Killing Floor game client with installed SDK or a dedicated server (aka has an `UCC.exe`).

```ini
# for this example we compile in our server
dir_Compile=D:\Games\KF Dedicated Server
```

- **dir_MoveTo** - path to root directory of either Killing Floor game client or dedicated server, where compiled files must be moved. This is purely optional and if this isn't needed - leave mod's **bMoveFiles** to `False`.

```ini
# and move compiled files to our game client
dir_MoveTo=D:\Games\SteamLibrary\steamapps\common\KillingFloor
```

> [!NOTE]
> If you choose your outpur directory where your windows user has no rights to write, `Desktop` for example - you need to create that folder manually.

- **dir_ReleaseOutput** - path to directory, where script will prepare all the files necessary (`.u`, `.ucl`, `.int`, `.uz2`) for public mod release. If you this isn't needed set **bMakeRelease** to `False` instead.

```ini
# any easy to access directory
dir_ReleaseOutput=D:\ReleaseMutators
```

> [!TIP]
> Usually people use same directory as **dir_Compile** for that aka game / server folder. But if you want, you can move your mod sources to any directory.

- **dir_Classes** - directory with your projects. For example, if you have two projects *SimpleProject* and *ComplexProject*, it can look something like this:

```ini
<dir_Classes>
├── MyShinyPreciousMutator
│   └── Classes
│       └── MyMutator.uc
└── YetAnotherPotatoMutator
    └── Classes
        ├── MainMutator.uc
        ├── BaseWeapon.uc
        ├── CoolWeapon.uc
        ├── LameWeapon.uc
        ├── BaseZed.uc
        ├── UltraClot.uc
        └── MegaCrawler.uc
```

- **mutatorName** - name of the mod to compile, when the script is called. In the above example it would be either *MyShinyPreciousMutator* or *YetAnotherPotatoMutator*. Or you can pass the mod name as a commandline argument to script / exe:

```bash
.\kf_compile_tool.exe MyShinyPreciousMutator
```

### Mod Sections

Allows you to describe how to compile each individual mod.

- **EditPackages** - your mod's package name. If you have dependencies add them all at once separated with comma:

```ini
EditPackages=MyModParent1,MyModParent2,MyMod
```

> [!CAUTION]
> If **bICompileOutsideofKF** is set to true, it will wipe the mod folder before and after compilation step in **dir_Compile**. This is intentional, to keep everything clean and separated.

- **bICompileOutsideofKF** - if your mod folder **dir_Classes** doesn't match **dir_Compile** - set to `True`.

- **bAltDirectories** - Set this flag to true to use alternative directory style for your source files. Create **sources** directory instead of **Classes** to contain your script files. And you can also add as many sub-directories as you want, to organize your files:

```txt
ComplexProject
└── sources
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

- **bMoveFiles** and **bMakeRelease** - were described above.
- **bCreateINT** - set this to `True` to automatically create localization files.
- **bMakeRedirect** - set this to `True` to automatically compress compiled files.

If you feel lost after this description - try to compile something. Terminal will warn you about errors / missing lines.

## Requirements

- 5 minutes of your time.
- Windows OS.
