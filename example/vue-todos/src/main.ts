import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './App.vue';
import 'virtual:windi.css';
import 'virtual:windi-devtools';

const app = createApp(App);
app.use(createPinia());
app.mount('#app');
