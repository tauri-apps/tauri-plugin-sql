
const daisyui = require('daisyui');

/** @type {import('tailwindcss').Config} */
const config = {
  content: ["./src/**/*.{html,js,svelte,ts}"],

daisyui: {
    themes: [
      {
        light: {
          ...require("daisyui/src/colors/themes")["[data-theme=light]"],
        },
        dark: {
          ...require("daisyui/src/colors/themes")["[data-theme=dark]"],
        }
      }
    ]
  },
  plugins: [daisyui]
};

module.exports = config;
