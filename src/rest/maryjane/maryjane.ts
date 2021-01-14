import * as Types from './types';

import { maryjaneUrl } from '../../../config.json';
import { BaseRestClient } from '../baserestclient';

export const BASE_URL = maryjaneUrl;

export enum Endpoints {
    GUILD = '/guilds/:id',
    USER = '/users/:id/profile',
    BULK_USER = '/users/bulk/:ids'
}

export class Maryjane extends BaseRestClient {
  constructor() {
    super(BASE_URL)
  }

  public guild(id: string): Promise<Types.Guild> {
    return this.get(this.toEndpointString(Endpoints.GUILD, { id }))
  }

  public user(id: string): Promise<Types.User> {
    return this.get(this.toEndpointString(Endpoints.USER, { id }))
  }

  public bulkUser(ids: string[]): Promise<Types.User[]> {
    return this.get(this.toEndpointString(Endpoints.BULK_USER, {
      ids: encodeURIComponent(ids.join(','))
    }));
  }
}
