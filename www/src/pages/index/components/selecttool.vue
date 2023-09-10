<template>
  <MenuTool v-model="tool"
            :entries="entries"
            :index="index"
            :open="open"
            @open="() => {on_open();}"
            />
</template>
<script>
import MenuTool from "./menutool.vue";
import { useComponentError } from "@/extras/extra-vue-ui/composable/error.js";

export default {
  name: "SelectTool",
  setup: function(props, context) {
    const { on_error, catcher } = useComponentError(context);
    return { on_error, catcher };
  },
  components: { MenuTool, },
  props: {
    modelValue: {
      type: String,
      required: true,
    },
    entries: {
      type: Array,
      required: true,
    },
  },
  data() {
    return {
      index_: -1,
      open: false,
    };
  },
  computed: {
    scroll_delta: function() { return 10.0; },
    tool: {
      get() { return this.modelValue; },
      set(v) { console.log("emit: ", v); this.$emit('update:modelValue', v); this.open = false; },
    },
    index: function() {
      if (this.entries.length > 0) {
        while (this.index_ < 0) { this.index_ += this.entries.length; }
        return this.index_ % this.entries.length;
      } else {
        return 0;
      }
    },
  },
  methods: {
    scroll: function(evt) {
      this.catcher("scroll",
      () => {
        if ('vibrate' in navigator) { navigator.vibrate(100); }
        this.index_ = this.index + evt.delta;
      });
    },
    click: function(evt) {
      this.catcher("click", 
      () => {
        if (this.open) {
          this.tool = this.entries[this.index].value;
        } else {
          this.on_open();
        }
      });
    },
    on_open: function() {
      this.catcher("on_open",
      () => {
        this.index_ = this.entries.findIndex((e) => e.value === this.modelValue);
        if (this.index_ === -1) { throw new Error("Unable to get index for tool '${this.modelValue}'."); }
        this.open = true;
      });
    },
  },
};
</script>
