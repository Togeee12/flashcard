import Vue from 'vue'; // Import Vue
import App from './App.vue';
// import axios from 'axios'

Vue.config.productionTip = false;
// Rust api endpoint in localhost place
// axios.defaults.baseURL = 'https://localhost:8000';
new Vue({
    render: (h) => h(App),
}).$mount('#app');
