!function(n){var t={};function e(r){if(t[r])return t[r].exports;var o=t[r]={i:r,l:!1,exports:{}};return n[r].call(o.exports,o,o.exports,e),o.l=!0,o.exports}e.m=n,e.c=t,e.d=function(n,t,r){e.o(n,t)||Object.defineProperty(n,t,{enumerable:!0,get:r})},e.r=function(n){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(n,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(n,"__esModule",{value:!0})},e.t=function(n,t){if(1&t&&(n=e(n)),8&t)return n;if(4&t&&"object"==typeof n&&n&&n.__esModule)return n;var r=Object.create(null);if(e.r(r),Object.defineProperty(r,"default",{enumerable:!0,value:n}),2&t&&"string"!=typeof n)for(var o in n)e.d(r,o,function(t){return n[t]}.bind(null,o));return r},e.n=function(n){var t=n&&n.__esModule?function(){return n.default}:function(){return n};return e.d(t,"a",t),t},e.o=function(n,t){return Object.prototype.hasOwnProperty.call(n,t)},e.p="",e(e.s=2)}([function(n,t,e){"use strict";e.r(t),function(n){function r(n){const t=e.p;let r="";return(!t||t.indexOf("://")<0)&&(r+=window.location.protocol+"//"+window.location.host),r+=t||"/",r+n}let o;e.d(t,"init_codemirror_pass",(function(){return m})),e.d(t,"run_script",(function(){return j})),e.d(t,"compile_script",(function(){return k})),e.d(t,"main_js",(function(){return O})),e.d(t,"Playground",(function(){return A})),e.d(t,"RhaiMode",(function(){return M})),e.d(t,"State",(function(){return P}));const i=new Array(32).fill(void 0);function c(n){return i[n]}i.push(void 0,null,!0,!1);let _=i.length;function a(n){const t=c(n);return function(n){n<36||(i[n]=_,_=n)}(n),t}let s=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});s.decode();let u=null;function f(){return null!==u&&u.buffer===o.memory.buffer||(u=new Uint8Array(o.memory.buffer)),u}function l(n,t){return s.decode(f().subarray(n,n+t))}function d(n){_===i.length&&i.push(i.length+1);const t=_;return _=i[t],i[t]=n,t}let b=0,w=new TextEncoder("utf-8");const g="function"==typeof w.encodeInto?function(n,t){return w.encodeInto(n,t)}:function(n,t){const e=w.encode(n);return t.set(e),{read:n.length,written:e.length}};function p(n,t,e){if(void 0===e){const e=w.encode(n),r=t(e.length);return f().subarray(r,r+e.length).set(e),b=e.length,r}let r=n.length,o=t(r);const i=f();let c=0;for(;c<r;c++){const t=n.charCodeAt(c);if(t>127)break;i[o+c]=t}if(c!==r){0!==c&&(n=n.slice(c)),o=e(o,r,r=c+3*n.length);const t=f().subarray(o+c,o+r);c+=g(n,t).written}return b=c,o}let y=null;function h(){return null!==y&&y.buffer===o.memory.buffer||(y=new Int32Array(o.memory.buffer)),y}function m(n){o.init_codemirror_pass(d(n))}function v(n,t){if(!(n instanceof t))throw new Error("expected instance of "+t.name);return n.ptr}function S(n){return null==n}function j(n,t,e,r){try{const s=o.__wbindgen_add_to_stack_pointer(-16);var i=p(n,o.__wbindgen_malloc,o.__wbindgen_realloc),c=b;o.run_script(s,i,c,d(t),d(e),S(r)?0:d(r));var _=h()[s/4+0],a=h()[s/4+1];return l(_,a)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(_,a)}}function k(n){try{const c=o.__wbindgen_add_to_stack_pointer(-16);var t=p(n,o.__wbindgen_malloc,o.__wbindgen_realloc),e=b;o.compile_script(c,t,e);var r=h()[c/4+0],i=h()[c/4+1];return l(r,i)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(r,i)}}function O(){o.main_js()}function x(n){return function(){try{return n.apply(this,arguments)}catch(n){o.__wbindgen_exn_store(d(n))}}}class A{static __wrap(n){const t=Object.create(A.prototype);return t.ptr=n,t}__destroy_into_raw(){const n=this.ptr;return this.ptr=0,n}free(){const n=this.__destroy_into_raw();o.__wbg_playground_free(n)}constructor(){var n=o.playground_new();return A.__wrap(n)}runScript(n,t,e,r){try{const s=o.__wbindgen_add_to_stack_pointer(-16);var i=p(n,o.__wbindgen_malloc,o.__wbindgen_realloc),c=b;o.playground_runScript(s,this.ptr,i,c,d(t),d(e),S(r)?0:d(r));var _=h()[s/4+0],a=h()[s/4+1];return l(_,a)}finally{o.__wbindgen_add_to_stack_pointer(16),o.__wbindgen_free(_,a)}}}class M{static __wrap(n){const t=Object.create(M.prototype);return t.ptr=n,t}__destroy_into_raw(){const n=this.ptr;return this.ptr=0,n}free(){const n=this.__destroy_into_raw();o.__wbg_rhaimode_free(n)}constructor(n){var t=o.rhaimode_new(n);return M.__wrap(t)}startState(){var n=o.rhaimode_startState(this.ptr);return P.__wrap(n)}copyState(n){v(n,P);var t=o.rhaimode_copyState(this.ptr,n.ptr);return P.__wrap(t)}token(n,t){try{const i=o.__wbindgen_add_to_stack_pointer(-16);v(t,P),o.rhaimode_token(i,this.ptr,d(n),t.ptr);var e=h()[i/4+0],r=h()[i/4+1];let c;return 0!==e&&(c=l(e,r).slice(),o.__wbindgen_free(e,1*r)),c}finally{o.__wbindgen_add_to_stack_pointer(16)}}indent(n,t){v(n,P);var e=p(t,o.__wbindgen_malloc,o.__wbindgen_realloc),r=b;return a(o.rhaimode_indent(this.ptr,n.ptr,e,r))}get electricInput(){return a(o.rhaimode_electricInput(this.ptr))}get lineComment(){return a(o.rhaimode_lineComment(this.ptr))}}class P{static __wrap(n){const t=Object.create(P.prototype);return t.ptr=n,t}__destroy_into_raw(){const n=this.ptr;return this.ptr=0,n}free(){const n=this.__destroy_into_raw();o.__wbg_state_free(n)}}t.default=async function t(e){void 0===e&&(e=new URL("index_bg.wasm",r("pkg/index.js")));const i={wbg:{}};i.wbg.__wbindgen_object_drop_ref=function(n){a(n)},i.wbg.__wbindgen_string_new=function(n,t){return d(l(n,t))},i.wbg.__wbg_call_b5c358098632d7b2=x((function(n,t,e){return d(c(n).call(c(t),c(e)))})),i.wbg.__wbg_next_0864114990d58dd8=function(n){return d(c(n).next())},i.wbg.__wbindgen_is_falsy=function(n){return!c(n)},i.wbg.__wbg_length_7c702a63636a6eb9=function(n){return c(n).length},i.wbg.__wbg_charCodeAt_54f912cd103fce15=function(n,t){return c(n).charCodeAt(t>>>0)},i.wbg.__wbg_backUp_4486cd9ee6fc45bd=function(n,t){c(n).backUp(t>>>0)},i.wbg.__wbg_peek_815da24154859651=function(n){return d(c(n).peek())},i.wbg.__wbindgen_number_new=function(n){return d(n)},i.wbg.__wbg_sol_10af73622bf7e8ee=function(n){return c(n).sol()},i.wbg.__wbg_indentation_bdc928ee5ce4dfa9=function(n){return c(n).indentation()},i.wbg.__wbg_log_8485ead621ceded9=function(n){console.log(c(n))},i.wbg.__wbindgen_object_clone_ref=function(n){return d(c(n))},i.wbg.__wbg_new_6081a2f15edcc59c=function(n,t,e,r){return d(new RegExp(l(n,t),l(e,r)))},i.wbg.__wbindgen_json_parse=function(n,t){return d(JSON.parse(l(n,t)))},i.wbg.__wbg_get_a96a2f48856bb1c3=x((function(n,t){return d(Reflect.get(c(n),c(t)))})),i.wbg.__wbg_now_9f22124bc74da886=function(n){return c(n).now()},i.wbg.__wbg_self_eeabd9085c04fc17=x((function(){return d(self.self)})),i.wbg.__wbg_window_f110c13310da2c8f=x((function(){return d(window.window)})),i.wbg.__wbg_globalThis_a2669bee93faee43=x((function(){return d(globalThis.globalThis)})),i.wbg.__wbg_global_a5584d717f4d6761=x((function(){return d(n.global)})),i.wbg.__wbindgen_is_undefined=function(n){return void 0===c(n)},i.wbg.__wbg_newnoargs_179d393e4626fcf7=function(n,t){return d(new Function(l(n,t)))},i.wbg.__wbg_call_8487a9f580e47219=x((function(n,t){return d(c(n).call(c(t)))})),i.wbg.__wbindgen_debug_string=function(n,t){var e=p(function n(t){const e=typeof t;if("number"==e||"boolean"==e||null==t)return""+t;if("string"==e)return`"${t}"`;if("symbol"==e){const n=t.description;return null==n?"Symbol":`Symbol(${n})`}if("function"==e){const n=t.name;return"string"==typeof n&&n.length>0?`Function(${n})`:"Function"}if(Array.isArray(t)){const e=t.length;let r="[";e>0&&(r+=n(t[0]));for(let o=1;o<e;o++)r+=", "+n(t[o]);return r+="]",r}const r=/\[object ([^\]]+)\]/.exec(toString.call(t));let o;if(!(r.length>1))return toString.call(t);if(o=r[1],"Object"==o)try{return"Object("+JSON.stringify(t)+")"}catch(n){return"Object"}return t instanceof Error?`${t.name}: ${t.message}\n${t.stack}`:o}(c(t)),o.__wbindgen_malloc,o.__wbindgen_realloc),r=b;h()[n/4+1]=r,h()[n/4+0]=e},i.wbg.__wbindgen_throw=function(n,t){throw new Error(l(n,t))},i.wbg.__wbindgen_rethrow=function(n){throw a(n)},("string"==typeof e||"function"==typeof Request&&e instanceof Request||"function"==typeof URL&&e instanceof URL)&&(e=fetch(e));const{instance:_,module:s}=await async function(n,t){if("function"==typeof Response&&n instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(n,t)}catch(t){if("application/wasm"==n.headers.get("Content-Type"))throw t;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",t)}const e=await n.arrayBuffer();return await WebAssembly.instantiate(e,t)}{const e=await WebAssembly.instantiate(n,t);return e instanceof WebAssembly.Instance?{instance:e,module:n}:e}}(await e,i);return o=_.exports,t.__wbindgen_wasm_module=s,o.__wbindgen_start(),o}}.call(this,e(1))},function(n,t){var e;e=function(){return this}();try{e=e||new Function("return this")()}catch(n){"object"==typeof window&&(e=window)}n.exports=e},function(n,t,e){"use strict";e.r(t);var r=e.p+"245e8d3e7f7e006303cb7242bc5dc2ef.wasm",o=e(0);const i=Object(o.default)(r).then(n=>o),c=i.then(n=>new n.Playground);self.onmessage=n=>{"runScript"===n.data.req?async function(n){const t=await c;function e(n){self.postMessage({req:"runScript/output",output:n})}try{let r=t.runScript(n,n=>{e("[PRINT] "+n)},n=>{e("[DEBUG] "+n)},n=>{self.postMessage({req:"runScript/updateOps",ops:n})});e(`\nScript returned: "${r}"`)}catch(n){e("\nEXCEPTION: "+n)}postMessage({req:"runScript/end"})}(n.data.script):console.log("Unknown message received by worker:",n.data)},i.then(()=>{self.postMessage({req:"_ready"})}),addEventListener("message",(function(n){var e,r=n.data,o=r.type,i=r.method,c=r.id,_=r.params;"RPC"===o&&i&&((e=t[i])?Promise.resolve().then((function(){return e.apply(t,_)})):Promise.reject("No such method")).then((function(n){postMessage({type:"RPC",id:c,result:n})})).catch((function(n){var t={message:n};n.stack&&(t.message=n.message,t.stack=n.stack,t.name=n.name),postMessage({type:"RPC",id:c,error:t})}))})),postMessage({type:"RPC",method:"ready"})}]);