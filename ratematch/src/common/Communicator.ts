import { language_to_string, type Event, type Language, type Match, type MatchDescription, type NewRating, type Rating } from "@/common/entities";
import type { ApiResponse, Page, UnparsedMatch } from "@/common/response_entities";

/***
 * Interface responsible of the compatibility with the backend's API
 */
export interface IRateMatchApi {
  /**
  * Method to get an event page.
  * @param page - The page number
  */
  get_event_page(page: number): Promise<Page<Event>>;
  /**
   * Method to get informations about a given event.
   * @param event_id - The id of the event
   */
  get_event_informations(event_id: number): Promise<Event>
  /**
  * Method to get every match in an event.
  * @param event_id - The id of the event
  */
  get_event_card(event_id: number): Promise<UnparsedMatch[]>;
  /**
   * Method to get informations about a given match.
   * @param match_id - The id of the event
   */
  get_match_informations(match_id: number): Promise<UnparsedMatch>;
  /**
   * Method to get localized description about a given match.
   * @param match_id - The id of the match
   * @param language - The description language
   */
  get_match_localized_description(match_id: number, language: Language): Promise<MatchDescription>;
  /**
   * Method to get ratings for an event, in a specific language.
   * @parem event_id - The id of the event
   * @param language - The rating language
   */
  get_event_ratings(event_id: number, page: number, language: Language): Promise<Page<Rating>>;
  /**
  * Method to get ratings for a match, in a specific language.
  * @param match_id - The match
  * @param page - The page number
  * @param language - The ratings language
  */
  get_match_ratings(match_id: number, page: number, language: Language): Promise<Page<Rating>>;
  /**
   * Method to get global (= independent of any match) ratings, in a specific language.
   *
   */
  get_global_ratings(page: number, language: Language): Promise<Page<Rating>>;
  /**
  * Method to send a new rating for a match.
  * @param rating - The rating to send
  */
  add_match_rating(rating: NewRating): Promise<boolean>;
  /**
  * Method to get the average score for an event. 
  * @param event_id - The event id
  * @returns Either the average or -1 if there is no rating registered
  */
  get_average_score_for_event(event_id: number): Promise<number>;
}

export class Communicator implements IRateMatchApi {
  static API_URL: String = import.meta.env.VITE_API_URL;

  private async _get_method<T>(url: string): Promise<T> {
    const response = await fetch(url, {
      mode: "cors",
    });

    if (!response.ok) {
      let data: ApiResponse<any> = await response.json();
      await Promise.reject(`Could not get data. [${url}]\nReason: ${data.message}`);
    }

    let data: ApiResponse<T> = await response.json();
    return data.data as T;
  }

  async get_event_page(page: number): Promise<Page<Event>> {
    return await this._get_method<Page<Event>>(`${Communicator.API_URL}/events?page=${page}`);
  }

  async get_event_informations(event_id: number): Promise<Event> {
    return await this._get_method<Event>(`${Communicator.API_URL}/events/${event_id}`);
  }

  async get_event_card(event_id: number): Promise<Match[]> {
    return await this._get_method<Match[]>(`${Communicator.API_URL}/events/${event_id}/matches`);
  }

  async get_match_informations(match_id: number): Promise<Match> {
    return await this._get_method<Match>(`${Communicator.API_URL}/match/${match_id}`)
  }

  async get_match_localized_description(match_id: number, language: Language): Promise<MatchDescription> {
    return await this._get_method<MatchDescription>(`${Communicator.API_URL}/match/${match_id}/description?lang=${language_to_string(language)}`);
  }

  async get_event_ratings(event_id: number, page: number, language: Language): Promise<Page<Rating>> {
    return await this._get_method<Page<Rating>>(`${Communicator.API_URL}/events/${event_id}/ratings?page=${page}&lang=${language_to_string(language)}`);
  }

  async get_match_ratings(match_id: number, page: number, language: Language): Promise<Page<Rating>> {
    return await this._get_method<Page<Rating>>(`${Communicator.API_URL}/match/${match_id}/ratings?page=${page}&lang=${language_to_string(language)}`);
  }

  async get_global_ratings(page: number, language: Language): Promise<Page<Rating>> {
    return await this._get_method<Page<Rating>>(`${Communicator.API_URL}/ratings?page=${page}&lang=${language_to_string(language)}`);
  }

  async add_match_rating(rating: NewRating): Promise<boolean> {
    const response = await fetch(`${Communicator.API_URL}/match/ratings`,
      {
        method: "POST", body: JSON.stringify(rating), mode: "cors",
        headers: { "content-type": "application/json" }
      }
    );

    return response.ok;
  }

  async get_average_score_for_event(event_id: number): Promise<number> {
    try {
      return await this._get_method(`${Communicator.API_URL}/events/${event_id}/average`);
    } catch (err) {
      return -1;
    }
  }

  async get_average_score_for_match(match_id: number): Promise<number> {
    try {
      return await this._get_method(`${Communicator.API_URL}/match/${match_id}/average`);
    } catch (err) {
      return -1;
    }
  }
}
