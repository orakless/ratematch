<script setup lang="ts">
import type { Event } from '@/common/entities';
import { CalendarDaysIcon, TicketIcon } from '@heroicons/vue/16/solid';
import { useRouter } from 'vue-router';

const router = useRouter();

defineProps<{
  event: Event
}>()

const go = (url: string) => router.push(url)

</script>

<template>
  <li @click="go(`/event/${event.id}`)"
    class="p-2 shadow-sm transition-all hover:scale-105 dark:bg-bg-dark not-dark:bg-white border cursor-pointer rounded-lg dark:border-secondary-dark not-dark:border-secondary-light">
    <span class="text-lg font-medium">{{ event.name }}</span>
    <div class="text-sm">
      <span>
        <TicketIcon class="size-4 inline" />
        {{ event.promotion }}
      </span> · <span>
        <CalendarDaysIcon class="size-4 inline" /> {{ event.date }}
      </span>
    </div>
  </li>
</template>
