# EIGEN README

## HANDIG TE WETEN

### DXF

Verschil tussen (Lightweight) LwPolyline en Polyline:

```txtd
For most 2D drafting tasks, LWPolylines are the preferred choice due to their efficiency and performance advantages. However, for 3D modeling and complex 2D shapes, Polylines may be more suitable.
```

### JAGUA-RS

`cd lbf`
`cargo run --release -- -i ../assets/swim.json -c ../assets/config_lbf.json -s ../solutions`

### JAGUAR-RS SERVER

See [README.md](./gui/server/README.md)

### WASM-PACK

Install: `cargo install wasm-pack`  
Build: `wasm-pack build --target web`
`cargo watch -s "wasm-pack build --target web"`
