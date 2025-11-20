<script setup lang="ts">
import { Communicator } from '@/common/Communicator';
import { Language, language_to_string } from '@/common/entities';
import { ref, type Ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';

const props = defineProps<{
  match_id: string
}>()
const router = useRouter();
const i18n = useI18n();

const username: Ref<string> = ref("");
const language: Ref<string> = ref(i18n.locale.value.toUpperCase());
const score: Ref<number> = ref(0);
const review: Ref<string> = ref("");

async function submit() {
  const new_rating = {
    match_id: parseInt(props.match_id),
    username: username.value,
    language_code: language.value,
    score: score.value,
    opinion: (review.value == "") ? undefined : review.value
  }

  const res = await (new Communicator()).add_match_rating(new_rating);
  if (!res)
    alert(i18n.t('message.newRatingError'));

  router.back();
}

console.log(i18n.availableLocales.map((locale) => locale.toUpperCase()))

</script>

<template>
  <div
    class="h-full flex max-w-7xl w-full border-x dark:border-secondary-dark not-dark:border-secondary-light flex-col items-center justify-center">
    <form @submit.prevent="submit" class="flex flex-col max-w-2xl w-full">
      <h2 class="text-2xl mb-2">{{ $t("message.formTitle") }}</h2>
      <section class="flex flex-row justify-between">
        <label for="username">
          {{ $t("message.formUsername") }}
          <input id="username"
            class="mb-2 px-2 py-1 rounded-md border dark:border-secondary-dark dark:bg-bg-dark not-dark:border-secondary-light not-dark:bg-white"
            v-model="username">
        </label>

        <label for="language">
          {{ $t("message.formLanguage") }}
          <select id="language"
            class="mb-2 px-2 py-1 rounded-md border dark:border-secondary-dark dark:bg-bg-dark not-dark:border-secondary-light not-dark:bg-white"
            v-model="language">
            <option v-for="language in i18n.availableLocales.map((locale) => locale.toUpperCase())">{{
              language }}
            </option>
          </select>
        </label>
        <label for="score">
          {{ $t("message.formScore") }}
          <input id="score" type="number" min="0.00" max="5.00" step="0.25"
            class="mb-2 px-2 py-1 rounded-md border dark:border-secondary-dark dark:bg-bg-dark not-dark:border-secondary-light not-dark:bg-white"
            v-model="score"></input>
        </label>
      </section>
      <label for="review">
        {{ $t("message.formReview") }}
      </label>
      <textarea id="review"
        class="mb-2 px-2 py-1 rounded-md border dark:border-secondary-dark dark:bg-bg-dark not-dark:border-secondary-light not-dark:bg-white"
        v-model="review"></textarea>
      <button
        class="px-2 py-1 dark:bg-bg-dark not-dark:bg-white rounded-md border dark:border-secondary-dark not-dark:border-secondary-light cursor-pointer">Submit</button>
    </form>
  </div>
</template>
