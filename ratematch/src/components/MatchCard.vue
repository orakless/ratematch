<script setup lang="ts">
import type { Match, MatchDescription } from '@/common/entities';
import { useRouter } from 'vue-router';

const router = useRouter();

const props = defineProps<{
  match: Match,
  match_desc: MatchDescription
}>();

const goToPage = () => router.push(`/match/${props.match.id}`)

</script>

<template>
  <li @click="goToPage" class="cursor-pointer max-w-60 snap-start shrink-0 w-full shadow-sm border 
          rounded-lg dark:border-secondary-dark overflow-hidden not-dark:bg-bg-lighter
          not-dark:border-secondary-light">
    <header
      class="max-h-16 h-full flex flex-col justify-center  text-lg pl-2 p-0.5 pb-0.5 border-b font-medium dark:border-secondary-dark not-dark:border-secondary-light not-dark:bg-white dark:bg-bg-dark">
      <p class="line-clamp-2">{{ match_desc.description }}</p>
    </header>
    <p class="p-2 line-clamp-3"> {{ match.workers }} </p>
  </li>
</template>
