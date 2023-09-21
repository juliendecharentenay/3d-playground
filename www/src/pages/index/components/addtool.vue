<template>
  <div class="flex gap-x-1 cursor-pointer items-center" @click="() => {$emit('close');}">
    <ChevronLeftIcon class="w-5 h-5" />
    <div>Back</div>
  </div>
</template>
<script>
import { ChevronLeftIcon, } from "@heroicons/vue/24/outline";
import { useComponentError } from "@/extras/extra-vue-ui/composable/error.js";

export default {
  name: "AddTool",
  inject: [ 'wasm' ],
  setup: function(props, context) {
    const { on_error, catcher } = useComponentError(context);
    return { on_error, catcher };
  },
  props: {
    modelValue: { type: Object, }, // required: true,
  },
  components: { ChevronLeftIcon, },
  emits: [ 'update:modelValue', 'add', 'close' ],
  watch: {
    modelValue(newModelValue) {
      this.check_model_value(newModelValue);
    }
  },
  mounted: function() {
    this.check_model_value(this.modelValue);
  },
  methods: {
    check_model_value: function(v) {
      this.catcher("check_model_value",
      () => {
        if (v === null) {
          this.$emit('update:modelValue', 
            this.wasm.HexahedronBuilder.new()
              .start([0.0, 0.0, 0.0])
              .end([0.1, 0.1, 0.1])
              .build()
          );
        }
      });
    },
    scroll: function(evt) {
      this.catcher("scroll",
      () => {
        this.$emit(
          'update:modelValue', 
          this.modelValue.translate(this.wasm.Translation.new(evt.delta * 0.1, 0.0, 0.0))
        );
      });
    },
    click: function(evt) {
      this.catcher("click",
      () => {
        this.$emit('add');
      });
    },
  },
};
</script>
