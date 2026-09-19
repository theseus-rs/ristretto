import logo from '../../images/logo.svg';
import { siteHelp } from './site-help';

export function siteHeader(product: 'playground' | 'jshell') {
  return `<header class="site-header">
    <div class="brand"><a class="brand-home" href="${import.meta.env.BASE_URL}"><img src="${logo}" alt="" /><span>ristretto</span></a><span class="brand-divider" aria-hidden="true">/</span><select id="product" class="product-picker" aria-label="Switch playground"><option value="playground" ${product === 'playground' ? 'selected' : ''}>playground</option><option value="jshell" ${product === 'jshell' ? 'selected' : ''}>jshell</option></select></div>
    <p class="local-note"><span>Your code stays on your device.</span></p>
    <div class="header-actions">${siteHelp(product)}<button id="theme" class="theme-toggle" type="button" aria-label="Color theme"></button><a class="github-link" href="https://github.com/theseus-rs/ristretto" target="_blank" rel="noreferrer">View on GitHub <span aria-hidden="true">↗</span></a></div>
  </header>`;
}

export function initializeSiteHeader() {
  const product = document.querySelector<HTMLSelectElement>('#product')!;
  product.addEventListener('change', () => {
    location.assign(`${import.meta.env.BASE_URL}${product.value}/`);
  });
  const help = document.querySelector<HTMLDetailsElement>('.header-help')!;
  const helpToggle = help.querySelector('summary')!;
  const closeHelp = () => {
    help.open = false;
    helpToggle.focus();
  };
  help.querySelector<HTMLButtonElement>('#close-help')!.addEventListener('click', closeHelp);
  document.addEventListener('keydown', (event) => {
    if (event.key === 'Escape' && help.open) {
      event.preventDefault();
      closeHelp();
    }
  });
  document.addEventListener('pointerdown', (event) => {
    if (event.target instanceof Node && !help.contains(event.target)) help.open = false;
  });
}
