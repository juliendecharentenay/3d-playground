<template>
  <div class="absolute inset-0 bg-gray-500 overflow-clip" id="viewer">
    <TouchAnimation :default_radius="0.10*Math.min(width, height)"
                    @error="on_error" v-if="loaded">
      <ViewerCanvas :elements="elements" :width="width" :height="height" @error="on_error" v-if="model !== null" />
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
      <SelectTool ref="select_tool" v-if="tool_name === 'select_tool'" 
                  v-model="tool_name" :entries="menu_tool_entries" 
                  />
      <AddTool    ref="add" v-if="tool_name === 'add'" 
                  v-model="element" @add="on_add" @close="() => {tool_name = 'select_tool'; element = null;}" 
                  />
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
import SelectTool from "./components/selecttool.vue";
import AddTool from "./components/addtool.vue";

import { mount_wasm } from "@/js/mount.js";

const MENU_TOOL_ENTRIES = [
  { value: 'select_tool', label: 'Select tool' },
  { value: 'add',         label: 'Add' },
  { value: 'translate',   label: 'Translate' },
  { value: 'rotate',      label: 'Rotate' },
  { value: 'stretch',     label: 'Stretch' },
];

function getModelElements(wasm) {
  return [
    wasm.GridBuilder.new()
        .center([0.0, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .tangent([1.0, 0.0, 0.0])
        .delta(0.1)
        .n(10)
        .build(),
  ];
}

export default {
  name: "App",
  setup() {
    const { width, height, resize } = useResizeable("viewer");
    const { error, on_error, catcher } = useError();
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
    SelectTool, AddTool,
  },
  mounted() {
    this.catcher("mounted", 
    () => {
      this.resize();
      this.mount_wasm()
      .then((wasm) => { this.wasm = wasm; })
      .then(() => {this.model = {elements: getModelElements(this.wasm),};})
      .then(() => {this.loaded = true;})
      .catch((e) => { this.on_error({msg: "Error in mount_wasm", e}); });
    });
  },
  data() { 
    return { 
      loaded: false, 
      wasm: null, 
      tool_name: 'select_tool',
      menu_tool_entries: MENU_TOOL_ENTRIES,
      model: null,
      element: null,
    }; 
  },
  computed: {
    tool: function() { return this.$refs[this.tool_name]; },
    touch_scroll_delta: function() { return 5.0; }, // console.log(this.tool); return this.tool === undefined || this.tool.scroll_delta === undefined ? 5.0 : this.tool.scroll_delta; },
    elements: function() {
      let arr = this.model.elements;
      if (this.element !== null) { console.log("Element added"); arr = arr.concat([this.element]); }
      return arr;
    },
  },
  methods: {
    on_touch_scroll_tool_scroll: function(evt) {
      this.catcher("on_touch_scroll_tool_scroll",
      () => {
        this.tool.scroll(evt);
      });
    },
    on_touch_scroll_tool_click: function(evt) {
      this.catcher("on_touch_scroll_tool_click", 
      () => {
        this.tool.click(evt);
      });
    },
    on_touch_scroll_event: function(key, evt) {
      console.log("Event", key, evt);
    },
    on_add: function() {
      this.catcher("on_add",
      () => {
        this.model.elements.push(this.element);
        this.element = null;
      });
    },
  },
}
</script>

