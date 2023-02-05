# fbx_reader_rs

Basic reader for .fbx format files. This is meant to be just practical, first attempt at rust programming
After specyfying path to fbx file code should verify if it's valid file and printout it's structure.
It's entirely possible to extend this to also write and edit fbx files, but due to existence of fbx SKD 
it seems redundant at best.

To understand fbx file formatting and inner working I suggest reading:
https://code.blender.org/2013/08/fbx-binary-file-format-specification/
https://web.archive.org/web/20160605023014/https://wiki.blender.org/index.php/User:Mont29/Foundation/FBX_File_Structure
and partially SDK documentation
http://docs.autodesk.com/FBX/2014/ENU/FBX-SDK-Documentation/index.html
