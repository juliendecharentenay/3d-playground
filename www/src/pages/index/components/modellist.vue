<template>
<div>
  <div v-if="models === null">Loading models</div>
  <div v-if="models !== null && models.length > 0">
  <TableComponent>
    <template #thead>
      <TableColumnHeader>Name</TableColumnHeader>
      <TableColumnHeader></TableColumnHeader>
    </template>
    <template #tbody>
      <tr v-for="m in models"
        :key="m.id()">
        <TableEntry>{{ m.name() }}</TableEntry>
        <TableEntry>
        <a :href="`/viewer.html?id=${m.id()}`" class="text-sm font-semibold text-indigo-600 hover:text-indigo-500">Open</a>
        </TableEntry>
      </tr>
    </template>
  </TableComponent>
  </div>
</div>
</template>
<script>
import { useComponentError } from "@/extras/extra-vue-ui/composable/error.js";

import TableComponent from "@/extras/extra-vue-ui/tables/tablecomponent.vue";
import TableColumnHeader from "@/extras/extra-vue-ui/tables/tablecolumnheader.vue";
import TableEntry from "@/extras/extra-vue-ui/tables/tableentry.vue";

import { get_list_models } from "@/js/storage.js";

export default {
  name: "ModelList",
  inject: [ 'wasm' ],
  setup: function(props, context) {
    const { on_error, catcher } = useComponentError(context);
    return { on_error, catcher };
  },
  components: {
    TableComponent,
    TableColumnHeader,
    TableEntry,
  },
  emits: [ 'error' ],
  data() {
    return {
      models: null,
    };
  },
  mounted: function() {
    this.catcher('mounted', 
    () => {
    console.log("List models");
      this.models = get_list_models(this.wasm);
    console.log("After List models");
    });
  },

}
</script>
