# PBRT v4 Customized

## 关于DataMaker

* 用法：切换到 `data-maker` 分支，编译完成后，在项目根目录下执行 `python .\data_maker.py` 即可
* `data_maker.py` 脚本可以方便的批量生成数据，让电脑跑一晚上~~，从而影响舍友睡眠~~
* 这个Customized PBRT提供了几个额外的命令行参数，便于批量生成数据
  * `--my-sppm-photons-per-iter`
    * 可以覆盖场景文件，即 `xxx.pbrt` 中指定的SPPM的单次迭代光子数，便于命令行完全指定参数
  * `--only-output-luminance`
    * 可以只输出RGB + 历史RGB
    * 用于生成referencec，避免输出文件过大。如果能接受超大reference输出（256次迭代 ≈ 500MB），可以不用启用本选项
    * **注意，代码中疑似有bug，在spp256 + 1e7光子情况下，最后输出结果可能会变成黑色。所以我这里附带了历史RGB**
  * `--my-reference-path`
    * 从命令行指定Refenrece Image