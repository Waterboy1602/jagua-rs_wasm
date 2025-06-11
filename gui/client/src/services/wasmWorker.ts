import { Status } from "../Enums";

import init, * as wasm from "../../wasm/pkg/wasm_jagua_rs";

let wasmInitialized = false;

self.onmessage = async (event) => {
  if (!wasmInitialized) {
    try {
      await init();
      wasmInitialized = true;
    } catch (e) {
      self.postMessage({ type: Status.ERROR, message: "Wasm initialization failed: " + e });
      return;
    }
  }

  const { type, payload } = event.data;

  if (type === Status.START) {
    self.postMessage({ type: Status.PROCESSING, message: `Wasm computation started` });
    const svgInput = payload.svgInput;
    try {
      await wasm.make_jaguars_instance(svgInput);
    } catch (e) {
      self.postMessage({ type: Status.ERROR, message: `Wasm computation failed: ` + e });
    }
  }
};
