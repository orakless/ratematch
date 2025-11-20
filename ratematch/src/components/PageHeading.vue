<script setup lang="ts">
import { Configurator } from '@/common/Configurator';
import { reactive } from 'vue';

const props = defineProps<{
  title: string,
  subtitle: string,
  date: Date,
  average_score: number
}>()

const config = reactive(Configurator.getInstance());

</script>

<template>
  <div class="bg-[url('/assets/hangman.webp')] bg-cover bg-no-repeat bg-position-[center_70%]">
    <div :class="config.theme"
      class="transition-all flex md:px-16 flex-row justify-between pt-56 p-4 pb-4 bg-linear-to-b from-transparent not-dark:to-bg-light dark:to-bg-darker to-80%">
      <div>
        <h2 class="text-3xl">{{ title }}</h2>
        <p>{{ subtitle }}</p>
        <p>{{ date.toLocaleDateString() }}</p>
      </div>
      <div class="flex flex-col items-center justify-center">
        <p>{{ $t('message.averageTitle') }}</p>
        <p class="text-5xl">{{ (average_score == -1) ? "--" : average_score.toFixed(2) }}*</p>
      </div>
    </div>
  </div>
</template>
