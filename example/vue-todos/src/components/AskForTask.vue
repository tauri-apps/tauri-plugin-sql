<script setup lang="ts">
import { ref } from 'vue';
import type {} from 'vue';
import { useStore } from '../stores/todos';
const { add } = useStore();

const title = ref('');
</script>

<template>
  <form
    class="new-task relative inline-block"
    action="new-task"
    @submit.prevent="
      add(title);
      title = '';
    "
  >
    <input
      v-model="title"
      class="w-100 p-4 bg-gray-50 dark:text-gray-800 rounded-full"
      type="text"
      name="title"
      autofocus
      autocomplete="off"
      placeholder="What do you need to do?"
    />
    <button
      class="absolute btn right-0 top-0 bottom-0 mr-3 place-items-center fill-white dark:fill-gray-200 flex"
      type="submit"
      :disabled="title.length < 1"
    >
      <i-carbon:add-filled
        class="text-gray-300 hover:text-gray-700 dark:hover:text-gray-600"
        :class="title.length > 1 ? 'opacity-100' : 'opacity-0'"
      />
    </button>
  </form>
</template>
