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

MAX_DEPTH = 7
SPPM_PHOTONS_PER_ITER = 221184 # 1.5 x pixels
OUTPUT_FILE_PATH = "result/"

# tip: how to use reference, e.g. "--my-reference-path", filename_ref,

def generate_data_using_a_scene(scene_name, scene_path, sppm_radius=0.02):

    spp = 512
    timestamp = get_timestamp_filename()
    filename_ref = f"{timestamp}_{scene_name}_bdpt_{spp}.exr"
    execute([
        r".\build\Debug\pbrt.exe",
        scene_path,
        "--my-integrator", "bdpt",
        "--spp", str(spp),
        "--my-max-depth", str(MAX_DEPTH),
        "--outfile", OUTPUT_FILE_PATH + filename_ref,
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
    ])

generate_data_using_a_scene("bathroom", r".\scene\contemporary-bathroom\contemporary-bathroom-common.pbrt", sppm_radius=0.02)
generate_data_using_a_scene("book", r".\scene\pbrt-book\book-common.pbrt", sppm_radius=0.1)
generate_data_using_a_scene("bmw", r"..\pbrt-v4-scenes-master\bmw-m6\bmw-m6-common.pbrt", sppm_radius=0.1)
generate_data_using_a_scene("zeroday-frame120", r"..\pbrt-v4-scenes-master\zero-day\frame120-common.pbrt")
generate_data_using_a_scene("watercolor-camera1", r"..\pbrt-v4-scenes-master\watercolor\camera-1-common.pbrt")
generate_data_using_a_scene("bistro-vespa", r"..\pbrt-v4-scenes-master\bistro\bistro_vespa-common.pbrt")