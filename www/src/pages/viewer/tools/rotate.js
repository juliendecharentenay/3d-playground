import { EventScroll } from "@/extras/extra-vue-ui/touch/events.js";

export class Rotate {
  uuid_;
  wasm_;

  constructor(wasm, uuid) {
    this.uuid_ = uuid;
    this.wasm_ = wasm;
    this.transformation_  = null;
    this.element_         = null;
    this.reference_point_ = null;
    this.vector_          = [0.0, 0.0, 0.0];
  }
  
  name() { return "Rotate"; }

  scroll(evt, model) {
    if (this.wasm_ === null) { throw new Error("Wasm is not specified"); }
    if (this.uuid_ === null) { throw new Error("Uuid is not specified"); }

    if (this.element_ === null) { this.element_ = model.get_element(this.uuid_); }
    if (this.element_ === null) { throw new Error(`Unable to retrieve element with uuid ${this.uuid_}`); }

    if (this.reference_point_ === null) { this.reference_point_ = this.element_.transform_reference_point(); }

    const delta = evt.delta/100.0 * Math.PI;
    if (evt.type === EventScroll.LEFT) {
      this.vector_[1] += delta;
    } else if (evt.type === EventScroll.TOP) {
      this.vector_[2] += delta;
    } else if (evt.type === EventScroll.BTMRIGHT) {
      this.vector_[0] += delta;
    } else {
      throw new Error(`Scroll event type ${evt.type} is not supported.`);
    }

    return model
    .update_element(
      this.element_
      .clone()
      .with_rotate(
        this.wasm_.RotationBuilder.default()
        .with_origin(this.reference_point_[0], this.reference_point_[1], this.reference_point_[2])
        .with_vector(this.vector_[0], this.vector_[1], this.vector_[2])
        .build()
      )
    );
  }

  click(evt, model) {
    return model;
  }
}
