# neomerge
A helper tool to merge and manage multiple NEO Scavenger mods with a simple configuration file

## Usage
Drag the executable into NEO Scavenger's install directory, running once will generate a config file (`mods.cfg`) based on the contents of `getmods.php`, running it again will make a backup of `getmods.php`, read the mods from `mods.cfg` and then overwrite `getmods.php` with the newly generated list - I reccomend creating your OWN backup of `getmods.php` before running neomerge, see below

The benefits are this config file is much, much easier to read and edit:
```
FieldsofDead = FoD
FieldsZero = 0
SagesPages = SP
M(m)MoD = MoD
MoDZero = 0
```
The order still matters however, but order should be preserved by the generation, so just take care of mod order when adding your own.

Another benefit is that mods can be commented out with a `#` like so:
```
#FieldsofDead = FoD
```
After running neomerge once again, this will automatically exlude any lines that begin with a # from being added to `getmods.php`, thus allowing you to easily temporarily disable mods.

## Backups of getmods.php

It's important to note that while neomerge will create a backup of `getmods.php` each time you run it, this can mean that if ran twice in a row with a bad configuration, you will just have a backup of the broken file, so if a change didn't work take care to restore `getmods.php` back, or create your own 100% working backup named sometihng else so that it'll always be there (this is good practice anyway)