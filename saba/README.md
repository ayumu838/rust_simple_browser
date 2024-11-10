# やったことのメモ

## run_on_wasaba.shがエラーになる

```bash
rustup target add x86_64-unknown-none
```

を実行すると解決する
参考: https://x.com/dd_llll_kk/status/1855056078347943941

## qemuが起動しない

```bash
export DISPLAY=0
```

を run_on_wasaba.sh の実行前にすると解消する

参考: https://github.com/d0iasm/sababook/issues/2#issuecomment-2466528747
