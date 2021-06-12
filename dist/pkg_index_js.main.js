/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
(self["webpackChunk"] = self["webpackChunk"] || []).push([["pkg_index_js"],{

/***/ "./pkg/index.js":
/*!**********************!*\
  !*** ./pkg/index.js ***!
  \**********************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"__wbg_alert_47c54d3b66d79e8d\": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_alert_47c54d3b66d79e8d),\n/* harmony export */   \"greet\": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.greet),\n/* harmony export */   \"solution\": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.solution)\n/* harmony export */ });\n/* harmony import */ var _index_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./index_bg.js */ \"./pkg/index_bg.js\");\n\n\n\n//# sourceURL=webpack:///./pkg/index.js?");

/***/ }),

/***/ "./pkg/index_bg.js":
/*!*************************!*\
  !*** ./pkg/index_bg.js ***!
  \*************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"greet\": () => (/* binding */ greet),\n/* harmony export */   \"solution\": () => (/* binding */ solution),\n/* harmony export */   \"__wbg_alert_47c54d3b66d79e8d\": () => (/* binding */ __wbg_alert_47c54d3b66d79e8d)\n/* harmony export */ });\n/* harmony import */ var _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./index_bg.wasm */ \"./pkg/index_bg.wasm\");\n/* module decorator */ module = __webpack_require__.hmd(module);\n/* provided dependency */ var TextDecoder = __webpack_require__(/*! text-encoding */ \"./node_modules/text-encoding/index.js\")[\"TextDecoder\"];\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nfunction logError(f, args) {\n    try {\n        return f.apply(this, args);\n    } catch (e) {\n        let error = (function () {\n            try {\n                return e instanceof Error ? `${e.message}\\n\\nStack:\\n${e.stack}` : e.toString();\n            } catch(_) {\n                return \"<failed to stringify thrown value>\";\n            }\n        }());\n        console.error(\"wasm-bindgen: imported JS function that was not marked as `catch` threw an error:\", error);\n        throw e;\n    }\n}\n\nfunction _assertNum(n) {\n    if (typeof(n) !== 'number') throw new Error('expected a number argument');\n}\n/**\n* @param {number} mx\n* @param {number} my\n* @param {number} dx\n* @param {number} dy\n*/\nfunction greet(mx, my, dx, dy) {\n    _assertNum(mx);\n    _assertNum(my);\n    _assertNum(dx);\n    _assertNum(dy);\n    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.greet(mx, my, dx, dy);\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {\n        cachegetInt32Memory0 = new Int32Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);\n    }\n    return cachegetInt32Memory0;\n}\n/**\n* @param {number} mx\n* @param {number} my\n* @param {number} dx\n* @param {number} dy\n* @returns {string}\n*/\nfunction solution(mx, my, dx, dy) {\n    try {\n        const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);\n        _assertNum(mx);\n        _assertNum(my);\n        _assertNum(dx);\n        _assertNum(dy);\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.solution(retptr, mx, my, dx, dy);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(r0, r1);\n    }\n}\n\nfunction __wbg_alert_47c54d3b66d79e8d() { return logError(function (arg0, arg1) {\n    alert(getStringFromWasm0(arg0, arg1));\n}, arguments) };\n\n\n\n//# sourceURL=webpack:///./pkg/index_bg.js?");

/***/ }),

/***/ "./pkg/index_bg.wasm":
/*!***************************!*\
  !*** ./pkg/index_bg.wasm ***!
  \***************************/
/***/ ((module, exports, __webpack_require__) => {

"use strict";
eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.id];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name) exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./index_bg.js */ \"./pkg/index_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"\"]()\n\n//# sourceURL=webpack:///./pkg/index_bg.wasm?");

/***/ })

}]);