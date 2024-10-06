wasmのビルド
```
wasm-pack build --target web
```
npmのビルド
```
wasm-pack build --target bundler
cd pkg
npm link
```
npmの使用
```
cd ../site
npm link hello-wasm
npm install
npm run serve

```
最適化
https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size