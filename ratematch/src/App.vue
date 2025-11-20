<script setup lang="ts">
import { reactive } from 'vue';
import Header from './components/Header.vue';
import { Configurator } from './common/Configurator';
import { useI18n } from 'vue-i18n';
import { language_to_string } from './common/entities';

const config = reactive(Configurator.getInstance());
const i18n = useI18n();

i18n.locale.value = language_to_string(config.language).toLowerCase();
</script>

<template>
  <div :class="config.theme"
    class="transition-all overflow-y-hidden flex flex-col items-center size-full dark:text-main-dark not-dark:text-main-light not-dark:bg-bg-light dark:bg-bg-darker">
    <Header />
    <RouterView>
    </RouterView>
    <div
      class="p-2 dark:bg-bg-dark not-dark:bg-white w-full text-center border-t dark:border-secondary-dark not-dark:border-secondary-light">
      {{ $t('message.credits') }}
    </div>
  </div>
</template>

<style scoped></style>
