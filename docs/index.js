!function(e){function n(n){for(var t,o,u=n[0],c=n[1],_=0,i=[];_<u.length;_++)o=u[_],Object.prototype.hasOwnProperty.call(r,o)&&r[o]&&i.push(r[o][0]),r[o]=0;for(t in c)Object.prototype.hasOwnProperty.call(c,t)&&(e[t]=c[t]);for(a&&a(n);i.length;)i.shift()()}var t={},r={0:0};var o={};var u={3:function(){return{"./index_bg.js":{__wbindgen_object_drop_ref:function(e){return t[2].exports.G(e)},__wbg_beginPath_733d5a9e3e769d24:function(e){return t[2].exports.b(e)},__wbg_moveTo_18ace182fe51d75d:function(e,n,r){return t[2].exports.s(e,n,r)},__wbindgen_string_new:function(e,n){return t[2].exports.H(e,n)},__wbg_setfillStyle_528a6a267c863ae7:function(e,n){return t[2].exports.v(e,n)},__wbg_arc_bdfc39ad6001708b:function(e,n,r,o,u,c){return t[2].exports.a(e,n,r,o,u,c)},__wbg_closePath_64f527552526a127:function(e){return t[2].exports.e(e)},__wbg_fill_dc4e97599365a189:function(e){return t[2].exports.h(e)},__wbg_setfont_884816cc1b46ae3f:function(e,n,r){return t[2].exports.w(e,n,r)},__wbg_fillText_25221e9cc35a1850:function(e,n,r,o,u){return t[2].exports.g(e,n,r,o,u)},__wbg_toDataURL_a5b10fd0735f52a6:function(e,n){return t[2].exports.z(e,n)},__wbg_setsrc_3eb04f553f8335c7:function(e,n,r){return t[2].exports.y(e,n,r)},__wbg_clearRect_07caefec3496ced1:function(e,n,r,o,u){return t[2].exports.d(e,n,r,o,u)},__wbg_getElementById_f3e94458ce77f0d0:function(e,n,r){return t[2].exports.j(e,n,r)},__wbg_instanceof_HtmlInputElement_8cafe5f30dfdb6bc:function(e){return t[2].exports.q(e)},__wbg_value_0627d4b1c27534e6:function(e,n){return t[2].exports.A(e,n)},__wbindgen_object_clone_ref:function(e){return t[2].exports.F(e)},__wbg_instanceof_Window_c4b70662a0d2c5ec:function(e){return t[2].exports.r(e)},__wbg_document_1c64944725c0d81d:function(e){return t[2].exports.f(e)},__wbg_instanceof_HtmlCanvasElement_25d964a0dde6717e:function(e){return t[2].exports.n(e)},__wbg_instanceof_HtmlImageElement_fbc502504b49055e:function(e){return t[2].exports.p(e)},__wbg_getContext_f701d0231ae22393:function(e,n,r){return t[2].exports.i(e,n,r)},__wbg_instanceof_CanvasRenderingContext2d_3abbe7ec7af32cae:function(e){return t[2].exports.m(e)},__wbg_instanceof_HtmlElement_df66c8b4a687aa43:function(e){return t[2].exports.o(e)},__wbg_setonclick_8da32c8c00a7359b:function(e,n){return t[2].exports.x(e,n)},__wbg_self_c6fbdfc2918d5e58:function(){return t[2].exports.u()},__wbg_window_baec038b5ab35c54:function(){return t[2].exports.B()},__wbg_globalThis_3f735a5746d41fbd:function(){return t[2].exports.k()},__wbg_global_1bc0b39582740e95:function(){return t[2].exports.l()},__wbindgen_is_undefined:function(e){return t[2].exports.E(e)},__wbg_newnoargs_be86524d73f67598:function(e,n){return t[2].exports.t(e,n)},__wbg_call_888d259a5fefc347:function(e,n){return t[2].exports.c(e,n)},__wbindgen_debug_string:function(e,n){return t[2].exports.D(e,n)},__wbindgen_throw:function(e,n){return t[2].exports.I(e,n)},__wbindgen_closure_wrapper59:function(e,n,r){return t[2].exports.C(e,n,r)}}}}};function c(n){if(t[n])return t[n].exports;var r=t[n]={i:n,l:!1,exports:{}};return e[n].call(r.exports,r,r.exports,c),r.l=!0,r.exports}c.e=function(e){var n=[],t=r[e];if(0!==t)if(t)n.push(t[2]);else{var _=new Promise((function(n,o){t=r[e]=[n,o]}));n.push(t[2]=_);var i,f=document.createElement("script");f.charset="utf-8",f.timeout=120,c.nc&&f.setAttribute("nonce",c.nc),f.src=function(e){return c.p+""+({}[e]||e)+".js"}(e);var a=new Error;i=function(n){f.onerror=f.onload=null,clearTimeout(s);var t=r[e];if(0!==t){if(t){var o=n&&("load"===n.type?"missing":n.type),u=n&&n.target&&n.target.src;a.message="Loading chunk "+e+" failed.\n("+o+": "+u+")",a.name="ChunkLoadError",a.type=o,a.request=u,t[1](a)}r[e]=void 0}};var s=setTimeout((function(){i({type:"timeout",target:f})}),12e4);f.onerror=f.onload=i,document.head.appendChild(f)}return({1:[3]}[e]||[]).forEach((function(e){var t=o[e];if(t)n.push(t);else{var r,_=u[e](),i=fetch(c.p+""+{3:"c60058b5df90d6b61097"}[e]+".module.wasm");if(_ instanceof Promise&&"function"==typeof WebAssembly.compileStreaming)r=Promise.all([WebAssembly.compileStreaming(i),_]).then((function(e){return WebAssembly.instantiate(e[0],e[1])}));else if("function"==typeof WebAssembly.instantiateStreaming)r=WebAssembly.instantiateStreaming(i,_);else{r=i.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,_)}))}n.push(o[e]=r.then((function(n){return c.w[e]=(n.instance||n).exports})))}})),Promise.all(n)},c.m=e,c.c=t,c.d=function(e,n,t){c.o(e,n)||Object.defineProperty(e,n,{enumerable:!0,get:t})},c.r=function(e){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},c.t=function(e,n){if(1&n&&(e=c(e)),8&n)return e;if(4&n&&"object"==typeof e&&e&&e.__esModule)return e;var t=Object.create(null);if(c.r(t),Object.defineProperty(t,"default",{enumerable:!0,value:e}),2&n&&"string"!=typeof e)for(var r in e)c.d(t,r,function(n){return e[n]}.bind(null,r));return t},c.n=function(e){var n=e&&e.__esModule?function(){return e.default}:function(){return e};return c.d(n,"a",n),n},c.o=function(e,n){return Object.prototype.hasOwnProperty.call(e,n)},c.p="",c.oe=function(e){throw console.error(e),e},c.w={};var _=window.webpackJsonp=window.webpackJsonp||[],i=_.push.bind(_);_.push=n,_=_.slice();for(var f=0;f<_.length;f++)n(_[f]);var a=i;c(c.s=0)}([function(e,n,t){t.e(1).then(t.bind(null,1)).catch(console.error)}]);