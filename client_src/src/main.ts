import "./styles/main.scss";
import "bootstrap";
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import clickablefield from "./plugins/clickablefield";

import App from './App.vue'
import router from './router'

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.use(clickablefield, {
    cssClass: 'clickable-field',
    inputDefaultCssClass: 'form-control',
    errorCssClass: 'error-msg-control'
});

app.mount('#app');