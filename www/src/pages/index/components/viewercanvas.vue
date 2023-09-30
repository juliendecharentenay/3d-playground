<template>
  <canvas :width="width" :height="height" class="bg-gray-100 absolute inset-0" ref="viewer"
    @mousedown="mouse_down"
    @mousemove="mouse_move"
    @mouseup="mouse_up"
    @wheel="wheel"
    @touchstart="touch"
    @touchend="touch"
    @touchcancel="touch"
    @touchmove="touch"
    >
  </canvas>
</template>
<script>
import { useComponentError } from "@/extras/extra-vue-ui/composable/error.js";
export default {
  name: "ViewerCanvas",
  inject: [ 'wasm' ],
  setup: function(props, context) {
    const { on_error, catcher } = useComponentError(context);
    return { on_error, catcher };
  },
  props: {
    width:    { type: Number, required: true, },
    height:   { type: Number, required: true, },
    elements: { type: Array, required: true, },
  },
  emits: [ 'error' ],
  data: function() {
    return {
      camera: null,
      renderer: null,
      context: null,
    };
  },
  watch: {
    elements() { this.draw(); },
    height(newV, oldV) { 
      this.catcher("height update", () => { if (this.camera !== null) { this.camera = this.camera.to_camera().height(newV); this.draw(); } });
    },
    width(newV, oldV)  { 
      this.catcher("width update", () => { if (this.camera !== null) { this.camera = this.camera.to_camera().width(newV); this.draw(); } });
    },
  },
  mounted: function() {
    this.catcher("mounted",
    () => {
      this.context = this.$refs.viewer.getContext("webgl2");
      this.camera = this.wasm.CameraBuilder.basic()
        .width(this.width)
        .height(this.height)
        .eye([-0.5, 2.0, 2.0])
        .target([0.0, 0.0, 0.0])
        .up([0.0, 0.0, 1.0])
        .into();
      this.renderer = this.wasm.RendererBuilder.new()
        .camera(this.camera.as_camera())
        .build();

      this.draw();
    });
  },
  methods: {
    draw: function() {
      requestAnimationFrame(() => {
        this.catcher("draw",
        () => {
          this.context.viewport(0, 0, this.width, this.height);
          this.context.clearColor(6.0/255.0, 78.0/255.0, 59.0/255.0, 1.0);
          this.context.clear(this.context.COLOR_BUFFER_BIT | this.context.DEPTH_BUFFER_BIT);
          this.context.enable(this.context.CULL_FACE);
          this.context.enable(this.context.DEPTH_TEST);
          if (this.camera !== null) { 
            this.renderer = this.renderer.with_camera(this.camera.as_camera());
            this.elements.forEach((e) => { e.draw(this.context, this.renderer); });
          }
        });
      });
    },
    mouse_down: function(e) {
      this.catcher("mouse_down", () => { if (this.camera !== null) { this.camera = this.camera.on_mouse_down(e); this.draw(); } } );
    },
    mouse_move: function(e) {
      this.catcher("mouse_move", () => { if (this.camera !== null) { this.camera = this.camera.on_mouse_move(e); this.draw(); } } );
    },
    mouse_up: function(e) {
      this.catcher("mouse_up", () => { if (this.camera !== null) { this.camera = this.camera.on_mouse_up(e); this.draw(); } } );
    },
    wheel: function(e) {
      this.catcher("wheel", () => { if (this.camera !== null) { this.camera = this.camera.on_wheel(e); this.draw(); } } );
    },
    touch: function(e) {
      this.catcher("touch", () => { if (this.camera !== null) { this.camera = this.camera.on_touch(e); this.draw(); } } );
    },
  },
}
</script>
