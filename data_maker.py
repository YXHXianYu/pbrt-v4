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
RESOLUTION = 384 * 384
SPPM_PHOTONS_PER_ITER = int(1.4 * RESOLUTION) # 1.4 x pixels
OUTPUT_FILE_PATH = "result/"
N_THREADS = 0 # 0: no limit

# tip: how to use reference, e.g. "--my-reference-path", filename_ref,

def generate_data_using_a_scene(scene_name, scene_path, sppm_radius=0.02, skip_bdpt=False):

    if not skip_bdpt:
        spp = 512
        # spp = 64
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
        ])

    spp = 4
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
    ])

    spp = 16
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
    ])

    spp = 64
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
    ])

    spp = 4
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
    ])

    spp = 16
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
    ])

    spp = 64
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
    ])

# 跳过这个场景，pt <-> bdpt/sppm 的渲染结果差异过大，是一个pt无法收敛的典型场景
# generate_data_using_a_scene("bathroom", r".\scene\contemporary-bathroom\contemporary-bathroom-common.pbrt", sppm_radius=0.02) 

# 跳过这个场景，这个场景2.7G，读取太慢
# generate_data_using_a_scene("watercolor-camera1", r"..\pbrt-v4-scenes-master\watercolor\camera-1-common.pbrt")

# 跳过这个场景，这个场景1.13G，读取太慢
# generate_data_using_a_scene("bistro-vespa", r"..\pbrt-v4-scenes-master\bistro\bistro_vespa-common.pbrt")

# generate_data_using_a_scene("book", r".\scene\pbrt-book\book-common.pbrt", sppm_radius=0.2)
# generate_data_using_a_scene("bmw", r"..\pbrt-v4-scenes-master\bmw-m6\bmw-m6-common.pbrt", sppm_radius=0.2)
# generate_data_using_a_scene("zeroday-frame120", r"..\pbrt-v4-scenes-master\zero-day\frame120-common.pbrt")

# generate_data_using_a_scene("bathroom2", r"..\benedikt-bitterli-scenes\bathroom2\scene-v4.pbrt")
# generate_data_using_a_scene("bedroom", r"..\benedikt-bitterli-scenes\bedroom\scene-v4.pbrt")
# generate_data_using_a_scene("classroom", r"..\benedikt-bitterli-scenes\classroom\scene-v4.pbrt")
# generate_data_using_a_scene("cornell-box", r"..\benedikt-bitterli-scenes\cornell-box\scene-v4.pbrt")
# generate_data_using_a_scene("dining-room", r"..\benedikt-bitterli-scenes\dining-room\scene-v4.pbrt")
# generate_data_using_a_scene("glass-of-water", r"..\benedikt-bitterli-scenes\glass-of-water\scene-v4.pbrt")
generate_data_using_a_scene("kitchen", r"..\benedikt-bitterli-scenes\kitchen\scene-v4.pbrt")
generate_data_using_a_scene("living-room", r"..\benedikt-bitterli-scenes\living-room\scene-v4.pbrt")
generate_data_using_a_scene("living-room-2", r"..\benedikt-bitterli-scenes\living-room-2\scene-v4.pbrt")
generate_data_using_a_scene("staircase", r"..\benedikt-bitterli-scenes\staircase\scene-v4.pbrt")
generate_data_using_a_scene("staircase2", r"..\benedikt-bitterli-scenes\staircase2\scene-v4.pbrt")
generate_data_using_a_scene("veach-ajar", r"..\benedikt-bitterli-scenes\veach-ajar\scene-v4.pbrt")
generate_data_using_a_scene("veach-bidir", r"..\benedikt-bitterli-scenes\veach-bidir\scene-v4.pbrt")

# 跳过这个场景，这个场景是caustic场景，pt无法正确渲染
# generate_data_using_a_scene("water-caustic", r"..\benedikt-bitterli-scenes\water-caustic\scene-v4.pbrt")