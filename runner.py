import subprocess
import logging
import time
import sys

# 配置日志
logging.basicConfig(filename='commands.log', level=logging.INFO,
                    format='%(asctime)s - %(message)s')

def run_commands(commands, info=print, error=print):
    for idx, cmd in enumerate(commands):
        info(f"开始执行命令 {idx+1}: {cmd}")
        start_time = time.time()
        
        # 使用 subprocess 执行命令，捕获 stdout 和 stderr
        try:
            # cmd = "\"" + cmd + "\""
            result = subprocess.run(["powershell", "-c", cmd], shell=True, capture_output=True, text=True, check=True)
            end_time = time.time()
            duration = end_time - start_time

            info(f"命令 {idx+1} 执行成功, 耗时: {duration:.2f} 秒")
            info(f"stdout: {result.stdout}")
            info(f"stderr: {result.stderr}")
            info(f"\n\n=====================================\n")

        except subprocess.CalledProcessError as e:
            end_time = time.time()
            duration = end_time - start_time

            error(f"命令 {idx+1} 执行失败, 耗时: {duration:.2f} 秒")
            error(f"stdout: {e.stdout}")
            error(f"stderr: {e.stderr}")
            error(f"\n\n=====================================\n")

if __name__ == "__main__":

    # 定义需要执行的命令列表
    commands_list_1 = [
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\barcelona-pavilion\pavilion-day-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\barcelona-pavilion\pavilion-day-sppm.pbrt",

        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\bistro\bistro_vespa-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\bistro\bistro_vespa-sppm.pbrt",
        
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\bmw-m6\bmw-m6-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\bmw-m6\bmw-m6-sppm.pbrt",

        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\contemporary-bathroom\contemporary-bathroom-sppm.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\contemporary-bathroom\contemporary-bathroom-pt.pbrt",

        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\crown\crown-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\crown\crown-sppm.pbrt",
        
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\killeroos\killeroo-simple-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\killeroos\killeroo-simple-sppm.pbrt",

        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\kroken\camera-1-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\kroken\camera-1-sppm.pbrt",
        
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\pbrt-book\book-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\pbrt-book\book-sppm.pbrt",
        
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\sanmiguel\sanmiguel-courtyard-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\sanmiguel\sanmiguel-courtyard-sppm.pbrt",
        
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\sportscar\sportscar-area-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\sportscar\sportscar-area-sppm.pbrt",
        
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\sportscar\sportscar-sky-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\sportscar\sportscar-sky-sppm.pbrt",
        
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\watercolor\camera-1-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\watercolor\camera-1-sppm.pbrt",
        
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\zero-day\frame120-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\zero-day\frame120-sppm.pbrt",
    ]

    commands_list_2 = [
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\kroken\camera-1-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 64 ..\pbrt-v4-scenes-master\kroken\camera-1-sppm.pbrt",

        r".\build\Debug\pbrt.exe --quiet --spp 128 ..\pbrt-v4-scenes-master\barcelona-pavilion\pavilion-day-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 128 ..\pbrt-v4-scenes-master\barcelona-pavilion\pavilion-day-sppm.pbrt",

        r".\build\Debug\pbrt.exe --quiet --spp 128 ..\pbrt-v4-scenes-master\bistro\bistro_vespa-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 128 ..\pbrt-v4-scenes-master\bistro\bistro_vespa-sppm.pbrt",
        
        r".\build\Debug\pbrt.exe --quiet --spp 128 ..\pbrt-v4-scenes-master\bmw-m6\bmw-m6-pt.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 128 ..\pbrt-v4-scenes-master\bmw-m6\bmw-m6-sppm.pbrt",

        r".\build\Debug\pbrt.exe --quiet --spp 128 ..\pbrt-v4-scenes-master\contemporary-bathroom\contemporary-bathroom-sppm.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 128 ..\pbrt-v4-scenes-master\contemporary-bathroom\contemporary-bathroom-pt.pbrt",
    ]

    commands_list = [
        # r".\build\Debug\pbrt.exe --quiet --spp 128 ..\pbrt-v4-scenes-master\contemporary-bathroom\contemporary-bathroom-sppm.pbrt",
        r".\build\Debug\pbrt.exe --quiet --spp 128 ..\pbrt-v4-scenes-master\contemporary-bathroom\contemporary-bathroom-pt.pbrt",
    ]

    if len(sys.argv) >= 2 and sys.argv[1] == "test":
        commands_list = [
            r".\build\Debug\pbrt.exe --spp 8 ..\pbrt-v4-scenes-master\contemporary-bathroom\contemporary-bathroom-pt.pbrt",
        ]
        info = print
        error = print
    else:
        info = logging.info
        error = logging.error

    # 执行命令
    run_commands(commands_list, info, error)