import os
import subprocess
from datetime import datetime

def get_timestamp_filename():
    now = datetime.now()
    return now.strftime("%Y-%m-%d_%H-%M-%S")

def execute(cmd):
    print(f"[DataMaker] 开始执行命令: {cmd}")
    start_time = datetime.now()
    subprocess.run(cmd, shell=True, check=True)
    end_time = datetime.now()
    duration = end_time - start_time
    print(f"[DataMaker] 命令执行成功, 耗时: {duration}")

MAX_DEPTH = 10
RESOLUTION_X = 512
RESOLUTION_Y = 512
SPPM_PHOTONS_PER_ITER = int(1.4 * (RESOLUTION_X * RESOLUTION_Y)) # 1.4 x pixels
OUTPUT_FILE_PATH = "result/"
N_THREADS = 0 # 0: no limit

# tip: how to use reference, e.g. "--my-reference-path", filename_ref,

def __bdpt_command(scene_name, scene_path, spp):
    timestamp = get_timestamp_filename()
    filename_ref = f"{timestamp}_{scene_name}_bdpt_{spp}.exr"
    execute([
        r".\build\Debug\pbrt.exe",
        scene_path,
        "--my-integrator", "bdpt",
        "--spp", str(spp),
        "--my-max-depth", str(MAX_DEPTH),
        "--outfile", OUTPUT_FILE_PATH + filename_ref,
        "--nthreads", str(N_THREADS),
        "--my-resolution-x", str(RESOLUTION_X),
        "--my-resolution-y", str(RESOLUTION_Y),
    ])

def __pt_command(scene_name, scene_path, spp):
    timestamp = get_timestamp_filename()
    filename_pt = f"{timestamp}_{scene_name}_pt_{spp}.exr"
    execute([
        r".\build\Debug\pbrt.exe",
        scene_path,
        "--my-integrator", "path",
        "--spp", str(spp),
        "--my-max-depth", str(MAX_DEPTH),
        "--outfile", OUTPUT_FILE_PATH + filename_pt,
        "--nthreads", str(N_THREADS),
        "--my-resolution-x", str(RESOLUTION_X),
        "--my-resolution-y", str(RESOLUTION_Y),
    ])

def __sppm_command(scene_name, scene_path, spp, sppm_radius):
    timestamp = get_timestamp_filename()
    filename_sppm = f"{timestamp}_{scene_name}_sppm_{spp}.exr"
    execute([
        r".\build\Debug\pbrt.exe",
        scene_path,
        "--my-integrator", "sppm",
        "--spp", str(spp),
        "--my-max-depth", str(MAX_DEPTH),
        "--my-sppm-photons-per-iter", str(SPPM_PHOTONS_PER_ITER),
        "--my-sppm-radius", str(sppm_radius),
        "--outfile", OUTPUT_FILE_PATH + filename_sppm,
        "--sppm-simplify-output",
        "--nthreads", str(N_THREADS),
        "--my-resolution-x", str(RESOLUTION_X),
        "--my-resolution-y", str(RESOLUTION_Y),
    ])

def generate_data_using_a_scene(scene_name, scene_path, sppm_radius=0.02, skip_bdpt=False):
    # bdpt
    # __bdpt_command(scene_name, scene_path, 512)
    # __bdpt_command(scene_name, scene_path, 4)
    # pt
    # __pt_command(scene_name, scene_path, 4)
    # __pt_command(scene_name, scene_path, 64)
    __pt_command(scene_name, scene_path, 2)
    # sppm
    # __sppm_command(scene_name, scene_path, 4, sppm_radius)
    # __sppm_command(scene_name, scene_path, 64, sppm_radius)
    __sppm_command(scene_name, scene_path, 2, sppm_radius)

def generate_preview_using_a_scene(scene_name, scene_path, spp=1):
    __pt_command(scene_name, scene_path, spp)

# generate_sppm_using_a_scene("bathroom", r".\scene\contemporary-bathroom\contemporary-bathroom-common.pbrt", spp=16, sppm_radius=0.02)
# generate_sppm_using_a_scene("bathroom2", r"..\benedikt-bitterli-scenes\bathroom2\scene-v4.pbrt", spp=16, sppm_radius=1.0)

# === data maker ===

# filter=box and sampler=independent

generate_data_using_a_scene("bedroom", r"..\benedikt-bitterli-scenes\bedroom\scene-v4.pbrt")
generate_data_using_a_scene("bedroom-v2", r"..\benedikt-bitterli-scenes\bedroom\scene-v4-2.pbrt")
generate_data_using_a_scene("bedroom-v3", r"..\benedikt-bitterli-scenes\bedroom\scene-v4-3.pbrt")

# generate_data_using_a_scene("pavilion", r"..\pbrt-v4-scenes-master\barcelona-pavilion\pavilion-day.pbrt", sppm_radius=1.0)
# generate_data_using_a_scene("pavilion-v2", r"..\pbrt-v4-scenes-master\barcelona-pavilion\pavilion-day-v2.pbrt", sppm_radius=1.0)
# generate_data_using_a_scene("pavilion-v3", r"..\pbrt-v4-scenes-master\barcelona-pavilion\pavilion-day-v3.pbrt", sppm_radius=1.0)

# generate_data_using_a_scene("book", r".\scene\pbrt-book\book-common.pbrt", sppm_radius=0.2)
# generate_data_using_a_scene("bmw", r"..\pbrt-v4-scenes-master\bmw-m6\bmw-m6-common.pbrt", sppm_radius=0.2)
# generate_data_using_a_scene("zeroday-frame120", r"..\pbrt-v4-scenes-master\zero-day\frame120-common.pbrt")

