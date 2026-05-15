# Musi examples

Every example is a numbered directory with:

- `musi.json`
- `index.ms`

Examples in this folder:

- `01-values`
- `02-functions`
- `03-sequences`
- `04-closures`
- `05-maybe`
- `06-records-variants`
- `07-glfw`
- `08-opengl`
- `09-sdl2`
- `10-sqlite3-interop`
- `11-c-calls-musi` (includes `c-calls-musi.c` and `c-calls-musi.sh`)
- `12-opengl-cube`
- `13-glfw-opengl-loop`
- `14-sdl2-input-loop`
- `15-sqlite3-mini-repo`
- `16-opengl-shader-pipeline`
- `17-c-struct-pointer-interop`

Quick checks:

```sh
cargo run -p music -- check examples/01-values/index.ms
cargo run -p music -- disasm examples/12-opengl-cube/index.ms --level hil
examples/11-c-calls-musi/c-calls-musi.sh
```
