<template>
  <div class="absolute inset-0 bg-gray-500 overflow-clip" id="viewer">
    <TouchAnimation :default_radius="0.10*Math.min(width, height)"
                    @error="on_error" v-if="loaded">
      <ViewerCanvas v-if="model !== null" 
                    :model="model"
                    :current_element="element" :width="width" :height="height" @error="on_error" 
                    :camera_prop="camera" @update:CameraProp="(v) => {camera = v;}"
                    />
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
      <OrbitTool  ref="default_tool" v-if="tool_name === 'default_tool' && camera !== null" 
                  v-model="camera"
                  @click="() => {tool_name = 'select_tool';}"
                  @error="on_error"
                  />
      <SelectTool ref="select_tool" v-if="tool_name === 'select_tool'" 
                  v-model="tool_name" :entries="menu_tool_entries" 
                  @error="on_error"
                  />
      <AddTool    ref="add_tool" v-if="tool_name === 'add_tool'" 
                  v-model="element" @add="on_add" @close="() => {tool_name = 'default_tool'; element = null;}" 
                  @error="on_error"
                  />
    </div>
    <div class="fixed top-2 left-2">
      <div class="flex flex-col">
      <input class="text-slate-400 text-sm bg-transparent focus:outline-none" v-model="name" placeholder="Untitled" />
      <div class="text-slate-500 text-xs" v-if="save_state !== null">{{ save_state }}</div>
      </div>
    </div>
  </div>
  <ModalErrorComposable :error="error" v-if="error !== null" @dismiss="error = null" />
</template>
<script>
import { computed } from 'vue';

import { useResizeable } from "@/extras/extra-vue-ui/composable/resizeable.js";
import { useError } from "@/extras/extra-vue-ui/composable/error.js";
import { useSaveable } from "@/extras/extra-vue-ui/composable/saveable.js";
import ModalErrorComposable from "@/extras/extra-vue-ui/modal/modalerrorcomposable.vue";
import ViewerCanvas from "./components/viewercanvas.vue";
import TouchAnimation from "@/extras/extra-vue-ui/touch/touchanimation.vue";
import TouchScrollTool from "@/extras/extra-vue-ui/touch/touchscrolltool.vue";
import SelectTool from "./components/selecttool.vue";
import AddTool from "./components/addtool.vue";
import OrbitTool from "./components/orbittool.vue";

import { mount_wasm } from "@/js/mount.js";
import { save_model, read_model } from "@/js/storage.js";

const MENU_TOOL_ENTRIES = [
  { value: 'default_tool',   label: 'Close' },
  { value: 'select_tool',    label: 'Select' },
  { value: 'add_tool',       label: 'Add' },
  { value: 'new_model',      label: 'New' },
  { value: 'exit',           label: 'Exit' },
  // { value: 'translate', label: 'Translate' },
  // { value: 'rotate',    label: 'Rotate' },
  // { value: 'stretch',   label: 'Stretch' },
];

function getGrid(wasm) {
  const grid = wasm.GridBuilder.new()
        .center([0.0, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .tangent([1.0, 0.0, 0.0])
        .delta(0.1)
        .n(10)
        .build();
  return grid;
}

export default {
  name: "App",
  setup() {
    const { width, height, resize } = useResizeable("viewer");
    const { error, on_error, catcher } = useError();
    const { register_save_function, save, save_state } = useSaveable();
    return { width, height, resize, 
             error, on_error, catcher,
             register_save_function, save, save_state,
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
    ViewerCanvas,
    TouchAnimation, TouchScrollTool,
    SelectTool, AddTool, OrbitTool,
  },
  mounted() {
    this.catcher("mounted", 
    () => {
      this.resize();
      this.mount_wasm()
      .then((wasm) => { this.wasm = wasm; })
      .then(() => { this.on_read(); })
      .then(() => { this.register_save_function(() => { this.on_save(); }); })
      .catch((e) => { this.on_error({msg: "Error in mount_wasm", e}); });
    });
  },
  data() { 
    return { 
      wasm: null, 
      tool_name_data: 'default_tool',
      menu_tool_entries: MENU_TOOL_ENTRIES,
      model: null,
      element: null,
      camera_data: null,
    }; 
  },
  computed: {
    camera: {
      get() { return this.camera_data; },
      set(v) {
        this.catcher("camera:set",
        () => {
          this.camera_data = v;
          if (this.model !== null && this.camera_data !== null) {
            this.model = this.model.set_camera(this.camera_data.as_camera());
            this.save();
          }
        });
      },
    },
    name: {
      get() { return this.model !== null ? this.model.name() : ""; },
      set(v) {
        this.catcher("name:set",
        () => {
          if (this.model !== null) {
            this.model = this.model.set_name(v);
            this.save();
          }
        });
      },
    },
    loaded: function() { return this.wasm !== null && this.model !== null; },
    tool: function() { return this.$refs[this.tool_name]; },
    touch_scroll_delta: function() { return 5.0; }, // console.log(this.tool); return this.tool === undefined || this.tool.scroll_delta === undefined ? 5.0 : this.tool.scroll_delta; },
    tool_name: {
      get() { return this.tool_name_data; },
      set(v) { 
        if (v === 'exit') {
          window.location.href = '/';
        } else if (v === 'new_model') {
          this.on_new_model();
          this.tool_name_data = 'default_tool';
        } else {
          this.tool_name_data = v; 
        }
      },
    },
  },
  methods: {
    on_save: function() {
      this.catcher("on_save", 
      () => { if (this.model !== null) { save_model(this.model); } }
      );
    },
    on_read: function() {
      try {
        const sp = new URLSearchParams(window.location.search);
        if (sp.has("id")) {
          const model = read_model(this.wasm, sp.get("id"));
          if (model === null) { throw new Error(`Unable to retrieve model with id ${id}`); }
          this.model = model;
          this.camera = this.model.camera();
        } else {
          this.on_new_model();
        }
      } catch (e) {
        console.error(e);
        if (this.model === null) { this.on_new_model(); }
      }
    },
    on_new_model: function() {
      this.catcher("on_new_model",
      () => {
        if (this.wasm === null) { throw new Error("Wasm not initialised yet"); }
        let grid = getGrid(this.wasm);
        this.model = this.wasm.ModelWasmed.default().add_element(grid);
        this.camera = null;
      });
    },
    on_touch_scroll_tool_scroll: function(evt) {
      this.catcher("on_touch_scroll_tool_scroll",
      () => {
        if ('vibrate' in navigator) { navigator.vibrate(100); }
        this.tool.scroll(evt);
      });
    },
    on_touch_scroll_tool_click: function(evt) {
      this.catcher("on_touch_scroll_tool_click", 
      () => {
        if ('vibrate' in navigator) { navigator.vibrate(250); }
        this.tool.click(evt);
      });
    },
    on_touch_scroll_event: function(key, evt) {
      console.log("Event", key, evt);
    },
    on_add: function() {
      this.catcher("on_add",
      () => {
        this.model = this.model.add_element(this.element);
        this.save();
        this.element = null;
      });
    },
  },
}
</script>

