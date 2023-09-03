<template>
  <div class="absolute inset-0 bg-gray-500 overflow-clip" id="viewer">
    <TouchAnimation :default_radius="0.10*Math.min(width, height)"
                    @error="on_error" v-if="loaded">
      <ViewerCanvas :width="width" :height="height" @error="on_error" />
      <div class="fixed right-5 bottom-5 flex flex-row items-end justify-end">
        <TouchScrollTool :delta="touch_scroll_delta"
                       @error="on_error"
                       @click="(evt) =>   {on_touch_scroll_tool_click(evt);}"
                       @scroll="(evt) =>  {on_touch_scroll_tool_scroll(evt);}"
                       @press="(evt) =>   {on_touch_scroll_event('press', evt);}"
                       @release="(evt) => {on_touch_scroll_event('release', evt);}"
                       />
      </div>
    </TouchAnimation>
    <div class="fixed bottom-5 left-2">
      <MenuTool v-model="tool" 
                :entries="menu_tool_entries"
                :index="menu_tool_index"
                :open="menu_tool_open" @open="() => {onMenuToolOpen();}" />
    </div>
  </div>
  <ModalErrorComposable :error="error" v-if="error !== null" @dismiss="error = null" />
</template>
<script>
import { computed } from 'vue';

import { useResizeable } from "@/extras/extra-vue-ui/composable/resizeable.js";
import { useError } from "@/extras/extra-vue-ui/composable/error.js";
import ModalErrorComposable from "@/extras/extra-vue-ui/modal/modalerrorcomposable.vue";
import ViewerCanvas from "./components/viewercanvas.vue";
import TouchAnimation from "@/extras/extra-vue-ui/touch/touchanimation.vue";
import TouchScrollTool from "@/extras/extra-vue-ui/touch/touchscrolltool.vue";
import MenuTool from "./components/menutool.vue";

import { mount_wasm } from "@/js/mount.js";

const MENU_TOOL_ENTRIES = [
  { value: 'select_tool', label: 'Select tool' },
  { value: 'add',         label: 'Add' },
  { value: 'translate',   label: 'Translate' },
  { value: 'rotate',      label: 'Rotate' },
  { value: 'stretch',     label: 'Stretch' },
];

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
    TouchAnimation, TouchScrollTool,
    MenuTool,
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
      current_tool: 'select_tool',
      menu_tool_index_: -1,
      menu_tool_entries: MENU_TOOL_ENTRIES,
      menu_tool_open: false,
    }; 
  },
  computed: {
    tool: {
      get()  { return this.current_tool; },
      set(v) { this.current_tool = v; this.menu_tool_open = false; },
    },
    menu_tool_index: function() {
      if (this.menu_tool_entries.length > 0) {
        while (this.menu_tool_index_ < 0) { this.menu_tool_index_ += this.menu_tool_entries.length; }
        return this.menu_tool_index_ % this.menu_tool_entries.length;
      } else {
        return 0;
      }
    },
    touch_scroll_delta: function() {
      if (this.menu_tool_open === true) {
        return 10.0;
      } else {
        return 5.0;
      }
    },
  },
  methods: {
    onMenuToolOpen: function() {
      this.catcher("onMenuToolOpen", 
      () => {
        this.menu_tool_index_ = this.menu_tool_entries.findIndex((e) => e.value === this.current_tool);
        if (this.menu_tool_index_ === -1) { throw new Error(`Unable to get index for tool ${this.current_tool}.`); }
        this.menu_tool_open = true;
      });
    },
    on_touch_scroll_tool_scroll: function(evt) {
      this.catcher("on_touch_scroll_tool_scroll",
      () => {
        if (this.menu_tool_open === true) {
          if ('vibrate' in navigator) { navigator.vibrate(100); }
          this.menu_tool_index_ = this.menu_tool_index + evt.delta;
        } else if (this.tool === 'select_tool') {
          // do nothing
        } else {
          throw new Error(`Scroll whell with tool ${this.tool} is not supported`);
        }
      });
    },
    on_touch_scroll_tool_click: function(evt) {
      this.catcher("on_touch_scroll_tool_click", 
      () => {
        if (this.menu_tool_open === true) {
          this.tool = this.menu_tool_entries[this.menu_tool_index].value;
        } else if (this.menu_tool_open === false) {
          this.onMenuToolOpen();
        } else if (this.tool === 'add') {
        } else {
          throw new Error(`Click with tool ${this.tool} is not supported`);
        }
      });
    },
    on_touch_scroll_event: function(key, evt) {
      console.log("Event", key, evt);
    },
  },
}
</script>

