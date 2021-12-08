<script setup lang="ts">
import type { PropType } from 'vue';
import type { Todo, uuid } from 'src/models/Todo';
import { useStore } from '../stores/todos';

const { markFinished, markIncomplete, remove } = useStore();

const props = defineProps({
  todo: { type: Object as PropType<Todo>, required: true },
  editing: { type: String as PropType<uuid>, required: false }
});
const style = (t: Todo) => `${t.completed ? '' : ''} `;
</script>

<template>
  <li class="flex flex-row items-center min-w-64 hover:bg-gray-50 px-2 -mx-2 cursor-default py-1">
    <div
      class="checkbox flex flex-grow-0 mr-4"
      :class="props.todo.completed ? 'fill-gray-100' : ''"
      @click="() => (props.todo.completed ? markIncomplete(props.todo) : markFinished(props.todo))"
    >
      <input type="checkbox" :checked="props.todo.completed" />
    </div>
    <div class="task flex flex-grow">
      <slot>
        {{ props.todo.title }}
      </slot>
    </div>
    <div
      class="remove flex px-2 text-gray-200 dark:text-gray-700 hover:text-gray-400 dark:hover:text-gray-600 cursor-pointer rounded"
      @click="remove(props.todo.id)"
    >
      <i-ic:round-cancel />
    </div>
  </li>
</template>
