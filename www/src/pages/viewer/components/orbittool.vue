<template>
  <div class="flex gap-1 cursor-pointer" @click="() => {$emit('click');}">
    <ChevronUpIcon class="w-5 h-5" />
    Select
  </div>
</template>
<script>
import { ChevronUpIcon } from "@heroicons/vue/24/outline";
import { useComponentError } from "@/extras/extra-vue-ui/composable/error.js";
import { EventScroll } from "@/extras/extra-vue-ui/touch/events.js";

export default {
  name: "OrbitTool",
  setup: function(props, context) {
    const { on_error, catcher } = useComponentError(context);
    return { on_error, catcher };
  },
  components: { ChevronUpIcon, },
  props: {
    modelValue: {
      type: Object,
      required: true,
    },
  },
  emits: [ 'click', 'error', 'update:modelValue' ],
  data() {
    return { };
  },
  computed: { 
    camera: function() { this.modelValue; },
  },
  methods: {
    scroll: function(evt) {
      this.catcher("scroll",
      () => {
        if (this.modelValue !== undefined) { 
          const camera = this.modelValue;
          if (evt.type === EventScroll.LEFT) {
            this.$emit('update:modelValue', camera.as_camera().orbit_around_target(0.0, evt.delta));
          } else if (evt.type === EventScroll.TOP) {
            this.$emit('update:modelValue', camera.as_camera().orbit_around_target(evt.delta, 0.0));
          } else if (evt.type === EventScroll.BTMRIGHT) {
            this.$emit('update:modelValue', camera.as_camera().zoom_front(evt.delta));
          } else {
            throw new(`Scroll event type ${evt.type} is not supported`);
          }
        }
      });
    },
    click: function(evt) {
      this.catcher("click", () => { this.$emit('click'); });
    },
  },
};
</script>
