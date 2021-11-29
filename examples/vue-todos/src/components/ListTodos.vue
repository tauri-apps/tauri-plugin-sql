<script setup lang="ts">
import type { Todo } from 'src/models/Todo';
import { computed } from 'vue';
import type { PropType } from 'vue';
import { useStore } from '../stores/todos';

const s = useStore();

export type TodoFilter = 'all' | 'completed' | 'incomplete';

const props = defineProps({
  filter: { type: String as PropType<TodoFilter>, default: undefined }
});

const items = computed(() => {
  const filter = props.filter || 'all';
  return s[filter];
});
</script>

<template>
  <div class="todos border-1 border-gray-400 p-4 rounded flex flex-col w-full">
    <ul class="flex flex-col flex-grow py-4 w-full">
      <todo-item v-for="todo in items" :key="todo.id" :todo="todo">
        <span
          :class="
            todo.completed ? 'text-gray-300 dark:text-gray-700' : 'text-gray-800 dark:text-gray-300'
          "
          >{{ todo.title }}</span
        >
      </todo-item>
    </ul>

    <footer-summary />
  </div>
</template>
