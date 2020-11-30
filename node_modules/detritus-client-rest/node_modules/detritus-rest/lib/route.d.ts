export interface RouteParameters {
    [key: string]: any;
}
export declare class Route {
    method: string;
    params: RouteParameters;
    path: string;
    urlPath: string;
    constructor(method: string, path?: string, params?: RouteParameters);
}
export declare const PathReplacementRegexp: RegExp;
export declare function replacePathParameters(path: string, parameters?: RouteParameters): string;
