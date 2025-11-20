<script setup lang="ts">
import { Configurator } from '@/common/Configurator';
import { Language, language_to_string } from '@/common/entities';
import { MoonIcon, SunIcon } from '@heroicons/vue/24/outline';
import { reactive } from 'vue';
import { useI18n } from 'vue-i18n';

const i18n = useI18n();

const config = reactive(Configurator.getInstance());

function toggleLanguage() {
  config.language = (config.language == Language.French) ? Language.English : Language.French;
  i18n.locale.value = language_to_string(config.language).toLowerCase();
}

function toggleStorage() {
  config.theme = (config.theme == "light") ? "dark" : "light";
}
</script>

<template>
  <div :class="config.theme"
    class="shadow-sm sticky top-0 flex flex-row items-center justify-center max-w-full w-full border-b dark:bg-bg-dark not-dark:bg-white not-dark:border-b-secondary-light dark:border-b-secondary-dark">
    <nav :class="config.theme" class="transition-all items-center pl-4 flex w-full max-w-7xl flex-row gap-4 p-2">
      <h1 :class="config.theme" class="transition-all italic font-bold text-xl">
        <RouterLink to="/">ratematch</RouterLink>
      </h1>
      <div :class="config.theme" class="transition-all flex flex-row w-full justify-end gap-2">
        <button @click="toggleLanguage"
          class="cursor-pointer shadow-sm border dark:border-secondary-dark not-dark:border-secondary-light px-2 p-1 rounded-sm">
          {{ $t("message.language_main") }}
        </button>
        <button @click="toggleStorage"
          class="cursor-pointer shadow-sm border dark:border-secondary-dark not-dark:border-secondary-light p-1 rounded-sm">
          <MoonIcon class="transition-all size-8" v-show="config.theme == 'dark'" />
          <SunIcon class="transition-all size-8" v-show="config.theme == 'light'" />
        </button>
      </div>
    </nav>
  </div>
</template>
