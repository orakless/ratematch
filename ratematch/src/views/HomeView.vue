<script setup lang="ts">
import { Communicator } from '@/common/Communicator';
import { Configurator } from '@/common/Configurator';
import { string_to_language, type Event, type MatchDescription, type Rating } from '@/common/entities';
import type { Page } from '@/common/response_entities';
import type { Option } from '@/common/types';
import EventListItem from '@/components/EventListItem.vue';
import RatingCard from '@/components/RatingCard.vue';
import { ArrowLeftIcon, CalendarIcon, ArrowRightIcon, ChatBubbleLeftIcon } from '@heroicons/vue/20/solid';
import { ref, watch, type Ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { onBeforeRouteLeave } from 'vue-router';

const events: Ref<Event[]> = ref([]);
const page = ref(1);
const page_total = ref(1);
const communicator = new Communicator();
const config = Configurator.getInstance();

const ratings: Ref<Rating[]> = ref([]);

const i18n = useI18n();

/**
* Refresh event list
*/
function obtainEvents() {
  communicator.get_event_page(page.value)
    .then((data: Page<Event>) => { events.value = data.items; page_total.value = data.page_total })
    .catch((error) => {
      console.error(error);
    });
}

function fetchLocalizedRatings() {
  const lang = string_to_language(i18n.locale.value.toUpperCase());

  communicator.get_global_ratings(1, lang)
    .then((rating_page: Page<Rating>) => ratings.value = rating_page.items)
}

const prevPage = () => { --page.value; obtainEvents(); }
const nextPage = () => { ++page.value; obtainEvents(); }

obtainEvents();
fetchLocalizedRatings();
const timer = setInterval(() => { obtainEvents(); fetchLocalizedRatings(); }, 5000);

watch((i18n.locale), fetchLocalizedRatings)

onBeforeRouteLeave(() => { clearInterval(timer) })

</script>

<template>
  <div
    class="max-w-7xl flex flex-col items-center gap-4 border-x h-full dark:border-secondary-dark not-dark:border-secondary-light">
    <div class="p-24 w-full gap-1 flex flex-col items-center bg-position-[center_70%] bg-[url('/assets/hangman.webp')]">
      <h3 class="not-dark:bg-white px-2 dark:bg-black italic text-2xl font-bold">{{ $t("message.welcomeTitle") }}</h3>
      <p class="not-dark:bg-white px-2 dark:bg-black">{{ $t("message.welcomeMessage") }}</p>
    </div>
    <div class="max-w-7xl w-full p-4 pt-0">
      <h2 class="text-2xl pb-2 font-medium">
        <CalendarIcon class="size-5 inline" /> {{ $t("message.eventListTitle") }}
      </h2>
      <div class="relative flex flex-row items-center justify-center">
        <button @click="prevPage" :disabled="page == 1"
          class="h-full transition-all not-disabled:hover:scale-110 disabled:opacity-50 cursor-pointer not-dark:bg-white dark:bg-bg-dark p-1 rounded-lg border dark:border-secondary-dark not-dark:border-secondary-light">
          <ArrowLeftIcon class="size-6" />
        </button>
        <ul class="w-full grid grid-cols-1 md:grid-cols-2 gap-2 p-2">
          <EventListItem v-for="event in events" :key="event.id" :event="event"></EventListItem>
        </ul>
        <button @click="nextPage" :disabled="page == page_total"
          class="h-full transition-all not-disabled:hover:scale-110 disabled:opacity-50 cursor-pointer not-dark:bg-white dark:bg-bg-dark p-1 rounded-lg border dark:border-secondary-dark not-dark:border-secondary-light">
          <ArrowRightIcon class=" size-6" />
        </button>
      </div>
    </div>
    <div class="w-full overflow-y-visible pl-4">
      <h2 class="transition-all dark:bg-bg-darker not-dark:bg-bg-light text-2xl pb-2 font-medium shrink-0 grow-0">
        <ChatBubbleLeftIcon class="size-5 inline" /> {{ $t("message.ratingTitle") }}
      </h2>
      <ul dir="ltr"
        class="snap-x snap-proximity w-full max-w-full flex flex-row gap-2 overflow-y-visible overflow-x-auto">
        <RatingCard v-for="rating in ratings" :key="rating.id" :match_desc="undefined" :rating="rating" />
      </ul>
    </div>
  </div>
</template>
