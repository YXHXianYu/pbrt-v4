# PBRT v4 Customized

## 关于DataMaker

* 用法：切换到 `data-maker` 分支，编译完成后，在项目根目录下执行 `python .\data_maker.py` 即可
* `data_maker.py` 脚本可以方便的批量生成数据，让电脑跑一晚上，~~从而影响舍友睡眠~~
* 这个Customized PBRT提供了几个额外的命令行参数，便于批量生成数据
  * `--my-sppm-photons-per-iter`
    * 可以覆盖场景文件，即 `xxx.pbrt` 中指定的SPPM的单次迭代光子数，便于命令行完全指定参数
  * `--only-output-luminance`
    * 可以只输出RGB + 历史RGB
    * 用于生成referencec，避免输出文件过大。如果能接受超大reference输出（256次迭代 ≈ 500MB），可以不用启用本选项
    * **注意，代码中疑似有bug，在spp256 + 1e7光子情况下，最后输出结果可能会变成黑色。所以我这里附带了历史RGB**
  * `--my-reference-path`
    * 从命令行指定Refenrece Image
* 推荐参数
  * 根目录下的reference使用spp=256, ppi=8e6，跑了大约8小时得到（cpu i5-12400F）
    * 注：此reference生成代码有误，漏了direct light；需要重跑（TODO）
* 关于跑实验的同时玩游戏
  * 经测试，12核的cpu，可以通过 `--nThreads` 参数限定只用4核，这个时候可以同时启动彩虹六号等主要占用gpu的游戏。实验和游戏都不会崩溃
  * 这样就可以跑一天实验的同时玩一天游戏了，最爽的一集（误）
