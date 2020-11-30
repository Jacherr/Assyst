export type SearchParam = string | SearchData;

export enum ContentTypes {
    ALL = -1,
    OTHER,
    IMAGE,
    ANIMATED,
    VIDEO,
    HTML
}

export interface SearchData {
    query: string;
    limit?: number;
    ct?: number;
    offset?: number;
    ocr?: boolean;
}

export interface SearchResultEntry {
    url: string;
    host: string;
    lastStatus: number;
    contentType: ContentTypes;
    ocr: string;
}

export interface InfoResult {
    urlQueue: number;
    totalURLs: number;
    rss: number;
    tableSize: number;
    indexesPerSecond: number;
    contentTypes: {
        image: number;
        animated: number;
        video: number;
        html: number;
        other: number;
    };
}

export interface NodesResult {
    id: number;
    port: number;
    ssl: boolean;
    ping: number;
    memory: number;
    available: boolean;
    queue: number
}
