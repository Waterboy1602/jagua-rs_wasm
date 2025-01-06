# EIGEN README

## HANDIG TE WETEN

### DXF

Verschil tussen (Lightweight) LwPolyline en Polyline:

```txt
For most 2D drafting tasks, LWPolylines are the preferred choice due to their efficiency and performance advantages. However, for 3D modeling and complex 2D shapes, Polylines may be more suitable.
```

`cargo run -- -i ../assets/swim.json -c ../assets/config_lbf.json -s ../solutions -l debug`
`cargo run --release -- -i ../assets/swim.json -c ../assets/config_lbf.json -s ../solutions`
