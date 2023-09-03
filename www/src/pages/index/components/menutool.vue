<template>
  <div>
    <div class="flex gap-1 cursor-pointer" @click="() => {$emit('open');}" v-if="open === false">
      <ChevronUpIcon class="w-5 h-5" />
      {{ current_entry.label }}
    </div>
    <div class="flex flex-col-reverse gap-y-1 items-stretch" v-if="open === true">
      <MenuToolEntry v-for="({value: v, label}, i) in entries"
                     :key="i"
                     :value="v" 
                     :active="value === v" 
                     :selected="index === i"
                     @click="() => {value = v;}">{{ label }}</MenuToolEntry>
    </div>
  </div>
</template>
<script>
import { ChevronUpIcon } from "@heroicons/vue/24/outline";
import MenuToolEntry from "./menutool/menutoolentry.vue";

export default {
  name: "MenuTool",
  props: {
    modelValue: {
      type: String,
      required: true,
    },
    open: {
      type: Boolean,
      default: false,
    },
    index: {
      type: Number,
      required: true,
    },
    entries: {
      type: Array,
      required: true,
    },
  },
  components: {
    ChevronUpIcon, MenuToolEntry,
  },
  data: function() {
    return {};
  },
  mounted: function() {
    try {
      //
    } catch (e) {
      this.on_error("Error in mounted", e);
    }
  },
  emits: [ 'update:modelValue', 'error' ],
  computed: {
    value: {
      get() { return this.modelValue !== null ? this.modelValue : 'select_tool'; },
      set(v) { this.$emit('update:modelValue', v); },
    },
    current_entry: function() {
      return this.entries.find((e) => e.value === this.value);
    },
  },
};
</script>

