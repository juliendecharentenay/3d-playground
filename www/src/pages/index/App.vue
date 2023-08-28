<template>
  <div class="absolute inset-0 bg-gray-500" id="viewer">
    <ViewerCanvas :width="width" :height="height" @error="on_error" v-if="loaded" />
  </div>
  <ModalErrorComposable :error="error" v-if="error !== null" @dismiss="error = null" />
</template>
<script>
import { computed } from 'vue';

import { useResizeable } from "@/extras/extra-vue-ui/composable/resizeable.js";
import { useError } from "@/extras/extra-vue-ui/composable/error.js";
import ModalErrorComposable from "@/extras/extra-vue-ui/modal/modalerrorcomposable.vue";
import ViewerCanvas from "./components/viewercanvas.vue";

import { mount_wasm } from "@/js/mount.js";

export default {
  name: "App",
  setup() {
    const { width, height, resize } = useResizeable("viewer");
    const { error, on_error, catcher } = useError(true);
    return { width, height, resize, 
             error, on_error, catcher };
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
    ViewerCanvas,
  },
  mounted() {
    this.catcher("mounted", 
    () => {
      this.resize();
      this.mount_wasm()
      .then((wasm) => { this.wasm = wasm; })
      .then(() => {this.loaded = true;})
      .catch((e) => { this.on_error({msg: "Error in mount_wasm", e}); });
    });
  },
  data() { 
    return { 
      loaded: false, 
      wasm: null, 
    }; 
  },
}
</script>

