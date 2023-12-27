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
    width:       { type: Number, required: true, },
    height:      { type: Number, required: true, },
    model:       { type: Object, required: true, },
    current_element: { type: Object, required: false, },
    camera_prop: { type: Object, },
  },
  emits: [ 'error', 'update:CameraProp', 'hover:Element', 'select:Element' ],
  data: function() {
    return {
      p_renderer: null,
      context: null,
      p_picker: null,
    };
  },
  watch: {
    model() { this.draw(); },
    current_element() { this.draw(); },
    height(newV, oldV) { 
      this.catcher("height update", () => { this.draw(); }); // if (this.camera !== null) { this.camera = this.camera.to_camera().height(newV); this.draw(); } });
    },
    width(newV, oldV)  { 
      this.catcher("width update", () => { this.draw(); }); // if (this.camera !== null) { this.camera = this.camera.to_camera().width(newV); this.draw(); } });
    },
    camera(newV) {
      this.catcher("camera update", () => { 
        if (newV !== null) {
          this.draw(); 
        } else {
          this.camera_initialise();
        }
      });
    },
  },
  computed: {
    camera: {
      get() { return this.camera_prop; },
      set(v) { this.$emit('update:CameraProp', v); },
    },
    picker: {
      get() {
        return this.catcher("picker:get",
        () => {
          if (this.p_picker === null && this.camera !== null) {
            this.p_picker = this.wasm.PickerBuilder.default()
            .camera(this.camera.as_camera())
            .build();
          }
          return this.p_picker;
        });
      },
      set(v) { this.p_picker = v; },
    },
    renderer: {
      get() { 
        return this.catcher("renderer:get",
          () => {
            if (this.p_renderer === null && this.camera !== null) {
              this.p_renderer = this.wasm.RendererBuilder.new()
              .camera(this.camera.as_camera())
              .build();
            }
            return this.p_renderer;
        });
      },
      set(v) { this.p_renderer = v; }
    },
  },
  mounted: function() {
    this.catcher("mounted",
    () => {
      this.context = this.$refs.viewer.getContext("webgl2");
      if (this.camera === null) { this.camera_initialise(); }
      this.draw();
    });
  },
  methods: {
    camera_initialise: function() {
      this.catcher("camera_initialise",
      () => {
        this.camera = this.wasm.CameraBuilder.basic()
        .width(this.width)
        .height(this.height)
        .eye([-0.5, 2.0, 2.0])
        .target([0.0, 0.0, 0.0])
        .up([0.0, 0.0, 1.0])
        .into();
      });
    },
    draw: function() {
      requestAnimationFrame(() => {
        this.catcher("draw",
        () => {
        /*
          this.context.viewport(0, 0, this.width, this.height);
          this.context.clearColor(6.0/255.0, 78.0/255.0, 59.0/255.0, 1.0);
          this.context.clear(this.context.COLOR_BUFFER_BIT | this.context.DEPTH_BUFFER_BIT);
          this.context.enable(this.context.CULL_FACE);
          this.context.enable(this.context.DEPTH_TEST);
          */
          if (this.renderer !== null) { 
            this.renderer = this.renderer.with_camera(this.camera.as_camera().height(this.height).width(this.width));
            this.model.draw(this.context, this.renderer);
            if (this.current_element != null) { this.current_element.draw(this.context, this.renderer); }

            if (this.picker !== null) {
              this.picker = this.picker
              .with_camera(this.camera.as_camera().height(this.height).width(this.width));

              const pick_hover = this.camera.pick_hover();
              if (pick_hover !== null) {
                // See https://webgl2fundamentals.org/webgl/lessons/webgl-picking.html
                this.picker = this.picker.with_pick_position(
                  pick_hover.clientX * this.width / pick_hover.target.clientWidth,
                  this.height - pick_hover.clientY * this.height / pick_hover.target.clientHeight - 1,
                );
                this.model.pick(this.context, this.picker);
                const r = this.picker.result();
                this.$emit("hover:Element", {uuid: (r !== undefined ? r : null)});
              } else {
                this.$emit("hover:Element", {uuid: null});
              }

              const pick_select = this.camera.pick_select();
              if (pick_select !== null) {
                // See https://webgl2fundamentals.org/webgl/lessons/webgl-picking.html
                this.picker = this.picker.with_pick_position(
                  pick_select.clientX * this.width / pick_select.target.clientWidth,
                  this.height - pick_select.clientY * this.height / pick_select.target.clientHeight - 1,
                );
                this.model.pick(this.context, this.picker);
                const r = this.picker.result();
                this.$emit("select:Element", {uuid: (r !== undefined ? r : null)});
              }
            }
          } else {
            setTimeout(() => {this.draw();}, 100);
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
