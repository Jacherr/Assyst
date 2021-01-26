import * as Types from './types';
import { BaseRestClient } from '../baserestclient';

import { zx8Token } from '../../../config.json';

export const BASE_URL = 'https://zx8.jacher.io';
export const VERSION = 'v1';

export const API_URL = BASE_URL + '/api/' + VERSION;

export enum Endpoints {
  EVAL = '/rpc/:node',
  INFO = '/info',
  NODES = '/nodes',
  RECENT = '/recent',
  SEARCH = '/search'
}

export class Zx8 extends BaseRestClient {
  constructor () {
    super(API_URL);
  }

  public eval (nodeId: number, code: string) {
    return this.post(this.toEndpointString(Endpoints.EVAL, {
      node: String(nodeId)
    }),
    {
      authorization: zx8Token
    },
    {
      code
    });
  }

  public info (): Promise<Types.InfoResult> {
    return this.get(Endpoints.INFO);
  }

  public nodes (): Promise<Types.NodesResult[]> {
    return this.get(Endpoints.NODES);
  }

  public randomAudio () {
    const offset = Math.floor((Math.random() * 2000) || 0);
    return this.search({
      ct: Types.ContentTypes.OTHER,
      offset,
      query: '.mp3',
      limit: 5
    });
  }

  public randomImage () {
    const offset = Math.floor((Math.random() * 1000000) || 0);
    return this.search({
      ct: Types.ContentTypes.IMAGE,
      offset,
      query: '.',
      limit: 5
    });
  }

  public randomHtml () {
    const offset = Math.floor((Math.random() * 15000000) || 0);
    return this.search({
      ct: Types.ContentTypes.HTML,
      offset,
      query: '.',
      limit: 5
    });
  }

  public randomVideo () {
    const offset = Math.floor((Math.random() * 3000) || 0);
    return this.search({
      ct: Types.ContentTypes.VIDEO,
      offset,
      query: '.',
      limit: 5
    });
  }

  public recentIndexes (): Promise<Types.SearchResultEntry[]> {
    return this.get(Endpoints.RECENT);
  }

  public search (data: Types.SearchParam) {
    const fd = typeof data === 'string' ? ({
      query: data
    } as Types.SearchData) : data;

    return this.get<Array<Types.SearchResultEntry>>(Endpoints.SEARCH + this.toQueryString(fd));
  }
}
