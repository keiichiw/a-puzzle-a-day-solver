/* @ts-self-types="./index.d.ts" */

import * as wasm from "./index_bg.wasm";
import { __wbg_set_wasm } from "./index_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    find_solution
} from "./index_bg.js";
export { wasm as __wasm }
