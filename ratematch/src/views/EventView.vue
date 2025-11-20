<script setup lang="ts">
import { Communicator } from '@/common/Communicator';
import { Configurator } from '@/common/Configurator';
import { Language, language_to_string, string_to_language, type Event, type Match, type MatchDescription, type Rating } from '@/common/entities';
import type { Option } from '@/common/types';
import MatchCard from '@/components/MatchCard.vue';
import PageHeading from '@/components/PageHeading.vue';
import RatingCard from '@/components/RatingCard.vue';
import { ChatBubbleLeftIcon, TrophyIcon } from '@heroicons/vue/20/solid';
import { reactive, ref, watch, type Ref } from 'vue';
import { useI18n } from 'vue-i18n';

const props = defineProps<{
  event_id: string
}>();

const config = reactive(Configurator.getInstance());
const communicator = new Communicator();
const i18n = useI18n();

const data_fetched = ref(false);

const event_id = ref(parseInt(props.event_id));

const event: Ref<Option<Event>> = ref();
const card: Ref<Match[]> = ref([]);
const match_desc: Map<number, MatchDescription> = reactive(new Map());
const ratings: Ref<Rating[]> = ref([]);
const average_score: Ref<number> = ref(-1000);

function fetchEventAverageRating() {
  communicator.get_average_score_for_event(event_id.value)
    .then(
      (avg_score) => average_score.value = avg_score
    )
}

function fetchLocalizedDescription(lang: Language) {
  if (card.value != undefined) {
    for (let match of card.value) {
      communicator.get_match_localized_description(match.id, lang)
        .then((desc) => { match_desc.set(match.id, desc) })
        .catch((err) => { console.error(err) }
        );
    }
  }
}

function fetchLocalizedRatings(lang: Language) {
  if (card.value != undefined) {
    communicator.get_event_ratings(event_id.value, 1, lang)
      .then((rating_page) => ratings.value = rating_page.items)
  }
}

function fetchLocalizedData() {
  const lang = string_to_language(i18n.locale.value.toUpperCase());

  fetchLocalizedDescription(lang);
  fetchLocalizedRatings(lang);
}

function fetch_events() {
  communicator.get_event_informations(event_id.value)
    .then((res_event) => {
      event.value = res_event;
      communicator.get_event_card(event_id.value)
        .then((res_card) => {
          card.value = res_card;
          data_fetched.value = true;
          fetchLocalizedData();
        })
    })
    .catch((err) => { console.error(err) });
}

function get_description_or_default(match_id: number): MatchDescription {
  const desc: Option<MatchDescription> = match_desc.get(match_id)

  if (desc == undefined) {
    return { match_id: match_id, description: "" } as MatchDescription;
  } else return desc;
}

fetch_events();
fetchEventAverageRating();

watch((i18n.locale), fetchLocalizedData)
</script>

<template>
  <div :class="config.theme" v-if="data_fetched"
    class="size-full max-h-full overflow-hidden max-w-7xl border-x dark:border-secondary-dark not-dark:border-secondary-light">
    <PageHeading :title="event.name" :subtitle="event.promotion" :date="new Date(event.date)"
      :average_score="average_score" />
    <div class="w-full flex flex-col gap-8 pl-4">

      <!-- Match list --->
      <div class="w-full">
        <h2 :class="config.theme"
          class="transition-all sticky top-0 dark:bg-bg-darker not-dark:bg-bg-light text-2xl pb-2 font-medium shrink-0 grow-0">
          <TrophyIcon class="size-5 inline" /> {{ $t("message.cardTitle") }}
        </h2>
        <ul dir="ltr"
          class="snap-x snap-mandatory w-full max-w-full flex flex-row gap-2 overflow-y-hidden overflow-x-auto">
          <MatchCard v-for="match in card" :key="match.id" :match="match"
            :match_desc="get_description_or_default(match.id)" />
        </ul>
      </div>

      <!-- Ratings list --->
      <div class="w-full overflow-y-visible">
        <h2 :class="config.theme"
          class="transition-all sticky top-0 dark:bg-bg-darker not-dark:bg-bg-light text-2xl pb-2 font-medium shrink-0 grow-0">
          <ChatBubbleLeftIcon class="size-5 inline" /> {{ $t("message.ratingTitle") }}
        </h2>
        <ul dir="ltr"
          class="snap-x snap-proximity w-full max-w-full flex flex-row gap-2 overflow-y-visible overflow-x-auto">
          <RatingCard v-for="rating in ratings" :key="rating.id"
            :match_desc="get_description_or_default(rating.match_id)" :rating="rating" />
        </ul>
      </div>

    </div>
  </div>
  <div v-else class="size-full flex flex-col items-center justify-center">
    <p>Fetching data...</p>
  </div>
</template>
