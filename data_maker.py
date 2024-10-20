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

# 获取时间戳并生成文件名

timestamp = get_timestamp_filename()
filename_ref = f"test_{timestamp}_bathroom_reference.exr"
execute([
    r".\build\Debug\pbrt.exe",
    r".\scene\contemporary-bathroom\contemporary-bathroom-test.pbrt",
    "--spp", "5",
    "--my-sppm-photons-per-iter", "1000000",
    "--only-output-luminance",
    "--outfile", filename_ref
])

timestamp = get_timestamp_filename()
filename_mse = f"test_{timestamp}_bathroom_mse.exr"
subprocess.run([
    r".\build\Debug\pbrt.exe",
    r".\scene\contemporary-bathroom\contemporary-bathroom-test.pbrt",
    "--spp", "4",
    "--my-sppm-photons-per-iter", "100000",
    "--my-reference-path", filename_ref,
    "--outfile", filename_mse
])

timestamp = get_timestamp_filename()
filename_ref = f"test_{timestamp}_pbrt-book_reference.exr"
execute([
    r".\build\Debug\pbrt.exe",
    r".\scene\pbrt-book\book-test.pbrt",
    "--spp", "5",
    "--my-sppm-photons-per-iter", "1000000",
    "--only-output-luminance",
    "--outfile", filename_ref
])

timestamp = get_timestamp_filename()
filename_mse = f"test_{timestamp}_pbrt-book_mse.exr"
subprocess.run([
    r".\build\Debug\pbrt.exe",
    r".\scene\pbrt-book\book-test.pbrt",
    "--spp", "4",
    "--my-sppm-photons-per-iter", "100000",
    "--my-reference-path", filename_ref,
    "--outfile", filename_mse
])

timestamp = get_timestamp_filename()
filename_ref = f"test_{timestamp}_bmw-m6_reference.exr"
execute([
    r".\build\Debug\pbrt.exe",
    r"..\pbrt-v4-scenes-master\bmw-m6\bmw-m6-sppm.pbrt",
    "--spp", "5",
    "--my-sppm-photons-per-iter", "1000000",
    "--only-output-luminance",
    "--outfile", filename_ref
])

timestamp = get_timestamp_filename()
filename_mse = f"test_{timestamp}_bmw-m6_mse.exr"
subprocess.run([
    r".\build\Debug\pbrt.exe",
    r"..\pbrt-v4-scenes-master\bmw-m6\bmw-m6-sppm.pbrt",
    "--spp", "4",
    "--my-sppm-photons-per-iter", "100000",
    "--my-reference-path", filename_ref,
    "--outfile", filename_mse
])