import { EventScroll } from "@/extras/extra-vue-ui/touch/events.js";

export class Pan {
  uuid_;
  wasm_;
  translation_;
  element_;

  constructor(wasm, uuid) {
    this.uuid_ = uuid;
    this.wasm_ = wasm;
    this.translation_ = null;
    this.element_ = null;
  }
  
  name() { return "Pan"; }

  scroll(evt, model) {
    if (this.wasm_ === null) { throw new Error("Wasm is not specified"); }
    if (this.uuid_ === null) { throw new Error("Uuid is not specified"); }

    if (this.element_ === null) { this.element_ = model.get_element(this.uuid_); }
    if (this.element_ === null) { throw new Error(`Unable to retrieve element with uuid ${this.uuid_}`); }

    let translation = null;
    if (evt.type === EventScroll.LEFT) {
      translation = this.wasm_.Translation.new(0.0, evt.delta * 0.1, 0.0);
    } else if (evt.type === EventScroll.TOP) {
      translation = this.wasm_.Translation.new(evt.delta * 0.1, 0.0, 0.0);
    } else if (evt.type === EventScroll.BTMRIGHT) {
      translation = this.wasm_.Translation.new(0.0, 0.0, evt.delta * 0.1);
    } else {
      throw new Error(`Scroll event type ${evt.type} is not supported.`);
    }

    this.translation_ = (this.translation_ === null) ? translation : this.translation_.combine(translation);

    return model.update_element(this.element_.clone().with_translate(this.translation_.clone()));
  }

  click(evt, model) {
    return model;
  }
}
