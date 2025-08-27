// ABOUTME: ES6 module that provides a Validator class for loading and executing WASM validators
// ABOUTME: Implements validate() and assert() methods with automatic WASM module caching

class ValidationError extends Error {
    constructor(message, value) {
        super(message);
        this.name = 'ValidationError';
        this.value = value;
    }
}

class Validator {
    constructor(wasmUrl) {
        this.wasmUrl = wasmUrl;
        this.wasmModule = null;
        this.wasmInstance = null;
        this.isLoading = false;
        this.loadPromise = null;
    }

    async ensureLoaded() {
        if (this.wasmInstance) {
            return;
        }

        if (this.isLoading) {
            await this.loadPromise;
            return;
        }

        this.isLoading = true;
        this.loadPromise = this.loadWasm();
        
        try {
            await this.loadPromise;
        } finally {
            this.isLoading = false;
        }
    }

    async loadWasm() {
        try {
            // Fetch the WASM module
            const response = await fetch(this.wasmUrl, { 
                headers: {
                    'Content-Type': 'application/wasm'
                }
            });
            if (!response.ok) {
                throw new Error(`Failed to fetch WASM module: ${response.statusText}`);
            }

            // Compile and instantiate the module
            const wasmBuffer = await response.arrayBuffer();
            this.wasmModule = await WebAssembly.compile(wasmBuffer);
            
            // Create imports object for WASM module
            const imports = this.createImports();
            
            // Instantiate the module
            this.wasmInstance = await WebAssembly.instantiate(this.wasmModule, imports);
            
            // Initialize WASM start function if it exists
            if (this.wasmInstance.exports.__wbindgen_start) {
                this.wasmInstance.exports.__wbindgen_start();
            }
        } catch (error) {
            throw new Error(`Failed to load WASM validator from ${this.wasmUrl}: ${error.message}`);
        }
    }

