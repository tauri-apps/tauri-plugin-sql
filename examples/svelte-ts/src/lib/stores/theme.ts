import { writable } from 'svelte/store';

type Theme = 'dark' | 'light';

const createThemeStore = () => {
  let theme = localStorage.getItem('theme') as Theme | null;
  if (!theme) {
    theme = window.matchMedia('(prefers-color-scheme: dark)').matches
      ? 'dark'
      : 'light';
    localStorage.setItem('theme', theme);
  }
  const { subscribe, update, set } = writable(theme);

  return {
    subscribe,
    toggleTheme: () =>
      update(currentTheme => {
        const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
        console.log('here');
        localStorage.setItem('theme', newTheme);
        return newTheme;
      }),
  };
};

export const theme = createThemeStore();
