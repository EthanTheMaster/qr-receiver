(function() {
    var wasm;
    const __exports = {};


    let cachegetUint8Memory = null;
    function getUint8Memory() {
        if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory;
    }

    let WASM_VECTOR_LEN = 0;

    function passArray8ToWasm(arg) {
        const ptr = wasm.__wbindgen_malloc(arg.length * 1);
        getUint8Memory().set(arg, ptr / 1);
        WASM_VECTOR_LEN = arg.length;
        return ptr;
    }

    function getArrayU8FromWasm(ptr, len) {
        return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
    }

    let cachedGlobalArgumentPtr = null;
    function globalArgumentPtr() {
        if (cachedGlobalArgumentPtr === null) {
            cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
        }
        return cachedGlobalArgumentPtr;
    }

    let cachegetUint32Memory = null;
    function getUint32Memory() {
        if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
            cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
        }
        return cachegetUint32Memory;
    }

    function freeBlock(ptr) {

        wasm.__wbg_block_free(ptr);
    }
    /**
    */
    class Block {

        free() {
            const ptr = this.ptr;
            this.ptr = 0;
            freeBlock(ptr);
        }

    }
    __exports.Block = Block;

    function freeQrReceiver(ptr) {

        wasm.__wbg_qrreceiver_free(ptr);
    }
    /**
    */
    class QrReceiver {

        static __wrap(ptr) {
            const obj = Object.create(QrReceiver.prototype);
            obj.ptr = ptr;

            return obj;
        }

        free() {
            const ptr = this.ptr;
            this.ptr = 0;
            freeQrReceiver(ptr);
        }

        /**
        * @returns {QrReceiver}
        */
        static new() {
            return QrReceiver.__wrap(wasm.qrreceiver_new());
        }
        /**
        * @param {Uint8Array} arg0
        * @returns {void}
        */
        process(arg0) {
            const ptr0 = passArray8ToWasm(arg0);
            const len0 = WASM_VECTOR_LEN;
            return wasm.qrreceiver_process(this.ptr, ptr0, len0);
        }
        /**
        * @returns {number}
        */
        get_progress_percentage() {
            return wasm.qrreceiver_get_progress_percentage(this.ptr);
        }
        /**
        * @returns {Uint8Array}
        */
        get_finished_data() {
            const retptr = globalArgumentPtr();
            wasm.qrreceiver_get_finished_data(retptr, this.ptr);
            const mem = getUint32Memory();
            const rustptr = mem[retptr / 4];
            const rustlen = mem[retptr / 4 + 1];

            const realRet = getArrayU8FromWasm(rustptr, rustlen).slice();
            wasm.__wbindgen_free(rustptr, rustlen * 1);
            return realRet;

        }
        /**
        * @returns {boolean}
        */
        has_completed_download() {
            return (wasm.qrreceiver_has_completed_download(this.ptr)) !== 0;
        }
        /**
        * @returns {number}
        */
        get_num_pending_blocks() {
            return wasm.qrreceiver_get_num_pending_blocks(this.ptr);
        }
    }
    __exports.QrReceiver = QrReceiver;

    function freeXorShift(ptr) {

        wasm.__wbg_xorshift_free(ptr);
    }
    /**
    */
    class XorShift {

        free() {
            const ptr = this.ptr;
            this.ptr = 0;
            freeXorShift(ptr);
        }

    }
    __exports.XorShift = XorShift;

    let cachedTextDecoder = new TextDecoder('utf-8');

    function getStringFromWasm(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
    }

    __exports.__wbindgen_throw = function(ptr, len) {
        throw new Error(getStringFromWasm(ptr, len));
    };

    function init(path_or_module) {
        let instantiation;
        const imports = { './wasm_qr_receiver': __exports };
        if (path_or_module instanceof WebAssembly.Module) {
            instantiation = WebAssembly.instantiate(path_or_module, imports)
            .then(instance => {
            return { instance, module: path_or_module }
        });
    } else {
        const data = fetch(path_or_module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            instantiation = WebAssembly.instantiateStreaming(data, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return data
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            instantiation = data
            .then(response => response.arrayBuffer())
            .then(buffer => WebAssembly.instantiate(buffer, imports));
        }
    }
    return instantiation.then(({instance}) => {
        wasm = init.wasm = instance.exports;

    });
};
self.wasm_bindgen = Object.assign(init, __exports);
})();