    createImports() {
        const memory = new WebAssembly.Memory({ initial: 17, maximum: 16384, shared: false });
        const heap = new Array(128).fill(undefined);
        heap.push(undefined, null, true, false);
        let heap_next = heap.length;
        
        const getObject = (idx) => heap[idx];
        
        const dropObject = (idx) => {
            if (idx < 132) return;
            heap[idx] = heap_next;
            heap_next = idx;
        };
        
        const takeObject = (idx) => {
            const ret = getObject(idx);
            dropObject(idx);
            return ret;
        };
        
        const addHeapObject = (obj) => {
            if (heap_next === heap.length) heap.push(heap.length + 1);
            const idx = heap_next;
            heap_next = heap[idx];
            heap[idx] = obj;
            return idx;
        };
        
        let WASM_VECTOR_LEN = 0;
        const cachedTextEncoder = new TextEncoder();
        const encodeString = (arg, view) => {
            return cachedTextEncoder.encodeInto(arg, view);
        };
        
        const passStringToWasm = (arg, malloc, realloc) => {
            if (!this.wasmInstance) return 0;
            
            if (realloc === undefined) {
                const buf = cachedTextEncoder.encode(arg);
                const ptr = malloc(buf.length, 1) >>> 0;
                const mem = new Uint8Array(this.wasmInstance.exports.memory.buffer);
                mem.subarray(ptr, ptr + buf.length).set(buf);
                WASM_VECTOR_LEN = buf.length;
                return ptr;
            }
            
            let len = arg.length;
            let ptr = malloc(len, 1) >>> 0;
            
            const mem = new Uint8Array(this.wasmInstance.exports.memory.buffer);
            
            let offset = 0;
            
            for (; offset < len; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
            
            if (offset !== len) {
                if (offset !== 0) {
                    arg = arg.slice(offset);
                }
                ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
                const view = new Uint8Array(this.wasmInstance.exports.memory.buffer).subarray(ptr + offset, ptr + len);
                const ret = encodeString(arg, view);
                offset += ret.written;
                ptr = realloc(ptr, len, offset, 1) >>> 0;
            }
            
            WASM_VECTOR_LEN = offset;
            return ptr;
        };
        
        const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        
        const getStringFromWasm = (ptr, len) => {
            ptr = ptr >>> 0;
            const mem = new Uint8Array(this.wasmInstance.exports.memory.buffer);
            return cachedTextDecoder.decode(mem.subarray(ptr, ptr + len));
        };
        
        const isLikeNone = (x) => x === undefined || x === null;
        
        const imports = {
            __wbindgen_placeholder__: {
                __wbindgen_string_get: (arg0, arg1) => {
                    const obj = getObject(arg1);
                    const ret = typeof(obj) === 'string' ? obj : undefined;
                    const ptr1 = isLikeNone(ret) ? 0 : passStringToWasm(ret, this.wasmInstance.exports.__wbindgen_malloc, this.wasmInstance.exports.__wbindgen_realloc);
                    const len1 = WASM_VECTOR_LEN;
                    const view = new DataView(this.wasmInstance.exports.memory.buffer);
                    view.setInt32(arg0 + 4 * 1, len1, true);
                    view.setInt32(arg0 + 4 * 0, ptr1, true);
                },
                __wbindgen_string_new: (ptr, len) => {
                    const ret = getStringFromWasm(ptr, len);
                    return addHeapObject(ret);
                },
                __wbindgen_object_drop_ref: (arg) => {
                    takeObject(arg);
                },
                __wbindgen_throw: (ptr, len) => {
                    throw new Error(getStringFromWasm(ptr, len));
                },
                __wbindgen_memory: () => {
                    const ret = this.wasmInstance.exports.memory;
                    return addHeapObject(ret);
                },
                __wbindgen_is_undefined: (arg) => {
                    const ret = getObject(arg) === undefined;
                    return ret;
                },
                __wbindgen_is_null: (arg) => {
                    const ret = getObject(arg) === null;
                    return ret;
                },
                __wbindgen_is_object: (arg) => {
                    const val = getObject(arg);
                    const ret = typeof(val) === 'object' && val !== null;
                    return ret;
                },
                __wbindgen_is_bigint: (arg) => {
                    const ret = typeof(getObject(arg)) === 'bigint';
                    return ret;
                },
                __wbindgen_is_function: (arg) => {
                    const ret = typeof(getObject(arg)) === 'function';
                    return ret;
                },
                __wbindgen_bigint_from_i64: (arg) => {
                    const ret = arg;
                    return addHeapObject(ret);
                },
                __wbindgen_bigint_from_u64: (arg) => {
                    const ret = BigInt.asUintN(64, arg);
                    return addHeapObject(ret);
                },
                __wbindgen_bigint_get_as_i64: (arg, ptr) => {
                    const v = getObject(arg);
                    const ret = typeof(v) === 'bigint' ? v : undefined;
                    const view = new DataView(this.wasmInstance.exports.memory.buffer);
                    view.setBigInt64(ptr + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
                    view.setInt32(ptr + 4 * 0, !isLikeNone(ret), true);
                },
                __wbindgen_jsval_loose_eq: (arg0, arg1) => {
                    const ret = getObject(arg0) == getObject(arg1);
                    return ret;
                },
                __wbindgen_jsval_eq: (arg0, arg1) => {
                    const ret = getObject(arg0) === getObject(arg1);
                    return ret;
                },
                __wbindgen_boolean_get: (arg) => {
                    const v = getObject(arg);
                    const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
                    return ret;
                },
                __wbindgen_number_get: (arg0, arg1) => {
                    const obj = getObject(arg1);
                    const ret = typeof(obj) === 'number' ? obj : undefined;
                    const view = new DataView(this.wasmInstance.exports.memory.buffer);
                    view.setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
                    view.setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
                },
                __wbindgen_error_new: (ptr, len) => {
                    const ret = new Error(getStringFromWasm(ptr, len));
                    return addHeapObject(ret);
                },
                __wbindgen_debug_string: (arg0, arg1) => {
                    const ret = String(getObject(arg1));
                    const ptr1 = passStringToWasm(ret, this.wasmInstance.exports.__wbindgen_malloc, this.wasmInstance.exports.__wbindgen_realloc);
                    const len1 = WASM_VECTOR_LEN;
                    const view = new DataView(this.wasmInstance.exports.memory.buffer);
                    view.setInt32(arg0 + 4 * 1, len1, true);
                    view.setInt32(arg0 + 4 * 0, ptr1, true);
                },
                __wbindgen_in: (arg0, arg1) => {
                    const ret = getObject(arg0) in getObject(arg1);
                    return ret;
                },
                __wbindgen_init_externref_table: () => {
                    const table = this.wasmInstance.exports.__wbindgen_export_2;
                    const offset = table.grow(4);
                    table.set(0, undefined);
                    table.set(offset + 0, undefined);
                    table.set(offset + 1, null);
                    table.set(offset + 2, true);
                    table.set(offset + 3, false);
                },
                __wbindgen_describe: (arg) => {
                    // Used for type descriptions in wasm-bindgen
                    // This is a no-op for runtime
                },
                __wbindgen_describe_closure: (arg) => {
                    // Used for closure descriptions in wasm-bindgen
                    // This is a no-op for runtime
                },
                __wbindgen_json_parse: (ptr, len) => {
                    const str = getStringFromWasm(ptr, len);
                    const ret = JSON.parse(str);
                    return addHeapObject(ret);
                },
                __wbindgen_json_serialize: (arg0, arg1) => {
                    const obj = getObject(arg1);
                    const ret = JSON.stringify(obj === undefined ? null : obj);
                    const ptr1 = passStringToWasm(ret, this.wasmInstance.exports.__wbindgen_malloc, this.wasmInstance.exports.__wbindgen_realloc);
                    const len1 = WASM_VECTOR_LEN;
                    const view = new DataView(this.wasmInstance.exports.memory.buffer);
                    view.setInt32(arg0 + 4 * 1, len1, true);
                    view.setInt32(arg0 + 4 * 0, ptr1, true);
                },
                __wbindgen_object_clone_ref: (arg) => {
                    const ret = getObject(arg);
                    return addHeapObject(ret);
                },
                __wbindgen_cb_drop: (arg) => {
                    const obj = takeObject(arg).original;
                    if (obj.cnt-- === 1) {
                        obj.a = 0;
                        return true;
                    }
                    const ret = false;
                    return ret;
                },
                __wbindgen_closure_wrapper: (arg0, arg1, arg2) => {
                    // Stub for closure wrapper
                    const ret = { a: arg0, b: arg1, cnt: 1, dtor: arg2 };
                    const real = (...args) => {
                        ret.cnt++;
                        const a = ret.a;
                        ret.a = 0;
                        try {
                            return this.wasmInstance.exports[ret.dtor](a, ret.b, ...args);
                        } finally {
                            if (--ret.cnt === 0) {
                                this.wasmInstance.exports.__wbindgen_export_2.get(ret.dtor)(a, ret.b);
                            } else {
                                ret.a = a;
                            }
                        }
                    };
                    real.original = ret;
                    return addHeapObject(real);
                },
                __wbg_getwithrefkey_1dc361bd10053bfe: (arg0, arg1) => {
                    const ret = getObject(arg0)[getObject(arg1)];
                    return addHeapObject(ret);
                },
                __wbg_get_b9b93047fe3cf45b: (arg0, arg1) => {
                    const ret = getObject(arg0)[arg1 >>> 0];
                    return addHeapObject(ret);
                },
                __wbg_get_67b2ba62fc30de12: (arg0, arg1) => {
                    const ret = getObject(arg0)[getObject(arg1)];
                    return addHeapObject(ret);
                },
                __wbg_next_25feadfc0913fea9: (arg) => {
                    const ret = getObject(arg).next;
                    return addHeapObject(ret);
                },
                __wbg_next_6574e1a8a62d1055: (arg) => {
                    try {
                        const ret = getObject(arg).next();
                        return addHeapObject(ret);
                    } catch (e) {
                        return addHeapObject(e);
                    }
                },
                __wbg_done_769e5ede4b31c67b: (arg) => {
                    const ret = getObject(arg).done;
                    return ret;
                },
                __wbg_value_cd1ffa7b1ab794f1: (arg) => {
                    const ret = getObject(arg).value;
                    return addHeapObject(ret);
                },
                __wbg_iterator_9a24c88df860dc65: () => {
                    const ret = Symbol.iterator;
                    return addHeapObject(ret);
                },
                __wbg_call_672a4d21634d4a24: (arg0, arg1) => {
                    try {
                        const ret = getObject(arg0).call(getObject(arg1));
                        return addHeapObject(ret);
                    } catch (e) {
                        return addHeapObject(e);
                    }
                },
                __wbg_entries_3265d4158b33e5dc: (arg) => {
                    const ret = Object.entries(getObject(arg));
                    return addHeapObject(ret);
                },
                __wbg_instanceof_Map_f3469ce2244d2430: (arg) => {
                    let result;
                    try {
                        result = getObject(arg) instanceof Map;
                    } catch (_) {
                        result = false;
                    }
                    const ret = result;
                    return ret;
                },
                __wbg_isArray_a1eab7e0d067391b: (arg) => {
                    const ret = Array.isArray(getObject(arg));
                    return ret;
                },
                __wbg_isSafeInteger_343e2beeeece1bb0: (arg) => {
                    const ret = Number.isSafeInteger(getObject(arg));
                    return ret;
                },
                __wbg_instanceof_Uint8Array_17156bcf118086a9: (arg) => {
                    let result;
                    try {
                        result = getObject(arg) instanceof Uint8Array;
                    } catch (_) {
                        result = false;
                    }
                    const ret = result;
                    return ret;
                },
                __wbg_instanceof_ArrayBuffer_e14585432e3737fc: (arg) => {
                    let result;
                    try {
                        result = getObject(arg) instanceof ArrayBuffer;
                    } catch (_) {
                        result = false;
                    }
                    const ret = result;
                    return ret;
                },
                __wbg_new_a12002a7f91c75be: (arg) => {
                    const ret = new Uint8Array(getObject(arg));
                    return addHeapObject(ret);
                },
                __wbg_set_65595bdd868b3009: (arg0, arg1, arg2) => {
                    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
                },
                __wbg_length_a446193dc22c12f8: (arg) => {
                    const ret = getObject(arg).length;
                    return ret;
                },
                __wbg_length_e2d2a49132c1b256: (arg) => {
                    const ret = getObject(arg).length;
                    return ret;
                },
                __wbg_buffer_609cc3eee51ed158: (arg) => {
                    const ret = getObject(arg).buffer;
                    return addHeapObject(ret);
                }
            },
            __wbindgen_externref_xform__: {
                __wbindgen_externref_table_grow: (delta) => {
                    const table = this.wasmInstance && this.wasmInstance.exports.__wbindgen_export_2;
                    if (table) {
                        const prev = table.length;
                        table.grow(delta);
                        return prev;
                    }
                    return 0;
                },
                __wbindgen_externref_table_set_null: (idx) => {
                    const table = this.wasmInstance && this.wasmInstance.exports.__wbindgen_export_2;
                    if (table) {
                        table.set(idx, null);
                    }
                }
            }
        };
        
        // Store heap for use in validation
        this.heap = heap;
        this.addHeapObject = addHeapObject;
        this.getObject = getObject;
        this.takeObject = takeObject;
        
        return imports;
    }

    async validate(value) {
        await this.ensureLoaded();

        if (!this.wasmInstance.exports.validate) {
            throw new Error('WASM module does not export a validate function');
        }

        try {
            // Create the input object with value property
            const input = { value };
            
            // Add the object to the heap for WASM to access
            const inputIdx = this.addHeapObject(input);
            
            // Call the validate function with the heap index
            const result = this.wasmInstance.exports.validate(inputIdx);
            
            // Convert result to boolean (0 = false, non-zero = true)
            return result !== 0;
        } catch (error) {
            console.error('Validation error:', error);
            return false;
        }
    }

    async assert(value) {
        const isValid = await this.validate(value);
        
        if (!isValid) {
            throw new ValidationError(
                `Validation failed for value: ${JSON.stringify(value)}`,
                value
            );
        }
        
        return true;
    }
}

// Export the Validator class and ValidationError
export { Validator, ValidationError };