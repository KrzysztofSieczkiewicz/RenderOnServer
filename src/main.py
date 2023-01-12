import bpy

from scene_manager import Scene_manager

file_path = "resources\sickle.blend"
output_path = "C:\Krzysiek\Programming\Back-end\RenderOnServer\Rendered\image.jpg"
file_format = "JPEG"
blender_path = "C:\Krzysiek\Programming\Back-end\Blender\blender.exe"

print("Initialize scene_manager")
Scene_manager(file_path, output_path, file_format)

print("Setting scene resolution")
Scene_manager.setup_scene_resolution(720, 500)

print("Call renderer that should save image to %s" %output_path)
Scene_manager.call_renderer()
