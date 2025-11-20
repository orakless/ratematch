<script setup lang="ts">
import { Communicator } from '@/common/Communicator';
import { string_to_language, type MatchDescription, type Rating } from '@/common/entities';
import type { Option } from '@/common/types';
import { ref, type Ref } from 'vue';

const communicator = new Communicator();

const props = defineProps<{
  match_desc: Option<MatchDescription>,
  rating: Rating,
}>();

const mdesc: Ref<MatchDescription> = ref({ "language_code": props.rating.language_code, "description": "", "match_id": -1, "id": -1 });

if (props.match_desc == undefined) {
  communicator.get_match_localized_description(props.rating.match_id, string_to_language(props.rating.language_code))
    .then((desc) => {
      mdesc.value = desc
    })
} else mdesc.value = props.match_desc;


</script>

<template>
  <li class="max-w-60 flex flex-col snap-start shrink-0 w-full shadow-sm border
      rounded-lg not-dark:border-secondary-light dark:border-secondary-dark overflow-hidden not-dark:bg-bg-lighter">
    <header
      class="max-h-16 h-16 flex flex-col justify-center text-lg pl-2 p-0.5 pb-0.5 border-b dark:border-secondary-dark not-dark:border-secondary-light not-dark:bg-white dark:bg-bg-dark">
      <p class="line-clamp-1 font-medium ">{{ rating.username }}</p>
      <RouterLink :to='`/match/${mdesc.match_id}`' class="line-clamp-1 text-sm">{{ mdesc.description }}</RouterLink>
    </header>
    <div class="grow flex flex-col p-2 justify-between">
      <p class="line-clamp-5"><span class="dark:text-yellow-300 not-dark:text-yellow-600">
          [{{ rating.score }}*]</span>
        {{ rating.opinion }}</p>
      <p class="text-end">{{ new Date(rating.publication_date).toLocaleString() }}</p>
    </div>
  </li>
</template>
