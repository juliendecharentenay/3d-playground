<template>
  <div class="px-2 rounded-r-md text-center"
       :class="{
                 'text-slate-900 border-slate-900 border': active,
                 'text-slate-800': ! active,
                 'hover:bg-slate-500': ! selected,
                 'bg-slate-500': selected,
               }"
       @click="() => {$emit('click', value);}">
    <slot></slot>
  </div>
</template>
<script>
export default {
  name: "MenuToolEntry",
  props: {
    value: {
      type: String,
      required: true,
    },
    active: {
      type: Boolean,
      default: false,
    },
    selected: {
      type: Boolean,
      default: false,
    },
  },
  emits: [ 'click' ],
};
</script>
