var e,t,n,r,o,i={844:function(e,t,n){var r=this&&this.__awaiter||function(e,t,n,r){return new(n||(n=Promise))((function(o,i){function a(e){try{l(r.next(e))}catch(e){i(e)}}function u(e){try{l(r.throw(e))}catch(e){i(e)}}function l(e){var t;e.done?o(e.value):(t=e.value,t instanceof n?t:new n((function(e){e(t)}))).then(a,u)}l((r=r.apply(e,t||[])).next())}))},o=this&&this.__generator||function(e,t){var n,r,o,i,a={label:0,sent:function(){if(1&o[0])throw o[1];return o[1]},trys:[],ops:[]};return i={next:u(0),throw:u(1),return:u(2)},"function"==typeof Symbol&&(i[Symbol.iterator]=function(){return this}),i;function u(i){return function(u){return function(i){if(n)throw new TypeError("Generator is already executing.");for(;a;)try{if(n=1,r&&(o=2&i[0]?r.return:i[0]?r.throw||((o=r.return)&&o.call(r),0):r.next)&&!(o=o.call(r,i[1])).done)return o;switch(r=0,o&&(i=[2&i[0],o.value]),i[0]){case 0:case 1:o=i;break;case 4:return a.label++,{value:i[1],done:!1};case 5:a.label++,r=i[1],i=[0];continue;case 7:i=a.ops.pop(),a.trys.pop();continue;default:if(!((o=(o=a.trys).length>0&&o[o.length-1])||6!==i[0]&&2!==i[0])){a=0;continue}if(3===i[0]&&(!o||i[1]>o[0]&&i[1]<o[3])){a.label=i[1];break}if(6===i[0]&&a.label<o[1]){a.label=o[1],o=i;break}if(o&&a.label<o[2]){a.label=o[2],a.ops.push(i);break}o[2]&&a.ops.pop(),a.trys.pop();continue}i=t.call(e,a)}catch(e){i=[6,e],r=0}finally{n=o=0}if(5&i[0])throw i[1];return{value:i[0]?i[1]:void 0,done:!0}}([i,u])}}},i=Promise.all([n.e(731),n.e(628)]).then(n.bind(n,628)),a="board",u="hint",l="month-form",c="day-form";document.getElementById("find-button").onclick=function(){var e=document.getElementById(l).selectedIndex+1,t=document.getElementById(c).selectedIndex+1;document.getElementById(a).innerText="Searching...",document.getElementById(u).innerText="",function(e,t){return r(this,void 0,void 0,(function(){var n;return o(this,(function(r){switch(r.label){case 0:if(!(1<=e&&e<=12&&1<=t&&t<=31))throw new Error("Error: invalid date: "+e+", "+t);return[4,i.then((function(n){return n.find_solution(e,t,!1)}))];case 1:return""!=(n=r.sent())?[2,n]:(document.getElementById(u).innerText="(No solution without flipping pieces.)",[4,i.then((function(n){return n.find_solution(e,t,!0)}))]);case 2:return[2,r.sent()]}}))}))}(e,t).then((function(e){var t;console.log(e),t=e,document.getElementById(a).innerText=t}))},function(){var e=new Date,t=document.getElementById(l);["January","February","March","April","May","June","July","August","September","October","November","December"].forEach((function(e){var n=document.createElement("option");n.text=e,t.add(n)})),t.selectedIndex=e.getMonth();for(var n=document.getElementById(c),r=1;r<=31;r++){var o=document.createElement("option");o.text=r.toString(),n.add(o)}n.selectedIndex=e.getDate()-1}()}},a={};function u(e){var t=a[e];if(void 0!==t)return t.exports;var n=a[e]={id:e,loaded:!1,exports:{}};return i[e].call(n.exports,n,n.exports,u),n.loaded=!0,n.exports}u.m=i,u.c=a,u.d=(e,t)=>{for(var n in t)u.o(t,n)&&!u.o(e,n)&&Object.defineProperty(e,n,{enumerable:!0,get:t[n]})},u.f={},u.e=e=>Promise.all(Object.keys(u.f).reduce(((t,n)=>(u.f[n](e,t),t)),[])),u.u=e=>e+".main.js",u.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),u.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),e={},u.l=(t,n,r,o)=>{if(e[t])e[t].push(n);else{var i,a;if(void 0!==r)for(var l=document.getElementsByTagName("script"),c=0;c<l.length;c++){var s=l[c];if(s.getAttribute("src")==t){i=s;break}}i||(a=!0,(i=document.createElement("script")).type="module",i.charset="utf-8",i.timeout=120,u.nc&&i.setAttribute("nonce",u.nc),i.src=t),e[t]=[n];var d=(n,r)=>{i.onerror=i.onload=null,clearTimeout(f);var o=e[t];if(delete e[t],i.parentNode&&i.parentNode.removeChild(i),o&&o.forEach((e=>e(r))),n)return n(r)},f=setTimeout(d.bind(null,void 0,{type:"timeout",target:i}),12e4);i.onerror=d.bind(null,i.onerror),i.onload=d.bind(null,i.onload),a&&document.head.appendChild(i)}},u.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},u.p="./dist/",(()=>{var e={179:0};u.f.j=(t,n)=>{var r=u.o(e,t)?e[t]:void 0;if(0!==r)if(r)n.push(r[2]);else{var o=new Promise(((n,o)=>r=e[t]=[n,o]));n.push(r[2]=o);var i=u.p+u.u(t),a=new Error;u.l(i,(n=>{if(u.o(e,t)&&(0!==(r=e[t])&&(e[t]=void 0),r)){var o=n&&("load"===n.type?"missing":n.type),i=n&&n.target&&n.target.src;a.message="Loading chunk "+t+" failed.\n("+o+": "+i+")",a.name="ChunkLoadError",a.type=o,a.request=i,r[1](a)}}),"chunk-"+t,t)}};var t=(t,n)=>{var r,o,[i,a,l]=n,c=0;for(r in a)u.o(a,r)&&(u.m[r]=a[r]);for(l&&l(u),t&&t(n);c<i.length;c++)o=i[c],u.o(e,o)&&e[o]&&e[o][0](),e[i[c]]=0},n=self.webpackChunk=self.webpackChunk||[];n.forEach(t.bind(null,0)),n.push=t.bind(null,n.push.bind(n))})(),n={},r={551:function(){return{"./index_bg.js":{__wbg_alert_47c54d3b66d79e8d:function(e,n){return void 0===t&&(t=u.c[785].exports),t.Vk(e,n)}}}}},o={628:[551]},u.w={},u.f.wasm=function(e,t){(o[e]||[]).forEach((function(o,i){var a=n[o];if(a)t.push(a);else{var l,c=r[o](),s=fetch(u.p+""+{628:{551:"3bae4bf3c1f07986b580"}}[e][o]+".module.wasm");l=c instanceof Promise&&"function"==typeof WebAssembly.compileStreaming?Promise.all([WebAssembly.compileStreaming(s),c]).then((function(e){return WebAssembly.instantiate(e[0],e[1])})):"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(s,c):s.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,c)})),t.push(n[o]=l.then((function(e){return u.w[o]=(e.instance||e).exports})))}}))},u(844);