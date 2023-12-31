export class Rotate {
  uuid_;
  wasm_;

  constructor(wasm, uuid) {
    this.uuid_ = uuid;
    this.wasm_ = wasm;
  }
  
  name() { return "Rotate"; }

  scroll(evt, model) {
    return model;
  }

  click(evt, model) {
    return model;
  }
}
