import EventView from '@/views/EventView.vue'
import HomeView from '@/views/HomeView.vue'
import MatchView from '@/views/MatchView.vue'
import RatingView from '@/views/RatingView.vue';
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/event/:event_id',
      name: 'event',
      component: EventView,
      props: true
    },
    {
      path: '/match/:match_id',
      name: 'match',
      component: MatchView,
      props: true
    },
    {
      path: '/match/:match_id/new_rating',
      name: 'new rating',
      component: RatingView,
      props: true
    }
  ],
});

export default router;
