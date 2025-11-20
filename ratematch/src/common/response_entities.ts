/**
 * Represents an ApiResponse.
 */
export interface ApiResponse<T> {
  message: String,
  data: T
}

/**
 * Represents a page of data.
 */
export interface Page<T> {
  page: number;
  page_total: number;
  items: T[];
}

export interface UnparsedMatch {
  id: number;
  event_id: number;
  workers: string;
}
