<script setup lang="ts">
import { ref } from 'vue';
import type {} from 'vue';
import { useStore } from '../stores/todos';
const s = useStore();
</script>

<template>
  <div class="flex flex-row text-gray-400 font-light">
    <div class="flex font-light flex-grow">
      <span v-if="s.incomplete.length !== 0">
        {{ s.incomplete.length }} item{{ s.incomplete.length === 1 ? '' : 's' }} left
      </span>
      <span class="italic" v-else> no items in your TODO list </span>
    </div>
    <div class="flex">
      <i-mdi:database-check-outline v-if="s.ready" class="text-green-700" />
      <i-mdi:database-remove-outline class="text-red-600" v-else />
    </div>
  </div>
  <div v-if="s.dbError" class="text-red-500">{{ s.dbError }}</div>
</template>
