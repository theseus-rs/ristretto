import { java } from '@codemirror/lang-java';
import { StreamLanguage } from '@codemirror/language';
import { kotlin, scala } from '@codemirror/legacy-modes/mode/clike';
import { groovy } from '@codemirror/legacy-modes/mode/groovy';
import { clojure } from '@codemirror/legacy-modes/mode/clojure';
import type { Language } from '../shared/protocol';
import javaIcon from './icons/java.svg';
import kotlinIcon from './icons/kotlin.svg';
import groovyIcon from './icons/groovy.svg';
import scalaIcon from './icons/scala.svg';
import clojureIcon from './icons/clojure.svg';

export const languageIcons: Record<Language, string> = {
  java: javaIcon,
  kotlin: kotlinIcon,
  groovy: groovyIcon,
  scala: scalaIcon,
  clojure: clojureIcon,
};

export const languageNames: Record<Language, string> = {
  java: 'Java',
  kotlin: 'Kotlin',
  groovy: 'Groovy',
  scala: 'Scala',
  clojure: 'Clojure',
};
export const filenames: Record<Language, string> = {
  java: 'Main.java',
  kotlin: 'Main.kts',
  groovy: 'Main.groovy',
  scala: 'Main.sc',
  clojure: 'Main.clj',
};
export function languageSupport(language: Language) {
  return language === 'java'
    ? java()
    : StreamLanguage.define({ kotlin, groovy, scala, clojure }[language]);
}
