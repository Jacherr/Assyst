import fetch from 'node-fetch';

export enum Endpoints {
    DISCORD_TENOR_GIF = 'https://discord.com/api/v8/gifs/search?media_format=gif&provider=tenor&locale=en-US&q=:q',
    DISCORD_TENOR_GIF_SUGGESTIONS = 'https://discord.com/api/v8/gifs/suggest?q=:q',
    TSU = 'https://tsu.sh',
    RUST = 'https://play.rust-lang.org/execute'
}

export interface RustData {
    channel?: string;
    backtrace?: boolean;
    crateType?: string;
    edition?: string;
    mode?: string;
    tests?: boolean;
}

export async function uploadToTsu (data: any, contentType: string) {
  return fetch(Endpoints.TSU, {
    headers: {
      'content-type': contentType
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