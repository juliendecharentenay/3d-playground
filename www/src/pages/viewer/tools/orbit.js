import { EventScroll } from "@/extras/extra-vue-ui/touch/events.js";

export class Orbit {
  constructor() {}

  name() { return "Orbit"; }

  scroll(evt, model) {
    let camera = model.camera();
    if (evt.type === EventScroll.LEFT) {
      camera = camera.as_camera().orbit_around_target(0.0, evt.delta);

    } else if (evt.type === EventScroll.TOP) {
      camera = camera.as_camera().orbit_around_target(evt.delta, 0.0);

    } else if (evt.type === EventScroll.BTMRIGHT) {
      camera = camera.as_camera().zoom_front(evt.delta);

    } else {
      throw new(`Scroll event type ${evt.type} is not supported`);
    }
    return model.set_camera(camera);
  }
  
  click(evt, model) {
    return model;
  }
}

