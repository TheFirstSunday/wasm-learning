
## Prerequisite

[Install the ssvmup tool](https://www.secondstate.io/articles/ssvmup/)
and the Serverless Framework.

## Build

```
$ ssvmup build --enable-aot
```

## Test

Create the layer for tensorflow and SSVM binaries.

```
$ cd ../layer
$ source download_dependencies.sh
```

Run the wasm application.

```
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/scf.wasm < test/tiktok_logo.json
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/scf.so < test/tiktok_logo.json

$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/scf.wasm < test/no_logo.json
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/scf.so < test/no_logo.json
```

## Deploy

```
$ cp pkg/scf.so scf/
$ sls deploy
```

Test the Jamstack web app via the website URL created from the above step.