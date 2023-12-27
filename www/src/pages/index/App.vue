<template>
<div class="relative isolate overflow-hidden bg-white">
  <svg class="absolute inset-0 -z-10 h-full w-full stroke-gray-200 [mask-image:radial-gradient(100%_100%_at_top_right,white,transparent)]" aria-hidden="true">
    <defs>
      <pattern id="0787a7c5-978c-4f66-83c7-11c213f99cb7" width="200" height="200" x="50%" y="-1" patternUnits="userSpaceOnUse">
        <path d="M.5 200V.5H200" fill="none" />
      </pattern>
    </defs>
    <rect width="100%" height="100%" stroke-width="0" fill="url(#0787a7c5-978c-4f66-83c7-11c213f99cb7)" />
  </svg>

  <div class="mx-auto max-w-7xl px-6 pb-10 lg:flex lg:px-8 lg:py-10">
    <div class="mx-auto max-w-2xl lg:mx-0 lg:max-w-xl lg:flex-shrink-0 lg:pt-8">
      <h1 class="mt-10 text-4xl font-bold tracking-tight text-gray-900 sm:text-6xl">3D Playground</h1>
      <p class="mt-6 text-lg leading-8 text-gray-600">A WebGL based 3D modelling engine using Rust/WASM</p>
      <div class="mt-10 flex items-center gap-x-6">
        <a href="#" class="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
          @click="on_new">New</a>
        <a href="https://github.com/juliendecharentenay/3dp" class="text-sm font-semibold leading-6 text-gray-900">OSS on Github <span aria-hidden="true">→</span></a>
      </div>

      <div class="mt-10 lg:-mx-10">
      <ModelList @error="on_error" v-if="wasm !== null" />
      </div>

    </div>
    <div class="hidden lg:block mx-auto mt-16 flex max-w-2xl sm:mt-24 lg:ml-10 lg:mr-0 lg:mt-0 lg:max-w-none lg:flex-none xl:ml-32">
      <div class="max-w-3xl flex-none sm:max-w-5xl lg:max-w-none">
        <div class="-m-2 rounded-xl bg-gray-900/5 p-2 ring-1 ring-inset ring-gray-900/10 lg:-m-4 lg:rounded-2xl lg:p-4">
          <img src="./assets/screenshot.png" alt="App screenshot" width="2432" height="1442" class="w-[76rem] rounded-md shadow-2xl ring-1 ring-gray-900/10">
        </div>
      </div>
    </div>
  </div>
  
  <ModalErrorComposable :error="error" v-if="error !== null" />
</div>
</template>
<script>
import { computed } from 'vue';

import { useError } from "@/extras/extra-vue-ui/composable/error.js";
import ModalErrorComposable from "@/extras/extra-vue-ui/modal/modalerrorcomposable.vue";

import { mount_wasm } from "@/js/mount.js";
import ModelList from "./components/modellist.vue";
import { save_model } from "@/js/storage.js";

export default {
  name: "App",
  setup() {
    const { error, on_error, catcher } = useError();
    return { 
      error, on_error, catcher,
    };
  },
  provide() {
    return {
      wasm: computed(() => this.wasm),
    };
  },
  props: {
    mount_wasm: {
      type: Function,
      default: mount_wasm,
    },
  },
  components: {
    ModalErrorComposable,
    ModelList,
  },
  mounted() {
    this.catcher("mounted", 
    () => {
      this.mount_wasm()
      .then((wasm) => { this.wasm = wasm; })
      .catch((e) => { this.on_error({msg: "Error in mount_wasm", e}); });
    });
  },
  data() { 
    return { 
      wasm: null, 
    }; 
  },
  computed: {
    loaded: function() { return this.wasm !== null; },
    on_new: function() {
      this.catcher("on_new",
      () => {
        if (this.wasm === null) { throw new Error("Wasm not initialised yet"); }
        const model = this.wasm.ModelWasmed.default().add_element(
          this.wasm.GridBuilder.new()
          .center([0.0, 0.0, 0.0])
          .normal([0.0, 0.0, 1.0])
          .tangent([1.0, 0.0, 0.0])
          .delta(0.1)
          .n(10)
          .build()
        );
        save_model(model)
        .then((model) => {
          const url = new URL(window.location); url.pathname = "/viewer.html"; url.searchParams.set("id", model.id());
          window.location = url;
        })
        .catch((e) => { this.on_error({msg: "Error in on_new::save_model", e}); });
      });
    },
  },
  methods: {
  },
}
</script>

