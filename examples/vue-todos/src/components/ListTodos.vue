<script setup lang="ts">
import type { Todo } from 'src/models/Todo';
import { computed, ref } from 'vue';
import type { PropType } from 'vue';
import { useStore } from '../stores/todos';

const s = useStore();

export type TodoFilter = 'all' | 'completed' | 'incomplete';
const filter = ref<TodoFilter>('all');

const items = computed(() => {
  return s[filter.value];
});
</script>

<template>
  <div class="rounded flex flex-col border-1 border-gray-400 min-h-90 w-full p-4 todos">
    <ul class="flex flex-col flex-grow w-full py-4">
      <todo-item v-for="todo in items" :key="todo.id" :todo="todo"> </todo-item>
    </ul>

    <footer-summary v-model:filter="filter" />
  </div>
</template>
