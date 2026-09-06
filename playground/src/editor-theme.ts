import { EditorView } from '@codemirror/view';
import { HighlightStyle, syntaxHighlighting } from '@codemirror/language';
import { tags } from '@lezer/highlight';

export type Theme = 'light' | 'dark';

// IntelliJ's current Light/Dark schemes, adapted to CodeMirror's Java syntax tags.
// https://github.com/JetBrains/intellij-community/tree/04b6fcde0aa6b01a4ccb536c137e8594274ab356/platform/platform-resources/src/themes/expUI
const palettes = {
  light: {
    background: '#ffffff',
    foreground: '#080808',
    caret: '#080808',
    activeLine: '#f5f8fe',
    selection: '#a6d2ff',
    gutter: '#aeb3c2',
    activeGutter: '#767a8a',
    keyword: '#0033b3',
    string: '#067d17',
    number: '#1750eb',
    comment: '#8c8c8c',
    docComment: '#8c8c8c',
    method: '#00627a',
    field: '#871094',
    annotation: '#9e880d',
    matchingBracket: '#93d9d9',
    error: '#bc2828',
    search: '#fff1b8',
  },
  dark: {
    background: '#1e1f22',
    foreground: '#bcbec4',
    caret: '#ced0d6',
    activeLine: '#26282e',
    selection: '#214283',
    gutter: '#4b5059',
    activeGutter: '#a1a3ab',
    keyword: '#cf8e6d',
    string: '#6aab73',
    number: '#2aacb8',
    comment: '#7a7e85',
    docComment: '#5f826b',
    method: '#56a8f5',
    field: '#c77dbb',
    annotation: '#b3ae60',
    matchingBracket: '#43454a',
    error: '#f75464',
    search: '#534d2a',
  },
};

export function editorTheme(theme: Theme) {
  const p = palettes[theme];
  return [
    EditorView.theme(
      {
        '&': { color: p.foreground, backgroundColor: p.background },
        '.cm-content': { caretColor: p.caret },
        '.cm-cursor, .cm-dropCursor': { borderLeftColor: p.caret },
        '&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection': {
          backgroundColor: p.selection,
        },
        '.cm-gutters': { backgroundColor: p.background, color: p.gutter, border: 'none' },
        '.cm-activeLine, .cm-activeLineGutter': { backgroundColor: p.activeLine },
        '.cm-activeLineGutter': { color: p.activeGutter },
        '&.cm-focused .cm-matchingBracket': { backgroundColor: p.matchingBracket },
        '.cm-searchMatch': { backgroundColor: p.search },
        '.cm-panels, .cm-tooltip': {
          color: 'var(--text)',
          backgroundColor: 'var(--surface)',
          borderColor: 'var(--border)',
        },
        '.cm-textfield, .cm-button': {
          color: 'var(--text)',
          background: 'var(--surface-2)',
          borderColor: 'var(--border)',
        },
        '.cm-tooltip-autocomplete > ul > li[aria-selected]': {
          color: p.foreground,
          backgroundColor: p.selection,
        },
        '.cm-foldPlaceholder': { color: p.comment, backgroundColor: p.activeLine, border: 'none' },
      },
      { dark: theme === 'dark' },
    ),
    syntaxHighlighting(
      HighlightStyle.define([
        {
          tag: [tags.keyword, tags.modifier, tags.bool, tags.null, tags.standard(tags.typeName)],
          color: p.keyword,
        },
        { tag: [tags.string, tags.character], color: p.string },
        { tag: tags.number, color: p.number },
        {
          tag: [tags.lineComment, tags.blockComment],
          color: p.comment,
          fontStyle: theme === 'light' ? 'italic' : 'normal',
        },
        { tag: tags.docComment, color: p.docComment, fontStyle: 'italic' },
        { tag: tags.function(tags.variableName), color: p.method },
        { tag: tags.propertyName, color: p.field },
        { tag: tags.annotation, color: p.annotation },
        {
          tag: [tags.variableName, tags.typeName, tags.operator, tags.punctuation],
          color: p.foreground,
        },
        { tag: tags.invalid, color: p.error },
      ]),
    ),
  ];
}
