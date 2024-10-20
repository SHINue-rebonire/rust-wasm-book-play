### memo
.wasmファイルから.watファイルを生成するのに利用  
`cargo install wasm-tools`  
  
出力  
`wasm-tools print target/wasm32-wasip1/debug/hello-wasi-cli.wasm > output.wat`
```
(module
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32)))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32)))
  (type (;7;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;8;) (func (param i32 i32 i32 i32)))
  (type (;9;) (func (param i32 i32 i32 i32 i32)))
  (type (;10;) (func (param i32 i32 i32 i32 i32) (result i32)))

  以下略
```

※ `cargo component build`した.wasmファイルだと↑は出ない。（抽象化された構造で出力されているっぽい？）  
※ `cargo build --target wasm32-wasip1`で出力した.wasmファイルを.watファイルにした。  