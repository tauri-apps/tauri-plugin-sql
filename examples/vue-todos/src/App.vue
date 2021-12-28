<script setup lang="ts">
import { UseDark } from '@vueuse/components';
import { onMounted } from 'vue';
import { useStore } from './stores/todos';

const s = useStore();

onMounted(async () => {
  try {
    await s.initializeDbBackedStore();
  } catch (e) {
    console.log(`There was a problem initializing the database`, e);
  }
});
</script>

<template>
  <UseDark v-slot="{ isDark, toggleDark }">
    <div class="h-full w-full page absolute dark:bg-gray-900 dark:text-gray-400">
      <div class="flex text-center w-full todo-app place-content-center">
        <div
          class="flex flex-col space-y-8 mt-12 w-full px-2 items-center sm:w-xl md:px-4 md:w-3xl"
        >
          <h1 class="font-medium text-5xl text-blue-500">TODO App</h1>
          <div class="flex flex-row space-x-4 ml-8">
            <ask-for-task />
            <button v-if="isDark" @click="toggleDark()"><i-carbon:moon /></button>
            <button v-else @click="toggleDark()"><i-carbon:sun /></button>
          </div>
          <list-todos />
        </div>
      </div>
    </div>
  </UseDark>
</template>
