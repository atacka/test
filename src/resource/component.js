// import { ref } from 'https://unpkg.com/vue@3/dist/vue.esm-browser.js'
import { ref } from 'http://wry.localhost/resource/vue@3.5.13/dist/vue.esm-browser.js'

export default {
  setup() {
    const count = ref(0)
    const state = ref({})

    function cntup(event) {
      window.ipc.postMessage(JSON.stringify({ kind : "count", data : 1 }));
    }
    const onClickFunc = async () =>{
      const response = await fetch("http://wry.localhost/", {
        method: "POST",
        body: JSON.stringify({ kind : "count", data : 2 }),
      });
      console.log(response);
    };
    const runScript = async () =>{
      const response = await fetch("http://wry.localhost/", {
        method: "POST",
        body: JSON.stringify({ kind : "script", data : "py script.py" }),
      });
      console.log(response);
    };

    window.addEventListener('customEvent', function(e) {
      console.log('customEvent', e);
      state.value = e.detail;
    }, false);

    return { count, state, cntup, onClickFunc, runScript }
  },
  template: `
    <div>Count is: {{ count }}</div>
    <div>state is: {{ state.count }}</div>
    <button @click="count++">Add 1 in js</button>
    <button @click="cntup">cnt up in rust</button>
    <button @click="runScript">script run</button>
  `
}
