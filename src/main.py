import bpy

file_path = "resources\Sickle.blend"
blender_path = "C:\Krzysiek\Programming\Back-end\Blender\blender.exe"

print("Opening the blender file")
bpy.ops.wm.open_mainfile(filepath=file_path)

print("Getting all objects inside file")
objects = bpy.data.objects

print("Iterate over objects and print")
for obj in objects:
    print(obj.name)
