/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + ({}[chunkId]||chunkId) + ".js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"./crate/pkg/waterguns_vs_paperplanes_wasm_bg.wasm": function() {
/******/ 			return {
/******/ 				"./waterguns_vs_paperplanes_wasm.js": {
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_new_59cb74e423758ede": function() {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbg_new_59cb74e423758ede"]();
/******/ 					},
/******/ 					"__wbg_stack_558ba5917b466edd": function(p0i32,p1i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbg_stack_558ba5917b466edd"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_error_4bb6c2a97407129a": function(p0i32,p1i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbg_error_4bb6c2a97407129a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_instanceof_Window": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_instanceof_Window"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_property_CSSStyleDeclaration": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_set_property_CSSStyleDeclaration"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__widl_f_set_css_text_CSSStyleDeclaration": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_set_css_text_CSSStyleDeclaration"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_instanceof_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_instanceof_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_draw_image_with_html_canvas_element_CanvasRenderingContext2D": function(p0i32,p1i32,p2f64,p3f64) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_draw_image_with_html_canvas_element_CanvasRenderingContext2D"](p0i32,p1i32,p2f64,p3f64);
/******/ 					},
/******/ 					"__widl_f_draw_image_with_html_image_element_and_dw_and_dh_CanvasRenderingContext2D": function(p0i32,p1i32,p2f64,p3f64,p4f64,p5f64) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_draw_image_with_html_image_element_and_dw_and_dh_CanvasRenderingContext2D"](p0i32,p1i32,p2f64,p3f64,p4f64,p5f64);
/******/ 					},
/******/ 					"__widl_f_begin_path_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_begin_path_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_fill_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_fill_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_stroke_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_stroke_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_stroke_style_CanvasRenderingContext2D": function(p0i32,p1i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_set_stroke_style_CanvasRenderingContext2D"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_set_fill_style_CanvasRenderingContext2D": function(p0i32,p1i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_set_fill_style_CanvasRenderingContext2D"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_close_path_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_close_path_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_ellipse_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3f64,p4f64,p5f64,p6f64,p7f64) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_ellipse_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3f64,p4f64,p5f64,p6f64,p7f64);
/******/ 					},
/******/ 					"__widl_f_rect_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_rect_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__widl_f_clear_rect_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_clear_rect_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__widl_f_fill_rect_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_fill_rect_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__widl_f_fill_text_CanvasRenderingContext2D": function(p0i32,p1i32,p2i32,p3f64,p4f64) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_fill_text_CanvasRenderingContext2D"](p0i32,p1i32,p2i32,p3f64,p4f64);
/******/ 					},
/******/ 					"__widl_f_set_font_CanvasRenderingContext2D": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_set_font_CanvasRenderingContext2D"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_rotate_CanvasRenderingContext2D": function(p0i32,p1f64) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_rotate_CanvasRenderingContext2D"](p0i32,p1f64);
/******/ 					},
/******/ 					"__widl_f_set_transform_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3f64,p4f64,p5f64,p6f64) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_set_transform_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3f64,p4f64,p5f64,p6f64);
/******/ 					},
/******/ 					"__widl_f_translate_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_translate_CanvasRenderingContext2D"](p0i32,p1f64,p2f64);
/******/ 					},
/******/ 					"__widl_f_top_DOMRectReadOnly": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_top_DOMRectReadOnly"](p0i32);
/******/ 					},
/******/ 					"__widl_f_left_DOMRectReadOnly": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_left_DOMRectReadOnly"](p0i32);
/******/ 					},
/******/ 					"__widl_f_create_element_Document": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_create_element_Document"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_body_Document": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_body_Document"](p0i32);
/******/ 					},
/******/ 					"__widl_f_get_bounding_client_rect_Element": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_get_bounding_client_rect_Element"](p0i32);
/******/ 					},
/******/ 					"__widl_instanceof_HTMLCanvasElement": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_instanceof_HTMLCanvasElement"](p0i32);
/******/ 					},
/******/ 					"__widl_f_get_context_HTMLCanvasElement": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_get_context_HTMLCanvasElement"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_width_HTMLCanvasElement": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_width_HTMLCanvasElement"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_width_HTMLCanvasElement": function(p0i32,p1i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_set_width_HTMLCanvasElement"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_height_HTMLCanvasElement": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_height_HTMLCanvasElement"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_height_HTMLCanvasElement": function(p0i32,p1i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_set_height_HTMLCanvasElement"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_style_HTMLElement": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_style_HTMLElement"](p0i32);
/******/ 					},
/******/ 					"__widl_f_new_Image": function() {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_new_Image"]();
/******/ 					},
/******/ 					"__widl_f_set_src_HTMLImageElement": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_set_src_HTMLImageElement"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_append_child_Node": function(p0i32,p1i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_append_child_Node"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_document_Window": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_document_Window"](p0i32);
/******/ 					},
/******/ 					"__widl_f_inner_width_Window": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_inner_width_Window"](p0i32);
/******/ 					},
/******/ 					"__widl_f_inner_height_Window": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__widl_f_inner_height_Window"](p0i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_ccf8cbd1628a0c21": function(p0i32,p1i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbg_newnoargs_ccf8cbd1628a0c21"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_1c71dead4ddfc1a7": function(p0i32,p1i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbg_call_1c71dead4ddfc1a7"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_now_8a0c0bdb99aef95d": function() {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbg_now_8a0c0bdb99aef95d"]();
/******/ 					},
/******/ 					"__wbg_globalThis_e18edfcaa69970d7": function() {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbg_globalThis_e18edfcaa69970d7"]();
/******/ 					},
/******/ 					"__wbg_self_c263ff272c9c2d42": function() {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbg_self_c263ff272c9c2d42"]();
/******/ 					},
/******/ 					"__wbg_window_043622d0c8518027": function() {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbg_window_043622d0c8518027"]();
/******/ 					},
/******/ 					"__wbg_global_7e97ac1b8ea927d0": function() {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbg_global_7e97ac1b8ea927d0"]();
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_number_get": function(p0i32,p1i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbindgen_number_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_rethrow": function(p0i32) {
/******/ 						return installedModules["./crate/pkg/waterguns_vs_paperplanes_wasm.js"].exports["__wbindgen_rethrow"](p0i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["./crate/pkg/waterguns_vs_paperplanes_wasm_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"./crate/pkg/waterguns_vs_paperplanes_wasm_bg.wasm":"bb255508fbec9c1c2b9b"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./src/index.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./src/index.js":
/*!**********************!*\
  !*** ./src/index.js ***!
  \**********************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("__webpack_require__.e(/*! import() */ 0).then(__webpack_require__.t.bind(null, /*! ./game.js */ \"./src/game.js\", 7))\n\n//# sourceURL=webpack:///./src/index.js?");

/***/ })

/******/ });