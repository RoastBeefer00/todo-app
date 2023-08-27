import * as wasm from "./vite_wasm_functions_bg.wasm";
import { __wbg_set_wasm } from "./vite_wasm_functions_bg.js";
__wbg_set_wasm(wasm);
export * from "./vite_wasm_functions_bg.js";
