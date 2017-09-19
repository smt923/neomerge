# neomerge
A helper tool to merge and manage multiple NEO Scavenger mods with a simple configuration file

## Usage
Drag the executable into NEO Scavenger's install directory, running once will generate a config file (`mods.cfg`) based on the contents of `getmods.php`, running it again will make a backup of `getmods.php`, read the mods from `mods.cfg` and then overwrite `getmods.php` with the newly generated list - I recommend creating your OWN backup of `getmods.php` before running neomerge, see below

The benefits are this config file is much, much easier to read and edit:
```
FieldsofDead = FoD
FieldsZero = 0
SagesPages = SP
M(m)MoD = MoD
MoDZero = 0
```
The order still matters, however, but the order should be preserved by the config generation, so just take care of mod order when adding your own.

Another benefit is that mods can be commented out with a `#` like so:
```
#FieldsofDead = FoD
```
After running neomerge once again, this will automatically exclude any lines that begin with a # from being added to `getmods.php`, thus allowing you to easily temporarily disable mods (As well as nicely seperate groups of mods).

## Backups of getmods.php

It's important to note that while neomerge will create a backup of `getmods.php` each time you run it, this can mean that if ran twice in a row with a bad configuration, you will just have a backup of the broken file, so if a change didn't work take care to restore `getmods.php` back, or create your own 100% working backup named something else so that it'll always be there (this is good practice anyway)

## Using multiple sets of mods

While I make no guarentee that two different pairs of mods will work together, combining multiple mods is easy.
Copy the folders into the game folder like usual, and open the `getmods.php` of the mod(s) you just installed, for example: (If, like this mod, it includes premade compatability versions, you'll still use the correct one for your mod setup)
```
...
&strModName5=ShoulderMod
&strModURL5=Mods/Shouldered/Mod
&strModName6=0
&strModURL6=Mods/Shouldered/Over
&strModName7=MoD
&strModURL7=Mods/Shouldered/Patch_MmMoD
&strModName8=0
&strModURL8=Mods/Shouldered/Over_MmMoD
```
Then, taking the name and the "URL" from the above, you can convert it to the format the configuration uses fairly simply:

```
"Mods/Shouldered/Mod" = ShoulderMod
"Mods/Shouldered/Over = 0
"Mods/Shouldered/Patch_MmMoD" = MoD
"Mods/Shouldered/Over_MmMoD" = 0
```

Simply insert these into your existing `mods.config` (again, I can't help with ordering or compatibility of mods working together, this will have to be done by mod creators):

```
# Mighty Mod of Doom
FieldsofDead = FoD
FieldsZero = 0
SagesPages = SP
M(m)MoD = MoD
MoDZero = 0

# Shouldered (MMoD)
"Mods/Shouldered/Mod" = ShoulderMod
"Mods/Shouldered/Over = 0
"Mods/Shouldered/Patch_MmMoD" = MoD
"Mods/Shouldered/Over_MmMoD" = 0
```