<script setup lang="ts">
import { Communicator } from '@/common/Communicator';
import { Configurator } from '@/common/Configurator';
import { string_to_language, type Event, type Match, type MatchDescription, type Rating } from '@/common/entities';
import type { Option } from '@/common/types';
import PageHeading from '@/components/PageHeading.vue';
import RatingCard from '@/components/RatingCard.vue';
import { ChatBubbleLeftIcon, PlusIcon } from '@heroicons/vue/20/solid';
import { reactive, ref, watch, type Ref } from 'vue';
import { useI18n } from 'vue-i18n';


const props = defineProps<{
  match_id: string
}>();

const config = reactive(Configurator.getInstance());
const match_id = ref(parseInt(props.match_id));
const i18n = useI18n();
const communicator = new Communicator();

const data_fetched = ref(false);

const match: Ref<Option<Match>> = ref();
const match_desc: Ref<Option<MatchDescription>> = ref();
const event: Ref<Option<Event>> = ref();

const average_score: Ref<number> = ref(-1);

const ratings: Ref<Rating[]> = ref([]);
const current_page: Ref<number> = ref(1);
const total_pages: Ref<number> = ref(1);

function fetchLocalizedDescription() {
  communicator.get_match_localized_description(match_id.value, string_to_language(i18n.locale.value.toUpperCase()))
    .then(
      (res_desc) => {
        match_desc.value = res_desc;
      }
    )
}

function fetchData() {
  communicator.get_match_informations(match_id.value)
    .then(
      (res_match) => {
        match.value = res_match;
        communicator.get_event_informations(res_match.event_id)
          .then(
            (res_event) => {
              event.value = res_event;
              communicator.get_match_localized_description(match_id.value, string_to_language(i18n.locale.value.toUpperCase()))
                .then(
                  (res_desc) => {
                    match_desc.value = res_desc;
                    data_fetched.value = true;
                  }
                )
            }
          );

      }
    )
}

function fetchAverageScore() {
  communicator.get_average_score_for_match(match_id.value)
    .then(
      (avg_score) => {
        average_score.value = avg_score;
      }
    )
}

function fetchRatings() {
  communicator.get_match_ratings(match_id.value, current_page.value, string_to_language(i18n.locale.value.toUpperCase()))
    .then(
      (ratings_page) => {
        total_pages.value = ratings_page.page_total;
        ratings.value = ratings_page.items;
      }
    )
}

fetchData();
fetchAverageScore();
fetchRatings();

watch(i18n.locale, () => { fetchRatings(); fetchLocalizedDescription(); });
</script>

<template>
  <div :class="config.theme" v-if="data_fetched"
    class="relative flex flex-col size-full max-h-full overflow-hidden max-w-7xl border-x dark:border-secondary-dark not-dark:border-secondary-light">
    <PageHeading :title="match_desc.description" :subtitle="match.workers" :date="new Date(event.date)"
      :average_score="average_score" />
    <RouterLink :to="`/event/${event?.id}`"
      class="absolute transition-all hover:scale-105 cursor-pointer top-4 left-4 dark:bg-bg-dark not-dark:bg-white px-4 py-2 rounded-md border dark:border-secondary-dark not-dark:border-secondary-light">
      {{ $t('message.backToEvent') }}
    </RouterLink>
    <RouterLink :to="`/match/${match_id}/new_rating`"
      class="absolute transition-all hover:scale-105 cursor-pointer top-4 right-4 dark:bg-bg-dark not-dark:bg-white pl-2 pr-4 py-2 rounded-md border dark:border-secondary-dark not-dark:border-secondary-light">
      <PlusIcon class="size-5 inline" /> {{ $t('message.addRatingButton') }}
    </RouterLink>
    <div v-if="ratings.length > 0" class="px-4">
      <h2 :class="config.theme"
        class="transition-all sticky top-0 dark:bg-bg-darker not-dark:bg-bg-light text-2xl pb-2 font-medium shrink-0 grow-0">
        <ChatBubbleLeftIcon class="size-5 inline" /> {{ $t("message.ratingTitle") }}
      </h2>
      <ul class="grid grid-cols-5">
        <RatingCard v-for="rating in ratings" :key="rating.id" :match_desc="match_desc" :rating="rating" />
      </ul>
    </div>
    <div class="flex flex-col text-center grow items-center justify-center" v-else>
      <h2 class="text-2xl">{{ $t("message.noRatingMessageTitle") }}</h2>
      <p>{{ $t("message.noRatingMessageContent") }}</p>
    </div>
  </div>
  <div v-else class="size-full flex flex-col items-center justify-center">
    <p>Fetching data...</p>
  </div>
</template>
