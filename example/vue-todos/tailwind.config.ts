import { defineConfig } from 'windicss/helpers';

export default defineConfig({
  plugins: [
    require('windicss/plugin/filters'),
    require('windicss/plugin/forms'),
    require('windicss/plugin/aspect-ratio'),
    require('windicss/plugin/line-clamp')
  ]
});
