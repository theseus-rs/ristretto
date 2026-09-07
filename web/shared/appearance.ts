import type { Theme } from './editor-theme';

const themePreferences = ['system', 'light', 'dark'] as const;
type ThemePreference = (typeof themePreferences)[number];
const themeLabels = { system: 'System', light: 'Light', dark: 'Dark' };
const themeIcons = {
  system: '<rect x="3" y="4" width="18" height="13" rx="2"/><path d="M8 21h8m-4-4v4"/>',
  light:
    '<circle cx="12" cy="12" r="4"/><path d="M12 2v2m0 16v2M2 12h2m16 0h2M5 5l1.5 1.5m11 11L19 19M5 19l1.5-1.5m11-11L19 5"/>',
  dark: '<path d="M20.9 13A9 9 0 0 1 11 3.1 9 9 0 1 0 20.9 13Z"/>',
};
const systemTheme = matchMedia('(prefers-color-scheme: dark)');
const themeKey = 'ristretto-playground-theme';
let themePreference: ThemePreference = 'system';
try {
  const saved = localStorage.getItem(themeKey);
  if (saved === 'light' || saved === 'dark') themePreference = saved;
} catch {
  /* Theme controls also work when browser storage is unavailable. */
}
export const selectedTheme = (): Theme =>
  themePreference === 'system' ? (systemTheme.matches ? 'dark' : 'light') : themePreference;

export function initializeAppearance(onChange: (theme: Theme) => void) {
  const themeButton = document.getElementById('theme') as HTMLButtonElement;
  function nextThemePreference(): ThemePreference {
    return themePreferences[
      (themePreferences.indexOf(themePreference) + 1) % themePreferences.length
    ];
  }
  function applyTheme() {
    const theme = selectedTheme();
    const label = `Color theme: ${themeLabels[themePreference]}. Switch to ${themeLabels[nextThemePreference()]}.`;
    themeButton.setAttribute('aria-label', label);
    themeButton.title = label;
    themeButton.innerHTML = `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${themeIcons[themePreference]}</svg>`;
    document.documentElement.dataset.theme = theme;
    document
      .querySelector('meta[name="theme-color"]')
      ?.setAttribute('content', theme === 'dark' ? '#1e1f22' : '#f5f7f4');
    onChange(theme);
  }
  themeButton.addEventListener('click', () => {
    themePreference = nextThemePreference();
    try {
      if (themePreference === 'system') localStorage.removeItem(themeKey);
      else localStorage.setItem(themeKey, themePreference);
    } catch {
      /* Storage is optional. */
    }
    applyTheme();
  });
  systemTheme.addEventListener('change', () => {
    if (themePreference === 'system') applyTheme();
  });
  window.addEventListener('storage', (event) => {
    if (event.key !== themeKey && event.key !== null) return;
    themePreference =
      event.newValue === 'light' || event.newValue === 'dark' ? event.newValue : 'system';
    applyTheme();
  });
  applyTheme();
}
