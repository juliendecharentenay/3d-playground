<template>
<div>
  <div v-if="models === null">Loading models</div>
  <div v-if="models !== null && models.length > 0">
  <TableComponent>
    <template #thead>
      <TableColumnHeader>Name</TableColumnHeader>
      <TableColumnHeader>Modified</TableColumnHeader>
      <TableColumnHeader>Created</TableColumnHeader>
      <TableColumnHeader></TableColumnHeader>
    </template>
    <template #tbody>
      <tr v-for="m in models"
        :key="m.id()">
        <TableEntry>{{ m.name() }}</TableEntry>
        <TableEntry>{{ m.last_modified() }}</TableEntry>
        <TableEntry>{{ m.created() }}</TableEntry>
        <TableEntry>
          <div class="flex flex-row gap-x-1 items-center">
            <a :href="`/viewer.html?id=${m.id()}`" class="text-sm font-semibold text-indigo-600 hover:text-indigo-500"><EyeIcon class="h-4 w-5" /></a>
            <div class="cursor-pointer text-indigo-600 hover:text-indigo-500" @click="() => {delete_model(m);}"><TrashIcon class="h-4 w-5" /></div>
          </div>
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

import { get_list_models, delete_model } from "@/js/storage.js";

import { TrashIcon, EyeIcon,
  } from "@heroicons/vue/24/outline";

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
    TrashIcon, EyeIcon,
  },
  emits: [ 'error' ],
  data() {
    return {
      models: null,
    };
  },
  mounted: function() {
    this.catcher('mounted', () => { this.load_models(); });
  },
  computed: {
    models2: function() {
      // if (this.models_ === null) { return null; }
      this.catcher('models',
      () => {
        return this.models_.toSorted((a, b) => a.last_modified().localeCompare(b.last_modified()));
      });
      return [];
    },
  },
  methods: {
    load_models: function() {
      this.catcher('delete_model',
      () => {
        this.models = get_list_models(this.wasm);
        this.models.sort((a,b) => a.last_modified().localeCompare(b.last_modified()));
      });
    },
    delete_model: function(m) {
      this.catcher('delete_model',
      () => {
        delete_model(this.wasm, m);
        this.load_models();
      });
    },
  },
}
</script>