# generate_data_using_a_scene("bathroom2", r"..\benedikt-bitterli-scenes\bathroom2\scene-v4.pbrt", sppm_radius=0.2)
# generate_data_using_a_scene("cornell-box", r"..\benedikt-bitterli-scenes\cornell-box\scene-v4.pbrt")
# generate_data_using_a_scene("dining-room", r"..\benedikt-bitterli-scenes\dining-room\scene-v4.pbrt")
# generate_data_using_a_scene("glass-of-water", r"..\benedikt-bitterli-scenes\glass-of-water\scene-v4.pbrt")
# generate_data_using_a_scene("kitchen", r"..\benedikt-bitterli-scenes\kitchen\scene-v4.pbrt")
# generate_data_using_a_scene("living-room", r"..\benedikt-bitterli-scenes\living-room\scene-v4.pbrt")
# generate_data_using_a_scene("living-room-2", r"..\benedikt-bitterli-scenes\living-room-2\scene-v4.pbrt")
# generate_data_using_a_scene("staircase", r"..\benedikt-bitterli-scenes\staircase\scene-v4.pbrt")
# generate_data_using_a_scene("staircase2", r"..\benedikt-bitterli-scenes\staircase2\scene-v4.pbrt")
# generate_data_using_a_scene("veach-ajar", r"..\benedikt-bitterli-scenes\veach-ajar\scene-v4.pbrt")
# generate_data_using_a_scene("veach-bidir", r"..\benedikt-bitterli-scenes\veach-bidir\scene-v4.pbrt")

# generate_data_using_a_scene("zeroday-frame25", r"..\pbrt-v4-scenes-master\zero-day\frame25.pbrt")
# generate_data_using_a_scene("zeroday-frame210", r"..\pbrt-v4-scenes-master\zero-day\frame210.pbrt")
# generate_data_using_a_scene("zeroday-frame380", r"..\pbrt-v4-scenes-master\zero-day\frame380.pbrt")

# generate_data_using_a_scene("dining-room-v2", r"..\benedikt-bitterli-scenes\dining-room\scene-v4-2.pbrt")
# generate_data_using_a_scene("dining-room-v3", r"..\benedikt-bitterli-scenes\dining-room\scene-v4-3.pbrt")
# generate_data_using_a_scene("glass-of-water-v2", r"..\benedikt-bitterli-scenes\glass-of-water\scene-v4-2.pbrt")
# generate_data_using_a_scene("glass-of-water-v3", r"..\benedikt-bitterli-scenes\glass-of-water\scene-v4-3.pbrt")
# generate_data_using_a_scene("kitchen-v2", r"..\benedikt-bitterli-scenes\kitchen\scene-v4-2.pbrt")
# generate_data_using_a_scene("kitchen-v3", r"..\benedikt-bitterli-scenes\kitchen\scene-v4-3.pbrt")
# generate_data_using_a_scene("living-room-v2", r"..\benedikt-bitterli-scenes\living-room\scene-v4-2.pbrt")
# generate_data_using_a_scene("living-room-v3", r"..\benedikt-bitterli-scenes\living-room\scene-v4-3.pbrt")
# generate_data_using_a_scene("staircase2-v2", r"..\benedikt-bitterli-scenes\staircase2\scene-v4-2.pbrt")
# generate_data_using_a_scene("staircase2-v3", r"..\benedikt-bitterli-scenes\staircase2\scene-v4-3.pbrt")
# generate_data_using_a_scene("veach-ajar-v2", r"..\benedikt-bitterli-scenes\veach-ajar\scene-v4-2.pbrt")
# generate_data_using_a_scene("veach-ajar-v3", r"..\benedikt-bitterli-scenes\veach-ajar\scene-v4-3.pbrt")

# 场景太黑+加载太慢
# generate_data_using_a_scene("kroken-camera-1", r"..\pbrt-v4-scenes-master\kroken\camera-1.pbrt")
# generate_data_using_a_scene("kroken-camera-2", r"..\pbrt-v4-scenes-master\kroken\camera-2.pbrt")
# generate_data_using_a_scene("kroken-camera-3", r"..\pbrt-v4-scenes-master\kroken\camera-3.pbrt")
# generate_data_using_a_scene("kroken-camera-4", r"..\pbrt-v4-scenes-master\kroken\camera-4.pbrt")

# 跳过这个场景，pt <-> bdpt/sppm 的渲染结果差异过大，是一个pt无法收敛的典型场景
# generate_data_using_a_scene("bathroom", r".\scene\contemporary-bathroom\contemporary-bathroom-common.pbrt", sppm_radius=0.02) 

# 跳过这个场景，这个场景2.7G，读取太慢
# generate_data_using_a_scene("watercolor-camera1", r"..\pbrt-v4-scenes-master\watercolor\camera-1-common.pbrt")

# 跳过这个场景，这个场景1.13G，读取太慢
# generate_data_using_a_scene("bistro-vespa", r"..\pbrt-v4-scenes-master\bistro\bistro_vespa-common.pbrt")

# 跳过这个场景，这个场景是caustic场景，pt无法正确渲染
# generate_data_using_a_scene("water-caustic", r"..\benedikt-bitterli-scenes\water-caustic\scene-v4.pbrt")

# 跳过这个场景，这个场景sppm渲染太暗
# generate_data_using_a_scene("classroom", r"..\benedikt-bitterli-scenes\classroom\scene-v4.pbrt")