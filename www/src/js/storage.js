
function storage() {
  return window.localStorage;
}

function update(wasm) {
  var json = storage().getItem("model");
  if (json !== null) {
    let m = wasm.ModelWasmed.from_json(json);
    storage().removeItem("model");
    save_model(m);
  }
}

export function save_model(model) {
  storage().setItem(JSON.stringify({"type": "model", "id": model.id()}), model.to_json());
  storage().setItem(JSON.stringify({"type": "model_info", "id": model.id()}), model.strip().to_json());
}

export function read_model(wasm, id) {
  update(wasm);
  let store = storage();
  for (let i = 0; i < store.length; i ++) {
    let k = JSON.parse(store.key(i));
    if (k.type === "model" && k.id === id) {
      return wasm.ModelWasmed.from_json(store.getItem(store.key(i)));
    }
  }
  return null;
}

export function get_list_models(wasm) {
  update(wasm);
  let store = storage(); let result = [];
  for (let i = 0; i < store.length; i ++) {
    let k = JSON.parse(store.key(i));
    if (k.type === "model_info") {
      result.push(wasm.ModelWasmed.from_json(store.getItem(store.key(i))));
    }
  }
  return result;
}

