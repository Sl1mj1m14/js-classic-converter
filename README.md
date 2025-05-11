![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
# Minecraft Javascript to Classic World Converter
Have you ever wanted to take your minecraft world out of the browser and into the olden days? Well now you can!
[Minecraft Classic](https://minecraft.wiki/w/Java_Edition_Classic) was the original development cycle of Minecraft, being created back in 2009, and despite originally being playable in the browser, now can only be played as a seperate application and store files locally. [Minecraft Classic Remake](https://classic.minecraft.net) was made 10 years later, being recoded in javascript, and being playable and storing worlds exclusively within a browser. As such, both of these formats, despite playing nearly identically, have entirely incompatible file formats. This program functions as a converter to convert Minecraft Classic Javascript Remake files to Minecraft Classic files. For the alternate program see [Classic to Javascript Converter](https://github.com/Sl1mj1m14/classic-js-converter)

## File Format for Classic Javascript
The javascript file format is a pair of json strings stored in a browser's local storage in the following format:

 ```js
savedGame: {"worldSeed":0,"changedBlocks":{},"worldSize":128,"version":1}
settings: {"music":false,"sound":true,"invert":false,"fps":false,"drawDistance":0,"forward":"W","left":"A","backward":"S","right":"D","jump":"<space>","build":"B","chat":"T","fog":"F","saveLoc":"<enter>","loadLoc":"R","username":"name"}
 ```

Note the tilemap is not actually stored in this file format, instead being built dynamically from the seed each time the level is loaded. As such this converter has ported over the entire classic javascript world generation code, so that during conversion the tilemap can be properly generated for the classic save format. 

Also typically settings would not be considered on a per world basis, however annoyingly in this instance, the `username` field is stored within settings, which translates directly to the `author` field of Classic World Saves, meaning that both json objects must be parsed.

Since the raw save format is just json strings, the actual file storage varies greatly per browser. 

### Firefox
*The Firefox local storage format is the only currently supported local storage format.*

Firefox local storage is stored in individual folders for each website at the following directory:

`C:/Users/user/AppData/Roaming/Mozilla/Firefox/Profiles/########.default-release/storage/default/`

Each website folder is named after the domain, with `+` replacing any characters that cannot be in a file name. There are currently only 2 websites that host [classic.minecraft.net](https://classic.minecraft.net), meaning the only applicable local storage files are as follows:

```
https+++classic.minecraft.net
https+++omniarchive.uk
```

Within each website folder, the only file that really matters in our case and actually stores the data is `ls/data.sqlite`. Within this database, the local storage objects are stored in key value pairs. Each key refers to one of the objects mentioned above, which is further compressed using snappy compression when stored to localStorage. Additionally, the database itself is vacuumed, which is an additional layer of compression that applies on top.

### Google Chrome
*This browser is not currently supported*

### Microsoft Edge
*This browser is not currently supported*

### Internet Explorer
*This browser is not currently supported*

### Generic Method
There is also an easy way to retreive localStorage data from any browser without needing to understand the file format. By running inspect element, and then navigating to the console, javascript commands can be written. By running the below commands, `savedGame` and `settings` data can be retrieved:

```js
localStorage.getItem("savedGame");
localStorage.getItem("settings");
```

The output from these commands can then be manually pasted into a text file, and then formatted into json key value pairs, for example:

**localStorage.json**
```js
{
    "savedGame" : {"worldSeed":20329889277135,"changedBlocks":{},"worldSize":256,"version":1},
    "settings" : {"music":false,"sound":true,"invert":false,"fps":false,"drawDistance":0,"forward":"W","left":"A","backward":"S","right":"D","jump":"<space>","build":"B","chat":"T","fog":"F","saveLoc":"<enter>","loadLoc":"R","username":"noname"}
}
```
## File Format for Classic
All save files in classic are gzip compressed, and saved with either a `.dat` or `.mine` extension. Typically they are saved as `level.dat`. The compression remains the same across all classic versions, but depending on the version the internal file structure differs.

### rd-132211 to 0.0.12a_03
Up through Classic 0.0.12a_03, all that was saved in the file was an array of blocks. The worlds were always 256 x 256 x 64, and the blocks were stored in the order x -> z -> y.

### Classic 0.0.13a-dev to Classic 0.0.13a_03
In the Classic 13a versions, the world size fields, the username, the world name, and a create timestamp were all added to the save format. This is the file format that is currently used by the converter, as it has the appropriate fields. Since javascript worlds do not save a world name or a timestamp, the following default values are used during conversion:

```
name: "--"
createTime: 0
```

### Classic 0.0.14a_08 and later
In all versions from Classic 0.0.14a_08 and onwards a modified version of Java's default serialization is used, and many more fields are gradually added over the course of versions. Serialization is not yet supported by the Classic library, and as such worlds cannot be saved in this format yet.

## How to Use
This program is an executable written in rust. Since it's an executable, it can be run just by double clicking. Launching the program initially will create an `input` folder, an `output` folder, and a `config.toml` file. The read in file should be placed in the input folder, and after successful execution the created file will be placed in the output folder.

The `config.toml` file contains an `input-mode` setting. This value can be set to either `0` (default) or `1`. 

Setting it to 0, the program will attempt to read in an sqlite database. In this case, the file at this path should be placed in the `input` folder:

`C:/Users/user/AppData/Roaming/Mozilla/Firefox/Profiles/########.default-release/storage/default/http+++classic.minecraft.net/ls/data.sqlite`
*Note that `http+++classic.minecraft.net` could also be `http+++omniarchive.uk`*

Setting the `input-mode` to `1` attempts to read in the raw json. A json file should be placed within the `input` folder. (See [Generic Method](#generic-method) for details)

Following successful execution , a `level.dat` file will be created in the `output` folder. This file can then be placed in the base directory of a minecraft instance, and classic can be launched. It is recommended to use Classic 0.0.23a_01 or later since that is the version the remake is built off of.