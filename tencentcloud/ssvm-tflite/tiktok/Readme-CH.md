
## 前期准备

[安装 ssvmup 工具](https://www.secondstate.io/articles/ssvmup/)
和 Serverless 框架。

## 创建

```
$ ssvmup build --enable-aot
```

## 测试

Create the layer for tensorflow and SSVM binaries.
为 tensorflow 和 SSVM 二进制文件创建 layer。

```
$ cd ../layer
$ source download_dependencies.sh
```

运行 wasm 应用。

```
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/scf.wasm < test/tiktok_logo.json
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/scf.so < test/tiktok_logo.json

$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/scf.wasm < test/no_logo.json
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/scf.so < test/no_logo.json
```

## 部署

```
$ cp pkg/scf.so scf/
$ sls deploy
```

通过以上步骤创建的网站 URL 测试 Jamstack web 应用。
