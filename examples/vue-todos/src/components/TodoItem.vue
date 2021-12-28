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
  <li
    class="flex flex-row items-center min-w-64 rounded hover:bg-gray-50 dark:hover:bg-gray-800 px-2 -mx-2 cursor-default py-1"
  >
    <div
      class="checkbox flex flex-grow-0 mr-4"
      :class="props.todo.completed ? 'completed' : 'incomplete'"
      @click="() => (props.todo.completed ? markIncomplete(props.todo) : markFinished(props.todo))"
    >
      <input type="checkbox" :checked="props.todo.completed" />
    </div>
    <div class="task flex flex-grow" :class="props.todo.completed ? 'completed' : 'incomplete'">
      <slot>
        {{ props.todo.title }}
      </slot>
    </div>
    <div class="remove" @click="remove(props.todo.id)">
      <i-ic:round-cancel />
    </div>
  </li>
</template>

<style lang="postcss" scoped>
.completed {
  @apply text-gray-300 dark:text-gray-600;
}
.incomplete {
  @apply text-gray-600 dark:text-gray-300;
}
.remove {
  @apply flex pl-2 pr-0.5 text-gray-300 dark:text-gray-700 hover:text-gray-400 dark:hover:text-gray-400 cursor-pointer rounded;
}
</style>
