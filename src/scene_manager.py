import bpy

class Scene_manager:

    def __init__(self, file_path, output_path, file_format):
        bpy.ops.wm.open_mainfile(filepath=file_path)
        bpy.context.scene.render.filepath = output_path
        bpy.context.scene.render.image_settings.file_format=file_format

    def import_fbx(fbx_path):
        bpy.ops.import_scene.fbx(filepath=fbx_path)

    def setup_scene_resolution(width, height):
        bpy.context.scene.render.resolution_x = width
        bpy.context.scene.render.resolution_y = height

    def call_renderer():
        bpy.ops.render.render(animation=False, write_still=True, use_viewport=True)   

