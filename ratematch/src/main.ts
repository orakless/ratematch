import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createI18n } from 'vue-i18n'

const app = createApp(App);

// for internationalization
const i18n = createI18n({
  legacy: false,
  locale: 'fre',
  fallbackLocale: 'fre',
  messages: {
    eng: {
      message: {
        language_main: 'English',
        language_fre: 'French',
        language_eng: 'English',
        eventListTitle: 'Latest events',
        welcomeTitle: 'Welcome to ratematch',
        welcomeMessage: 'The wrestling opinion database',
        cardTitle: 'Event card',
        ratingTitle: 'Latest ratings',
        averageTitle: 'Average score',
        backToEvent: 'Back to event',
        noRatingMessageTitle: 'No rating for now.',
        noRatingMessageContent: 'You can add your own rating by clicking the "Add rating" button on top-right of your screen.',
        addRatingButton: 'Add a rating',
        credits: 'ratematch is a school project, made by Eva G.',
        formUsername: 'Username',
        formLanguage: 'Language',
        formScore: 'Score',
        formReview: 'Review (optional)',
        formTitle: 'Add a new rating'
      }
    },
    fre: {
      message: {
        language_main: 'Français',
        language_fre: 'Français',
        language_eng: 'Anglais',
        eventListTitle: 'Derniers événements',
        welcomeTitle: 'Bienvenue sur ratematch',
        welcomeMessage: 'La base de données des avis sur le catch',
        cardTitle: 'Carte de l\'événement',
        ratingTitle: 'Derniers avis',
        averageTitle: 'Score moyen',
        backToEvent: 'Revenir à l\'événement',
        noRatingMessageTitle: 'Pas d\'avis pour l\'instant',
        noRatingMessageContent: 'Vous pouvez ajouter votre propre avis en appuyant sur le bouton "Ajouter un avis" en haut à droite de votre écran.',
        addRatingButton: 'Ajouter un avis',
        credits: 'ratematch est un projet étudiant, développé par Eva G.',
        formUsername: 'Nom d\'utilisateur',
        formLanguage: 'Langue',
        formScore: 'Note',
        formReview: 'Critique (optionnelle)',
        formTitle: 'Ajouter un nouvel avis'
      }
    }
  }
});

app.use(router);
app.use(i18n);

app.mount('#app');
