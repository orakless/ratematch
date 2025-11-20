import { useI18n } from "vue-i18n";
import { Language, language_to_string, string_to_language } from "./entities";

/**
 * Configurator is in charge of major configuration choices (theme, language),
 * and keep the values in memory for runtime usage, and is responsible of saving
 * the configuration in localStorage.
 */
export class Configurator {
  private static _instance: Configurator;

  private _theme: string = "light";
  private _language: Language = Language.French;

  static getInstance() {
    if (Configurator._instance == null || Configurator._instance == undefined) {
      Configurator._instance = new Configurator();
    }
    return Configurator._instance;
  }

  get theme() {
    return this._theme;
  }

  get language() {
    return this._language;
  }

  set theme(theme: string) {
    this._theme = theme;
    localStorage.setItem("theme", theme);
  }

  set language(language: Language) {
    this._language = language;
    localStorage.setItem("language", language_to_string(language));
  }

  /**
  * /!\ DO NOT USE CONSTRUCTOR! This is a singleton! You need to call getInstance().
  */
  constructor() {
    if (localStorage.getItem("theme")) {
      this.theme = localStorage.getItem("theme") as string;
    } else localStorage.setItem("theme", this.theme);

    if (localStorage.getItem("language")) {
      this.language = string_to_language(localStorage.getItem("language") as string);
    } else localStorage.setItem("language", language_to_string(this.language));
  }
}
