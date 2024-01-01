import { EventScroll } from "@/extras/extra-vue-ui/touch/events.js";

export class Scale {
  uuid_;
  wasm_;
  transformation_;
  element_;

  constructor(wasm, uuid) {
    this.uuid_ = uuid;
    this.wasm_ = wasm;
    this.transformation_ = null;
    this.element_        = null;
  }
  
  name() { return "Scale"; }

  scroll(evt, model) {
    console.log("scaling...");
    if (this.wasm_ === null) { throw new Error("Wasm is not specified"); }
    if (this.uuid_ === null) { throw new Error("Uuid is not specified"); }

    if (this.element_ === null) { this.element_ = model.get_element(this.uuid_); }
    if (this.element_ === null) { throw new Error(`Unable to retrieve element with uuid ${this.uuid_}`); }

    let reference_point = this.element_.transform_reference_point();
    let scaling_builder = this.wasm_.ScalingBuilder.default()
      .with_origin(reference_point[0], reference_point[1], reference_point[2]);
    if (evt.type === EventScroll.LEFT) {
      scaling_builder = scaling_builder.with_vector(1.0, 1.0 + evt.delta * 0.1, 1.0);
    } else if (evt.type === EventScroll.TOP) {
      scaling_builder = scaling_builder.with_vector(1.0 + evt.delta * 0.1, 1.0, 1.0);
    } else if (evt.type === EventScroll.BTMRIGHT) {
      scaling_builder = scaling_builder.with_vector(1.0, 1.0, 1.0 + evt.delta * 0.1);
    } else {
      throw new Error(`Scroll event type ${evt.type} is not supported.`);
    }
    let transformation = scaling_builder.build();

    this.transformation_ = (this.transformation_ === null) ? transformation : this.transformation_.combine(transformation);

    return model.update_element(this.element_.clone().with_scale(this.transformation_.clone()));
  }

  click(evt, model) {
    return model;
  }
}
