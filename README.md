# KF Compile Tool

[`CompileSettings.ini`]: CompileSettings.ini
[`Compile.py`]: Compile.py

Killing Floor 1 advanced compilation script. Made to replace obsolete [KFCmdlet](https://github.com/InsultingPros/KFCmdlet).

We make use of separate [killingfloor.ini](https://wiki.beyondunreal.com/Legacy:Compiling_With_UCC#Tips) so we don't touch any config of your compile directory. And you can even keep your mods folders in other place, script will auto handle everything and cleanup after execution.

## Installation

- Drag'n'drop the [`Compile.py`] to any desired directory.
- Optionally do the same for `CompileSettings`, but if you forget about it, script will create a new one for you.

## Features and Usage

All settings should be set in [`CompileSettings.ini`].

`Global` section, fill **all** directories:

- `dir_Compile`           - it may be your game client or local dedicated server.
- `dir_MoveTo`            - if you want to automatically move files from client to server, or vice versa.
- `dir_ReleaseOutput`     - if you want to automatically get *.u, *.ucl, *.uz2 files in a separate place.
- `dir_Classes`           - if you decide to keep your mod folder away from `dir_Compile`.
- `mutatorName`           - the mod package name you want to compile. Must have a corresponding section below with same name!

Mods section, you can as many as you want:

- `EditPackages`          - auto adds to EditPackages. Mostly it will be the mod's package name only, but if you have dependencies you can add them all at once separated with comma. Example:

```ini
[TestMod]
EditPackages=TestModParent1,TestModParent2,TestMod
```

- `bICompileOutsideofKF`  - is your mod folder is outside of `dir_Compile`.
- `bAltDirectories`       - is your mod's classes sorted in `source` directory and it's subdirectories: [Example repo](https://insultplayers.ru/git/dkanus/Acedia).
- `bMoveFiles`            - move files from `dir_Compile` to `dir_MoveTo`.
- `bCreateINT`            - creates localization file.
- `bMakeRedirect`         - creates *.uz2 files and automatically puts inside `Redirect` folder.
- `bMakeRelease`          - exports *.u, *.ucl, *.uz2 to `dir_ReleaseOutput`.

If you feel lost after this description - check the [`CompileSettings.ini`] (it's as simple as possible) and try to compile. Terminal will warn you about all possible fixes / missed lines.

## Requirements

- Script was made by [Python 3.10.2](https://docs.python.org/release/3.10.2/).
- KF 1 SDK / UCC.exe. Obviously.
