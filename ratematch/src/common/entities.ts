import type { Option } from "./types";

export enum Language {
  French,
  English
}

export function language_to_string(lang: Language): string {
  switch (lang) {
    case Language.French:
      return "FRE";
    case Language.English:
      return "ENG";
  }
}

export function string_to_language(str: string): Language {
  switch (str) {
    case "FRE":
      return Language.French;
    case "ENG":
    default:
      return Language.English;
  }
}

export interface Event {
  id: number;
  name: string;
  promotion: string;
  date: string;
}

export interface Match {
  id: number;
  event_id: number;
  workers: string;
}

export interface MatchDescription {
  id: number;
  match_id: number;
  description: string;
  language_code: string;
}

export interface Rating {
  id: number;
  match_id: number;
  language_code: string;
  username: string;
  score: number;
  publication_date: string;
  opinion: Option<string>;
}

export interface NewRating {
  match_id: number;
  language_code: string;
  username: string;
  score: number;
  opinion: Option<string>;
}
