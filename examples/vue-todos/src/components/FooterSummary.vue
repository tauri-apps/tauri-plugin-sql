<script setup lang="ts">
import { ref } from 'vue';
import type { PropType } from 'vue';
import { useStore } from '../stores/todos';
import type { TodoFilter } from './ListTodos.vue';
const s = useStore();
defineProps({
  filter: { type: String as PropType<TodoFilter> }
});
</script>

<template>
  <div class="font-light w-full grid text-gray-400 grid-cols-12 content-between">
    <div class="flex font-light col-span-3">
      <span v-if="s.incomplete.length !== 0">
        {{ s.incomplete.length }} item{{ s.incomplete.length === 1 ? '' : 's' }} left
      </span>
      <span class="italic" v-else> no todo items </span>
    </div>
    <div class="flex flex-row space-x-2 col-span-6 filters justify-center">
      <div
        :class="$props.filter === 'all' ? 'filter-active' : 'filter-inactive'"
        @click="$emit('update:filter', 'all')"
      >
        All
      </div>
      <div
        :class="$props.filter === 'incomplete' ? 'filter-active' : 'cursor-pointer'"
        @click="$emit('update:filter', 'incomplete')"
      >
        Incomplete
      </div>
      <div
        :class="$props.filter === 'completed' ? 'filter-active' : 'filter-inactive'"
        @click="$emit('update:filter', 'completed')"
      >
        Completed
      </div>
    </div>
    <div class="flex-grow text-right col-span-3 block">
      <i-mdi:database-check-outline
        v-if="s.ready"
        class="text-green-700"
        :title="s.dbConnectionString"
        alt="connected to db successfully"
      />
      <i-mdi:database-remove-outline
        class="text-red-600"
        v-else
        :title="s.dbError"
        :alt="s.dbError"
      />
    </div>
  </div>
</template>

<style lang="postcss" scoped>
.filter-active {
  @apply cursor-default font-medium underline;
}

.filter-inactive {
  @apply cursor-pointer font-light;
}
</style>
