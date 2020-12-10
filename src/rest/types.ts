export enum RatelimitHeaders {
    LIMIT = 'x-ratelimit-limit',
    REMAINING = 'x-ratelimit-remaining',
    RESET = 'x-ratelimit-reset'
}

export interface RatelimitInfo {
    [RatelimitHeaders.LIMIT]: number
    [RatelimitHeaders.REMAINING]: number
    [RatelimitHeaders.RESET]: number
}
