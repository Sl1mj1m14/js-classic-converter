# Minecraft Javascript to Classic World Converter
Have you ever wanted to take your minecraft world out of the browser and into the olden days? Well now you can!
Minecraft Classic was the original development cycle of Minecraft, being created back in 2009, and despite originally being playable in the browser, now can only be played as a seperate application and store files locally. Minecraft Classic Remake was made 10 years later, being recoded in javascript, and being playable and storing worlds exclusively within a browser. As such, both of these formats, despite playing nearly identically, have entirely incompatible file formats. This program functions as a converter to convert Minecraft Classic Javascript Remake files to Minecraft Classic files.

## File Format for Classic Javascript
The javascript file format is a pair of json strings stored in a browser's local storage in the following format:

 ```js
savedGame: {"worldSeed":0,"changedBlocks":{},"worldSize":128,"version":1}
settings: {"music":false,"sound":true,"invert":false,"fps":false,"drawDistance":0,"forward":"W","left":"A","backward":"S","right":"D","jump":"<space>","build":"B","chat":"T","fog":"F","saveLoc":"<enter>","loadLoc":"R","username":"name"}
 ```

Note the tilemap is not actually stored in this file format, instead being built dynamically from the seed each time the level is loaded. As such this converter has ported over the entire classic javascript world generation code, so that during conversion the tilemap can be properly generated for the classic save format. Also typically settings would not be considered on a per world basis, however annoyingly in this instance, the username is stored within settings, which translates directly to the `author` field of Classic World Saves, meaning that both json objects must be parsed.

## File Format for Classic

## How to Use

### Read In Modes
