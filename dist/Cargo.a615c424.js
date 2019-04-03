// modules are defined as an array
// [ module function, map of requires ]
//
// map of requires is short require name -> numeric require
//
// anything defined in a previous bundle is accessed via the
// orig method which is the require for previous bundles

// eslint-disable-next-line no-global-assign
parcelRequire = (function (modules, cache, entry, globalName) {
  // Save the require from previous bundle to this closure if any
  var previousRequire = typeof parcelRequire === 'function' && parcelRequire;
  var nodeRequire = typeof require === 'function' && require;

  function newRequire(name, jumped) {
    if (!cache[name]) {
      if (!modules[name]) {
        // if we cannot find the module within our internal map or
        // cache jump to the current global require ie. the last bundle
        // that was added to the page.
        var currentRequire = typeof parcelRequire === 'function' && parcelRequire;
        if (!jumped && currentRequire) {
          return currentRequire(name, true);
        }

        // If there are other bundles on this page the require from the
        // previous one is saved to 'previousRequire'. Repeat this as
        // many times as there are bundles until the module is found or
        // we exhaust the require chain.
        if (previousRequire) {
          return previousRequire(name, true);
        }

        // Try the node require function if it exists.
        if (nodeRequire && typeof name === 'string') {
          return nodeRequire(name);
        }

        var err = new Error('Cannot find module \'' + name + '\'');
        err.code = 'MODULE_NOT_FOUND';
        throw err;
      }

      localRequire.resolve = resolve;
      localRequire.cache = {};

      var module = cache[name] = new newRequire.Module(name);

      modules[name][0].call(module.exports, localRequire, module, module.exports, this);
    }

    return cache[name].exports;

    function localRequire(x){
      return newRequire(localRequire.resolve(x));
    }

    function resolve(x){
      return modules[name][1][x] || x;
    }
  }

  function Module(moduleName) {
    this.id = moduleName;
    this.bundle = newRequire;
    this.exports = {};
  }

  newRequire.isParcelRequire = true;
  newRequire.Module = Module;
  newRequire.modules = modules;
  newRequire.cache = cache;
  newRequire.parent = previousRequire;
  newRequire.register = function (id, exports) {
    modules[id] = [function (require, module) {
      module.exports = exports;
    }, {}];
  };

  for (var i = 0; i < entry.length; i++) {
    newRequire(entry[i]);
  }

  if (entry.length) {
    // Expose entry point to Node, AMD or browser globals
    // Based on https://github.com/ForbesLindesay/umd/blob/master/template.js
    var mainExports = newRequire(entry[entry.length - 1]);

    // CommonJS
    if (typeof exports === "object" && typeof module !== "undefined") {
      module.exports = mainExports;

    // RequireJS
    } else if (typeof define === "function" && define.amd) {
     define(function () {
       return mainExports;
     });

    // <script>
    } else if (globalName) {
      this[globalName] = mainExports;
    }
  }

  // Override the current require with this new one
  return newRequire;
})({"../.cache/.cargo-web/loader-19867e9930789556f8daa1c21cea3ed0.js":[function(require,module,exports) {
function _typeof(obj) { if (typeof Symbol === "function" && typeof Symbol.iterator === "symbol") { _typeof = function _typeof(obj) { return typeof obj; }; } else { _typeof = function _typeof(obj) { return obj && typeof Symbol === "function" && obj.constructor === Symbol && obj !== Symbol.prototype ? "symbol" : typeof obj; }; } return _typeof(obj); }

module.exports = function (bundle) {
  function __initialize(__wasm_module, __load_asynchronously) {
    return function (module_factory) {
      var instance = module_factory();

      if (__load_asynchronously) {
        return WebAssembly.instantiate(__wasm_module, instance.imports).then(function (wasm_instance) {
          var exports = instance.initialize(wasm_instance);
          console.log("Finished loading Rust wasm module 'pando'");
          return exports;
        }).catch(function (error) {
          console.log("Error loading Rust wasm module 'pando':", error);
          throw error;
        });
      } else {
        var instance = new WebAssembly.Instance(__wasm_module, instance.imports);
        return instance.initialize(wasm_instance);
      }
    }(function () {
      var Module = {};
      Module.STDWEB_PRIVATE = {}; // This is based on code from Emscripten's preamble.js.

      Module.STDWEB_PRIVATE.to_utf8 = function to_utf8(str, addr) {
        for (var i = 0; i < str.length; ++i) {
          // Gotcha: charCodeAt returns a 16-bit word that is a UTF-16 encoded code unit, not a Unicode code point of the character! So decode UTF16->UTF32->UTF8.
          // See http://unicode.org/faq/utf_bom.html#utf16-3
          // For UTF8 byte structure, see http://en.wikipedia.org/wiki/UTF-8#Description and https://www.ietf.org/rfc/rfc2279.txt and https://tools.ietf.org/html/rfc3629
          var u = str.charCodeAt(i); // possibly a lead surrogate

          if (u >= 0xD800 && u <= 0xDFFF) {
            u = 0x10000 + ((u & 0x3FF) << 10) | str.charCodeAt(++i) & 0x3FF;
          }

          if (u <= 0x7F) {
            HEAPU8[addr++] = u;
          } else if (u <= 0x7FF) {
            HEAPU8[addr++] = 0xC0 | u >> 6;
            HEAPU8[addr++] = 0x80 | u & 63;
          } else if (u <= 0xFFFF) {
            HEAPU8[addr++] = 0xE0 | u >> 12;
            HEAPU8[addr++] = 0x80 | u >> 6 & 63;
            HEAPU8[addr++] = 0x80 | u & 63;
          } else if (u <= 0x1FFFFF) {
            HEAPU8[addr++] = 0xF0 | u >> 18;
            HEAPU8[addr++] = 0x80 | u >> 12 & 63;
            HEAPU8[addr++] = 0x80 | u >> 6 & 63;
            HEAPU8[addr++] = 0x80 | u & 63;
          } else if (u <= 0x3FFFFFF) {
            HEAPU8[addr++] = 0xF8 | u >> 24;
            HEAPU8[addr++] = 0x80 | u >> 18 & 63;
            HEAPU8[addr++] = 0x80 | u >> 12 & 63;
            HEAPU8[addr++] = 0x80 | u >> 6 & 63;
            HEAPU8[addr++] = 0x80 | u & 63;
          } else {
            HEAPU8[addr++] = 0xFC | u >> 30;
            HEAPU8[addr++] = 0x80 | u >> 24 & 63;
            HEAPU8[addr++] = 0x80 | u >> 18 & 63;
            HEAPU8[addr++] = 0x80 | u >> 12 & 63;
            HEAPU8[addr++] = 0x80 | u >> 6 & 63;
            HEAPU8[addr++] = 0x80 | u & 63;
          }
        }
      };

      Module.STDWEB_PRIVATE.noop = function () {};

      Module.STDWEB_PRIVATE.to_js = function to_js(address) {
        var kind = HEAPU8[address + 12];

        if (kind === 0) {
          return undefined;
        } else if (kind === 1) {
          return null;
        } else if (kind === 2) {
          return HEAP32[address / 4];
        } else if (kind === 3) {
          return HEAPF64[address / 8];
        } else if (kind === 4) {
          var pointer = HEAPU32[address / 4];
          var length = HEAPU32[(address + 4) / 4];
          return Module.STDWEB_PRIVATE.to_js_string(pointer, length);
        } else if (kind === 5) {
          return false;
        } else if (kind === 6) {
          return true;
        } else if (kind === 7) {
          var pointer = Module.STDWEB_PRIVATE.arena + HEAPU32[address / 4];
          var length = HEAPU32[(address + 4) / 4];
          var _output = [];

          for (var i = 0; i < length; ++i) {
            _output.push(Module.STDWEB_PRIVATE.to_js(pointer + i * 16));
          }

          return _output;
        } else if (kind === 8) {
          var arena = Module.STDWEB_PRIVATE.arena;
          var value_array_pointer = arena + HEAPU32[address / 4];
          var length = HEAPU32[(address + 4) / 4];
          var key_array_pointer = arena + HEAPU32[(address + 8) / 4];
          var _output = {};

          for (var i = 0; i < length; ++i) {
            var key_pointer = HEAPU32[(key_array_pointer + i * 8) / 4];
            var key_length = HEAPU32[(key_array_pointer + 4 + i * 8) / 4];
            var key = Module.STDWEB_PRIVATE.to_js_string(key_pointer, key_length);
            var value = Module.STDWEB_PRIVATE.to_js(value_array_pointer + i * 16);
            _output[key] = value;
          }

          return _output;
        } else if (kind === 9) {
          return Module.STDWEB_PRIVATE.acquire_js_reference(HEAP32[address / 4]);
        } else if (kind === 10 || kind === 12 || kind === 13) {
          var adapter_pointer = HEAPU32[address / 4];
          var pointer = HEAPU32[(address + 4) / 4];
          var deallocator_pointer = HEAPU32[(address + 8) / 4];
          var num_ongoing_calls = 0;
          var drop_queued = false;

          var _output = function output() {
            if (pointer === 0 || drop_queued === true) {
              if (kind === 10) {
                throw new ReferenceError("Already dropped Rust function called!");
              } else if (kind === 12) {
                throw new ReferenceError("Already dropped FnMut function called!");
              } else {
                throw new ReferenceError("Already called or dropped FnOnce function called!");
              }
            }

            var function_pointer = pointer;

            if (kind === 13) {
              _output.drop = Module.STDWEB_PRIVATE.noop;
              pointer = 0;
            }

            if (num_ongoing_calls !== 0) {
              if (kind === 12 || kind === 13) {
                throw new ReferenceError("FnMut function called multiple times concurrently!");
              }
            }

            var args = Module.STDWEB_PRIVATE.alloc(16);
            Module.STDWEB_PRIVATE.serialize_array(args, arguments);

            try {
              num_ongoing_calls += 1;
              Module.STDWEB_PRIVATE.dyncall("vii", adapter_pointer, [function_pointer, args]);
              var result = Module.STDWEB_PRIVATE.tmp;
              Module.STDWEB_PRIVATE.tmp = null;
            } finally {
              num_ongoing_calls -= 1;
            }

            if (drop_queued === true && num_ongoing_calls === 0) {
              _output.drop();
            }

            return result;
          };

          _output.drop = function () {
            if (num_ongoing_calls !== 0) {
              drop_queued = true;
              return;
            }

            _output.drop = Module.STDWEB_PRIVATE.noop;
            var function_pointer = pointer;
            pointer = 0;

            if (function_pointer != 0) {
              Module.STDWEB_PRIVATE.dyncall("vi", deallocator_pointer, [function_pointer]);
            }
          };

          return _output;
        } else if (kind === 14) {
          var pointer = HEAPU32[address / 4];
          var length = HEAPU32[(address + 4) / 4];
          var array_kind = HEAPU32[(address + 8) / 4];
          var pointer_end = pointer + length;

          switch (array_kind) {
            case 0:
              return HEAPU8.subarray(pointer, pointer_end);

            case 1:
              return HEAP8.subarray(pointer, pointer_end);

            case 2:
              return HEAPU16.subarray(pointer, pointer_end);

            case 3:
              return HEAP16.subarray(pointer, pointer_end);

            case 4:
              return HEAPU32.subarray(pointer, pointer_end);

            case 5:
              return HEAP32.subarray(pointer, pointer_end);

            case 6:
              return HEAPF32.subarray(pointer, pointer_end);

            case 7:
              return HEAPF64.subarray(pointer, pointer_end);
          }
        } else if (kind === 15) {
          return Module.STDWEB_PRIVATE.get_raw_value(HEAPU32[address / 4]);
        }
      };

      Module.STDWEB_PRIVATE.serialize_object = function serialize_object(address, value) {
        var keys = Object.keys(value);
        var length = keys.length;
        var key_array_pointer = Module.STDWEB_PRIVATE.alloc(length * 8);
        var value_array_pointer = Module.STDWEB_PRIVATE.alloc(length * 16);
        HEAPU8[address + 12] = 8;
        HEAPU32[address / 4] = value_array_pointer;
        HEAPU32[(address + 4) / 4] = length;
        HEAPU32[(address + 8) / 4] = key_array_pointer;

        for (var i = 0; i < length; ++i) {
          var key = keys[i];
          var key_address = key_array_pointer + i * 8;
          Module.STDWEB_PRIVATE.to_utf8_string(key_address, key);
          Module.STDWEB_PRIVATE.from_js(value_array_pointer + i * 16, value[key]);
        }
      };

      Module.STDWEB_PRIVATE.serialize_array = function serialize_array(address, value) {
        var length = value.length;
        var pointer = Module.STDWEB_PRIVATE.alloc(length * 16);
        HEAPU8[address + 12] = 7;
        HEAPU32[address / 4] = pointer;
        HEAPU32[(address + 4) / 4] = length;

        for (var i = 0; i < length; ++i) {
          Module.STDWEB_PRIVATE.from_js(pointer + i * 16, value[i]);
        }
      }; // New browsers and recent Node


      var cachedEncoder = typeof TextEncoder === "function" ? new TextEncoder("utf-8") // Old Node (before v11)
      : (typeof util === "undefined" ? "undefined" : _typeof(util)) === "object" && util && typeof util.TextEncoder === "function" ? new util.TextEncoder("utf-8") // Old browsers
      : null;

      if (cachedEncoder != null) {
        Module.STDWEB_PRIVATE.to_utf8_string = function to_utf8_string(address, value) {
          var buffer = cachedEncoder.encode(value);
          var length = buffer.length;
          var pointer = 0;

          if (length > 0) {
            pointer = Module.STDWEB_PRIVATE.alloc(length);
            HEAPU8.set(buffer, pointer);
          }

          HEAPU32[address / 4] = pointer;
          HEAPU32[(address + 4) / 4] = length;
        };
      } else {
        Module.STDWEB_PRIVATE.to_utf8_string = function to_utf8_string(address, value) {
          var length = Module.STDWEB_PRIVATE.utf8_len(value);
          var pointer = 0;

          if (length > 0) {
            pointer = Module.STDWEB_PRIVATE.alloc(length);
            Module.STDWEB_PRIVATE.to_utf8(value, pointer);
          }

          HEAPU32[address / 4] = pointer;
          HEAPU32[(address + 4) / 4] = length;
        };
      }

      Module.STDWEB_PRIVATE.from_js = function from_js(address, value) {
        var kind = Object.prototype.toString.call(value);

        if (kind === "[object String]") {
          HEAPU8[address + 12] = 4;
          Module.STDWEB_PRIVATE.to_utf8_string(address, value);
        } else if (kind === "[object Number]") {
          if (value === (value | 0)) {
            HEAPU8[address + 12] = 2;
            HEAP32[address / 4] = value;
          } else {
            HEAPU8[address + 12] = 3;
            HEAPF64[address / 8] = value;
          }
        } else if (value === null) {
          HEAPU8[address + 12] = 1;
        } else if (value === undefined) {
          HEAPU8[address + 12] = 0;
        } else if (value === false) {
          HEAPU8[address + 12] = 5;
        } else if (value === true) {
          HEAPU8[address + 12] = 6;
        } else if (kind === "[object Symbol]") {
          var id = Module.STDWEB_PRIVATE.register_raw_value(value);
          HEAPU8[address + 12] = 15;
          HEAP32[address / 4] = id;
        } else {
          var refid = Module.STDWEB_PRIVATE.acquire_rust_reference(value);
          HEAPU8[address + 12] = 9;
          HEAP32[address / 4] = refid;
        }
      }; // New browsers and recent Node


      var cachedDecoder = typeof TextDecoder === "function" ? new TextDecoder("utf-8") // Old Node (before v11)
      : (typeof util === "undefined" ? "undefined" : _typeof(util)) === "object" && util && typeof util.TextDecoder === "function" ? new util.TextDecoder("utf-8") // Old browsers
      : null;

      if (cachedDecoder != null) {
        Module.STDWEB_PRIVATE.to_js_string = function to_js_string(index, length) {
          return cachedDecoder.decode(HEAPU8.subarray(index, index + length));
        };
      } else {
        // This is ported from Rust's stdlib; it's faster than
        // the string conversion from Emscripten.
        Module.STDWEB_PRIVATE.to_js_string = function to_js_string(index, length) {
          index = index | 0;
          length = length | 0;
          var end = (index | 0) + (length | 0);
          var output = "";

          while (index < end) {
            var x = HEAPU8[index++];

            if (x < 128) {
              output += String.fromCharCode(x);
              continue;
            }

            var init = x & 0x7F >> 2;
            var y = 0;

            if (index < end) {
              y = HEAPU8[index++];
            }

            var ch = init << 6 | y & 63;

            if (x >= 0xE0) {
              var z = 0;

              if (index < end) {
                z = HEAPU8[index++];
              }

              var y_z = (y & 63) << 6 | z & 63;
              ch = init << 12 | y_z;

              if (x >= 0xF0) {
                var w = 0;

                if (index < end) {
                  w = HEAPU8[index++];
                }

                ch = (init & 7) << 18 | (y_z << 6 | w & 63);
                output += String.fromCharCode(0xD7C0 + (ch >> 10));
                ch = 0xDC00 + (ch & 0x3FF);
              }
            }

            output += String.fromCharCode(ch);
            continue;
          }

          return output;
        };
      }

      Module.STDWEB_PRIVATE.id_to_ref_map = {};
      Module.STDWEB_PRIVATE.id_to_refcount_map = {};
      Module.STDWEB_PRIVATE.ref_to_id_map = new WeakMap(); // Not all types can be stored in a WeakMap

      Module.STDWEB_PRIVATE.ref_to_id_map_fallback = new Map();
      Module.STDWEB_PRIVATE.last_refid = 1;
      Module.STDWEB_PRIVATE.id_to_raw_value_map = {};
      Module.STDWEB_PRIVATE.last_raw_value_id = 1;

      Module.STDWEB_PRIVATE.acquire_rust_reference = function (reference) {
        if (reference === undefined || reference === null) {
          return 0;
        }

        var id_to_refcount_map = Module.STDWEB_PRIVATE.id_to_refcount_map;
        var id_to_ref_map = Module.STDWEB_PRIVATE.id_to_ref_map;
        var ref_to_id_map = Module.STDWEB_PRIVATE.ref_to_id_map;
        var ref_to_id_map_fallback = Module.STDWEB_PRIVATE.ref_to_id_map_fallback;
        var refid = ref_to_id_map.get(reference);

        if (refid === undefined) {
          refid = ref_to_id_map_fallback.get(reference);
        }

        if (refid === undefined) {
          refid = Module.STDWEB_PRIVATE.last_refid++;

          try {
            ref_to_id_map.set(reference, refid);
          } catch (e) {
            ref_to_id_map_fallback.set(reference, refid);
          }
        }

        if (refid in id_to_ref_map) {
          id_to_refcount_map[refid]++;
        } else {
          id_to_ref_map[refid] = reference;
          id_to_refcount_map[refid] = 1;
        }

        return refid;
      };

      Module.STDWEB_PRIVATE.acquire_js_reference = function (refid) {
        return Module.STDWEB_PRIVATE.id_to_ref_map[refid];
      };

      Module.STDWEB_PRIVATE.increment_refcount = function (refid) {
        Module.STDWEB_PRIVATE.id_to_refcount_map[refid]++;
      };

      Module.STDWEB_PRIVATE.decrement_refcount = function (refid) {
        var id_to_refcount_map = Module.STDWEB_PRIVATE.id_to_refcount_map;

        if (0 == --id_to_refcount_map[refid]) {
          var id_to_ref_map = Module.STDWEB_PRIVATE.id_to_ref_map;
          var ref_to_id_map_fallback = Module.STDWEB_PRIVATE.ref_to_id_map_fallback;
          var reference = id_to_ref_map[refid];
          delete id_to_ref_map[refid];
          delete id_to_refcount_map[refid];
          ref_to_id_map_fallback.delete(reference);
        }
      };

      Module.STDWEB_PRIVATE.register_raw_value = function (value) {
        var id = Module.STDWEB_PRIVATE.last_raw_value_id++;
        Module.STDWEB_PRIVATE.id_to_raw_value_map[id] = value;
        return id;
      };

      Module.STDWEB_PRIVATE.unregister_raw_value = function (id) {
        delete Module.STDWEB_PRIVATE.id_to_raw_value_map[id];
      };

      Module.STDWEB_PRIVATE.get_raw_value = function (id) {
        return Module.STDWEB_PRIVATE.id_to_raw_value_map[id];
      };

      Module.STDWEB_PRIVATE.alloc = function alloc(size) {
        return Module.web_malloc(size);
      };

      Module.STDWEB_PRIVATE.dyncall = function (signature, ptr, args) {
        return Module.web_table.get(ptr).apply(null, args);
      }; // This is based on code from Emscripten's preamble.js.


      Module.STDWEB_PRIVATE.utf8_len = function utf8_len(str) {
        var len = 0;

        for (var i = 0; i < str.length; ++i) {
          // Gotcha: charCodeAt returns a 16-bit word that is a UTF-16 encoded code unit, not a Unicode code point of the character! So decode UTF16->UTF32->UTF8.
          // See http://unicode.org/faq/utf_bom.html#utf16-3
          var u = str.charCodeAt(i); // possibly a lead surrogate

          if (u >= 0xD800 && u <= 0xDFFF) {
            u = 0x10000 + ((u & 0x3FF) << 10) | str.charCodeAt(++i) & 0x3FF;
          }

          if (u <= 0x7F) {
            ++len;
          } else if (u <= 0x7FF) {
            len += 2;
          } else if (u <= 0xFFFF) {
            len += 3;
          } else if (u <= 0x1FFFFF) {
            len += 4;
          } else if (u <= 0x3FFFFFF) {
            len += 5;
          } else {
            len += 6;
          }
        }

        return len;
      };

      Module.STDWEB_PRIVATE.prepare_any_arg = function (value) {
        var arg = Module.STDWEB_PRIVATE.alloc(16);
        Module.STDWEB_PRIVATE.from_js(arg, value);
        return arg;
      };

      Module.STDWEB_PRIVATE.acquire_tmp = function (dummy) {
        var value = Module.STDWEB_PRIVATE.tmp;
        Module.STDWEB_PRIVATE.tmp = null;
        return value;
      };

      var HEAP8 = null;
      var HEAP16 = null;
      var HEAP32 = null;
      var HEAPU8 = null;
      var HEAPU16 = null;
      var HEAPU32 = null;
      var HEAPF32 = null;
      var HEAPF64 = null;
      Object.defineProperty(Module, 'exports', {
        value: {}
      });

      function __web_on_grow() {
        var buffer = Module.instance.exports.memory.buffer;
        HEAP8 = new Int8Array(buffer);
        HEAP16 = new Int16Array(buffer);
        HEAP32 = new Int32Array(buffer);
        HEAPU8 = new Uint8Array(buffer);
        HEAPU16 = new Uint16Array(buffer);
        HEAPU32 = new Uint32Array(buffer);
        HEAPF32 = new Float32Array(buffer);
        HEAPF64 = new Float64Array(buffer);
      }

      return {
        imports: {
          env: {
            "__cargo_web_snippet_1639bcfa7392f2a90b432593a0b026dab94ec9c7": function __cargo_web_snippet_1639bcfa7392f2a90b432593a0b026dab94ec9c7($0, $1) {
              $1 = Module.STDWEB_PRIVATE.to_js($1);
              Module.STDWEB_PRIVATE.from_js($0, function () {
                return {
                  success: false,
                  reason: $1
                };
              }());
            },
            "__cargo_web_snippet_4fd31c9e56d40b8642cf9e6f96fd6b570f355cea": function __cargo_web_snippet_4fd31c9e56d40b8642cf9e6f96fd6b570f355cea($0) {
              $0 = Module.STDWEB_PRIVATE.to_js($0);
              console.error($0);
            },
            "__cargo_web_snippet_59c9efb7acb3bed4a30c1e11ef7d595eb6da96f0": function __cargo_web_snippet_59c9efb7acb3bed4a30c1e11ef7d595eb6da96f0($0, $1) {
              $1 = Module.STDWEB_PRIVATE.to_js($1);
              Module.STDWEB_PRIVATE.from_js($0, function () {
                return {
                  success: true,
                  dotCode: $1
                };
              }());
            },
            "__cargo_web_snippet_80d6d56760c65e49b7be8b6b01c1ea861b046bf0": function __cargo_web_snippet_80d6d56760c65e49b7be8b6b01c1ea861b046bf0($0) {
              Module.STDWEB_PRIVATE.decrement_refcount($0);
            },
            "__cargo_web_snippet_e9638d6405ab65f78daf4a5af9c9de14ecf1e2ec": function __cargo_web_snippet_e9638d6405ab65f78daf4a5af9c9de14ecf1e2ec($0) {
              $0 = Module.STDWEB_PRIVATE.to_js($0);
              Module.STDWEB_PRIVATE.unregister_raw_value($0);
            },
            "__cargo_web_snippet_ff5103e6cc179d13b4c7a785bdce2708fd559fc0": function __cargo_web_snippet_ff5103e6cc179d13b4c7a785bdce2708fd559fc0($0) {
              Module.STDWEB_PRIVATE.tmp = Module.STDWEB_PRIVATE.to_js($0);
            },
            "__web_on_grow": __web_on_grow
          }
        },
        initialize: function initialize(instance) {
          Object.defineProperty(Module, 'instance', {
            value: instance
          });
          Object.defineProperty(Module, 'web_malloc', {
            value: Module.instance.exports.__web_malloc
          });
          Object.defineProperty(Module, 'web_free', {
            value: Module.instance.exports.__web_free
          });
          Object.defineProperty(Module, 'web_table', {
            value: Module.instance.exports.__indirect_function_table
          });

          Module.exports.compile = function compile(pando_code) {
            return Module.STDWEB_PRIVATE.acquire_tmp(Module.instance.exports.compile(Module.STDWEB_PRIVATE.prepare_any_arg(pando_code)));
          };

          Module.exports.newTask = function newTask(task_identifier, pando_code) {
            return Module.STDWEB_PRIVATE.acquire_tmp(Module.instance.exports.newTask(Module.STDWEB_PRIVATE.prepare_any_arg(task_identifier), Module.STDWEB_PRIVATE.prepare_any_arg(pando_code)));
          };

          Module.exports.deleteTask = function deleteTask(task_identifier, pando_code) {
            return Module.STDWEB_PRIVATE.acquire_tmp(Module.instance.exports.deleteTask(Module.STDWEB_PRIVATE.prepare_any_arg(task_identifier), Module.STDWEB_PRIVATE.prepare_any_arg(pando_code)));
          };

          Module.exports.toggleDependency = function toggleDependency(task_identifier, dependency_identifier, pando_code) {
            return Module.STDWEB_PRIVATE.acquire_tmp(Module.instance.exports.toggleDependency(Module.STDWEB_PRIVATE.prepare_any_arg(task_identifier), Module.STDWEB_PRIVATE.prepare_any_arg(dependency_identifier), Module.STDWEB_PRIVATE.prepare_any_arg(pando_code)));
          };

          Module.exports.progressTask = function progressTask(task_identifier, pando_code) {
            return Module.STDWEB_PRIVATE.acquire_tmp(Module.instance.exports.progressTask(Module.STDWEB_PRIVATE.prepare_any_arg(task_identifier), Module.STDWEB_PRIVATE.prepare_any_arg(pando_code)));
          };

          __web_on_grow();

          return Module.exports;
        }
      };
    });
  }

  return fetch(bundle).then(function (response) {
    return response.arrayBuffer();
  }).then(function (bytes) {
    return WebAssembly.compile(bytes);
  }).then(function (mod) {
    return __initialize(mod, true);
  });
};
},{}],"../../../../Users/keith/AppData/Roaming/npm/node_modules/parcel-bundler/src/builtins/hmr-runtime.js":[function(require,module,exports) {
var global = arguments[3];
var OVERLAY_ID = '__parcel__error__overlay__';
var OldModule = module.bundle.Module;

function Module(moduleName) {
  OldModule.call(this, moduleName);
  this.hot = {
    data: module.bundle.hotData,
    _acceptCallbacks: [],
    _disposeCallbacks: [],
    accept: function (fn) {
      this._acceptCallbacks.push(fn || function () {});
    },
    dispose: function (fn) {
      this._disposeCallbacks.push(fn);
    }
  };
  module.bundle.hotData = null;
}

module.bundle.Module = Module;
var parent = module.bundle.parent;

if ((!parent || !parent.isParcelRequire) && typeof WebSocket !== 'undefined') {
  var hostname = "" || location.hostname;
  var protocol = location.protocol === 'https:' ? 'wss' : 'ws';
  var ws = new WebSocket(protocol + '://' + hostname + ':' + "52002" + '/');

  ws.onmessage = function (event) {
    var data = JSON.parse(event.data);

    if (data.type === 'update') {
      console.clear();
      data.assets.forEach(function (asset) {
        hmrApply(global.parcelRequire, asset);
      });
      data.assets.forEach(function (asset) {
        if (!asset.isNew) {
          hmrAccept(global.parcelRequire, asset.id);
        }
      });
    }

    if (data.type === 'reload') {
      ws.close();

      ws.onclose = function () {
        location.reload();
      };
    }

    if (data.type === 'error-resolved') {
      console.log('[parcel] ✨ Error resolved');
      removeErrorOverlay();
    }

    if (data.type === 'error') {
      console.error('[parcel] 🚨  ' + data.error.message + '\n' + data.error.stack);
      removeErrorOverlay();
      var overlay = createErrorOverlay(data);
      document.body.appendChild(overlay);
    }
  };
}

function removeErrorOverlay() {
  var overlay = document.getElementById(OVERLAY_ID);

  if (overlay) {
    overlay.remove();
  }
}

function createErrorOverlay(data) {
  var overlay = document.createElement('div');
  overlay.id = OVERLAY_ID; // html encode message and stack trace

  var message = document.createElement('div');
  var stackTrace = document.createElement('pre');
  message.innerText = data.error.message;
  stackTrace.innerText = data.error.stack;
  overlay.innerHTML = '<div style="background: black; font-size: 16px; color: white; position: fixed; height: 100%; width: 100%; top: 0px; left: 0px; padding: 30px; opacity: 0.85; font-family: Menlo, Consolas, monospace; z-index: 9999;">' + '<span style="background: red; padding: 2px 4px; border-radius: 2px;">ERROR</span>' + '<span style="top: 2px; margin-left: 5px; position: relative;">🚨</span>' + '<div style="font-size: 18px; font-weight: bold; margin-top: 20px;">' + message.innerHTML + '</div>' + '<pre>' + stackTrace.innerHTML + '</pre>' + '</div>';
  return overlay;
}

function getParents(bundle, id) {
  var modules = bundle.modules;

  if (!modules) {
    return [];
  }

  var parents = [];
  var k, d, dep;

  for (k in modules) {
    for (d in modules[k][1]) {
      dep = modules[k][1][d];

      if (dep === id || Array.isArray(dep) && dep[dep.length - 1] === id) {
        parents.push(k);
      }
    }
  }

  if (bundle.parent) {
    parents = parents.concat(getParents(bundle.parent, id));
  }

  return parents;
}

function hmrApply(bundle, asset) {
  var modules = bundle.modules;

  if (!modules) {
    return;
  }

  if (modules[asset.id] || !bundle.parent) {
    var fn = new Function('require', 'module', 'exports', asset.generated.js);
    asset.isNew = !modules[asset.id];
    modules[asset.id] = [fn, asset.deps];
  } else if (bundle.parent) {
    hmrApply(bundle.parent, asset);
  }
}

function hmrAccept(bundle, id) {
  var modules = bundle.modules;

  if (!modules) {
    return;
  }

  if (!modules[id] && bundle.parent) {
    return hmrAccept(bundle.parent, id);
  }

  var cached = bundle.cache[id];
  bundle.hotData = {};

  if (cached) {
    cached.hot.data = bundle.hotData;
  }

  if (cached && cached.hot && cached.hot._disposeCallbacks.length) {
    cached.hot._disposeCallbacks.forEach(function (cb) {
      cb(bundle.hotData);
    });
  }

  delete bundle.cache[id];
  bundle(id);
  cached = bundle.cache[id];

  if (cached && cached.hot && cached.hot._acceptCallbacks.length) {
    cached.hot._acceptCallbacks.forEach(function (cb) {
      cb();
    });

    return true;
  }

  return getParents(global.parcelRequire, id).some(function (id) {
    return hmrAccept(global.parcelRequire, id);
  });
}
},{}]},{},["../../../../Users/keith/AppData/Roaming/npm/node_modules/parcel-bundler/src/builtins/hmr-runtime.js"], null)