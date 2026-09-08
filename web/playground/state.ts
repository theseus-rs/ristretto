import {
  isJavaVersion,
  isLanguage,
  type JavaVersion,
  type Language,
  type ScalaVersion,
  type Target,
} from '../shared/protocol';

export type Draft = { source: string; className: string; example: string };
export type PlaygroundState = {
  language: Language;
  scalaVersion: ScalaVersion;
  javaVersion: JavaVersion;
  drafts: Partial<Record<Target, Draft>>;
};
export const storageKey = 'ristretto-playground-source-v2';
export const legacyStorageKey = 'ristretto-playground-source-v1';

/** Invalid or unavailable storage must never prevent editing. */
export function restoreState(raw: string | null, legacy: string | null): PlaygroundState {
  const result: PlaygroundState = {
    language: 'java',
    scalaVersion: '3',
    javaVersion: 25,
    drafts: {},
  };
  try {
    const saved = parse(raw);
    const old = parse(legacy);
    const version = saved?.javaVersion ?? old?.javaVersion;
    if (isJavaVersion(version)) result.javaVersion = version;
    if (isLanguage(saved?.language)) result.language = saved.language;
    if (saved?.scalaVersion === '2.13') result.scalaVersion = '2.13';
    for (const target of ['java', 'kotlin', 'groovy', 'scala2', 'scala3', 'clojure'] as const) {
      const draft = saved?.drafts?.[target] ?? (target === 'java' ? old : undefined);
      if (typeof draft?.source === 'string' && typeof draft?.className === 'string') {
        result.drafts[target] = {
          source: draft.source,
          className: draft.className,
          example: typeof draft.example === 'string' ? draft.example : 'hello',
        };
      }
    }
  } catch {
    /* Fall back to the initial examples. */
  }
  return result;
}

function parse(raw: string | null) {
  try {
    return JSON.parse(raw ?? 'null');
  } catch {
    return null;
  }
}
