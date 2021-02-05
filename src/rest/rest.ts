import * as rextester from '../constants/rextester';

import fetch from 'node-fetch';

import { STATUS_CODES } from 'http'

import { isapiAuth, filerAuth } from '../../config.json';

export enum Endpoints {
    DISCORD_TENOR_GIF = 'https://discord.com/api/v8/gifs/search?media_format=gif&provider=tenor&locale=en-US&q=:q',
    DISCORD_TENOR_GIF_SUGGESTIONS = 'https://discord.com/api/v8/gifs/suggest?q=:q',
    FILER = 'https://cdn.jacher.io/',
    ISAPI = 'https://isapi.jacher.io/',
    OCR = 'https://ocr--y21_.repl.co/?url=:url',
    RUST = 'https://play.rust-lang.org/execute',
    BAD_TRANSLATOR = 'http://translate.y21_.repl.co/?text=:text',
    BULK_USER_LOOKUP = 'https://discard.cc/api/users/bulk/:ids',
    FAKE_EVAL_EXPERIMENTAL = 'http://q.y21_.repl.co/?code=:code'
}

export type Serializable = string | number | boolean

export interface IsapiData {
  image?: Buffer
  text?: string
  cpuTime: number
  wallTime: number
  format?: string
}

export interface RustData {
    channel?: string;
    backtrace?: boolean;
    crateType?: string;
    edition?: string;
    mode?: string;
    tests?: boolean;
}

export interface DiscardUser {
  id: string,
  username: string,
  avatar: string,
  discriminator: string,
  bot: boolean
}

export async function executeImageScript(script: string, inject?: { [key: string]: Serializable }): Promise<IsapiData> {
  return fetch(Endpoints.ISAPI, {
    method: 'POST',
    headers: {
      authorization: isapiAuth
    },
    body: JSON.stringify({
      script,
      inject
    })
  }).then(async res => {
    if(res.status !== 200) {
      if(res.status === 400) throw new Error(await res.text());
      else throw new Error(`Error ${res.status} (${STATUS_CODES[res.status]})`)
    } else {
      const out: IsapiData = {
        image: res.status === 200 ? await res.buffer() : undefined,
        text: res.headers.get('x-text') ? res.headers.get('x-text')?.split(' ').map(a => String.fromCharCode(parseInt(a))).join('') : undefined,
        cpuTime: parseInt(res.headers.get('x-cpu-time') as string),
        wallTime: parseInt(res.headers.get('x-wall-time') as string),
        format: res.headers.get('x-format') ?? undefined
      }
      return out;
    }
  })
}

export async function ocrImage(url: string) {
  return fetch(Endpoints.OCR.replace(':url', encodeURIComponent(url))).then(c => c.text());
}

export async function uploadFile (data: any, contentType: string) {
  return fetch(Endpoints.FILER, {
    headers: {
      'content-type': contentType,
      authorization: filerAuth
    },
    method: 'POST',
    body: data
  }).then(r => r.text());
}

export async function runRustCode (code: string, options: RustData = {}) {
  if (!code.includes('fn main(')) code = `fn main() {\n\t${code}\n}`;

  return fetch(Endpoints.RUST, {
    method: 'POST',
    body: JSON.stringify({
      code,
      channel: 'stable',
      crateType: 'bin',
      edition: '2018',
      mode: 'debug',
      tests: false,
      ...options
    })
  })
    .then(x => x.json())
    .then(x => x.error || x.stdout || x.stderr);
}

export async function fetchGifs (query: string): Promise<string[]> {
  return fetch(Endpoints.DISCORD_TENOR_GIF.replace(':q', encodeURIComponent(query))).then(x => x.json()).then(j => j.map((result: any) => result.src));
}

export async function fetchGifSuggestions(query: string) {
  return fetch(Endpoints.DISCORD_TENOR_GIF_SUGGESTIONS.replace(':q', encodeURIComponent(query))).then(x => x.json())
}

export async function badTranslate(text: string) {
  return fetch(Endpoints.BAD_TRANSLATOR.replace(':text', encodeURIComponent(text))).then(x => x.text());
}

export async function bulkUserLookup(userIds: Array<string>): Promise<Array<DiscardUser>> {
  return fetch(Endpoints.BULK_USER_LOOKUP.replace(':ids', userIds.join(','))).then(x => x.json());
}

export async function runCode(lang: string, code: string): Promise<string> {
  const langCode = rextester.LANGUAGES.get(lang);
  if (!langCode) return 'Language not found';

  return fetch(`${rextester.HOST}?LanguageChoice=${langCode}`, {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json',
        'LanguageChoice': String(langCode)
    },
    body: JSON.stringify({
        Program: code,
        CompilerArgs: '-o a.out source_file.c'
    })
  }).then(x => x.json()).then(x => x.Errors ?? x.Result ?? "Empty result");
}