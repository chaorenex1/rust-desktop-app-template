import { createApp } from 'vue';
import { createPinia } from 'pinia';
import ElementPlus from 'element-plus';
import * as ElementPlusIconsVue from '@element-plus/icons-vue';
import 'element-plus/dist/index.css';
import './styles/main.css';
import App from './App.vue';
import router from './router';

// Fix passive event listener warning for wheel events
if (typeof window !== 'undefined') {
  const addEventListenerOriginal = EventTarget.prototype.addEventListener;
  EventTarget.prototype.addEventListener = function(type: string, listener: any, options?: any) {
    const isTouchOrWheel = type === 'wheel' || type === 'mousewheel' || type === 'touchstart' || type === 'touchmove';
    
    if (isTouchOrWheel && typeof options === 'boolean') {
      options = { passive: true, capture: options };
    } else if (isTouchOrWheel && typeof options === 'object' && options.passive === undefined) {
      options = { ...options, passive: true };
    } else if (isTouchOrWheel && !options) {
      options = { passive: true };
    }
    
    return addEventListenerOriginal.call(this, type, listener, options);
  };
}

// Create Vue app
const app = createApp(App);

// Register Element Plus icons
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}

// Use plugins
app.use(createPinia());
app.use(ElementPlus);
app.use(router);

// Mount app
app.mount('#app');
