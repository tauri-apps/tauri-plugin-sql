<script setup lang="ts">
import { UseDark } from '@vueuse/components';
import { onMounted } from 'vue';
import { useStore } from './stores/todos';

const s = useStore();

onMounted(async () => {
  try {
    await s.init();
  } catch (e) {
    console.log(`There was a problem initializing the store`);
  }
});
</script>

<template>
  <UseDark v-slot="{ isDark, toggleDark }">
    <div class="page absolute h-full w-full dark:bg-gray-900 dark:text-gray-400">
      <div class="todo-app w-full text-center place-content-center flex">
        <div class="flex px-2 md:px-4 flex-col space-y-16 w-xl md:w-3xl items-center mt-16">
          <h1 class="text-5xl text-blue-500 font-medium">TODO App</h1>
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
