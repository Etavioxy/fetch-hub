import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";
import { useConfigStore } from './configStore';

const app = createApp(App);

// Load config groups before mounting the app
const { loadConfigGroups } = useConfigStore();
loadConfigGroups().then(() => {
  app.mount('#app');
});