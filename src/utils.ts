export type Serializable = string | number | boolean;

export const CODEBLOCK_REGEX = new RegExp('^\\s*```\\w*|```\\s*$', 'g');
export const TIME_REGEX = /(\d+)([a-z]+)/gi

export interface TableData {
  header: Array<Serializable>;
  rows: Array<Array<Serializable>>;
  offset?: number;
}

export interface ElapsedTime {
  seconds: number,
  minutes: number,
  hours: number,
  days: number,
  years: number
}

const enum Units {
  SECOND = 1000,
  MINUTE = SECOND * 60,
  HOUR = MINUTE * 60,
  DAY = HOUR * 24,
  WEEK = DAY * 7,
  YEAR = DAY * 365.25
}

const multipliers = new Map([
  ['s', Units.SECOND],
  ['sec', Units.SECOND],
  ['m', Units.MINUTE],
  ['min', Units.MINUTE],
  ['h', Units.HOUR],
  ['hour', Units.HOUR],
  ['hours', Units.HOUR],
  ['d', Units.DAY],
  ['days', Units.DAY],
  ['day', Units.DAY],
  ['y', Units.YEAR],
  ['years', Units.YEAR],
  ['year', Units.YEAR]
]);

export function parseTimestamp(input: string, max?: number): number {
  let total = 0;

  for (const [, value, unit] of input.matchAll(TIME_REGEX)) {
    const multiplier = multipliers.get(unit.toLowerCase());

    const valueNum = Number(value);

    if (multiplier) {
      const cur = valueNum * multiplier;
      if (max && total + cur > max) return max;

      total += cur;
    }
  }

  return total;
}

export function elapsed(ms: number): ElapsedTime {
  let res: ElapsedTime = {
    years: 0,
    days: 0,
    hours: 0,
    minutes: 0,
    seconds: 0
  };

  res.years = Math.floor(ms / Units.YEAR);
  ms -= res.years * Units.YEAR;

  res.days = Math.floor(ms / Units.DAY);
  ms -= res.days * Units.DAY;

  res.hours = Math.floor(ms / (Units.HOUR));
  ms -= res.hours * Units.HOUR;

  res.minutes = Math.floor(ms / Units.MINUTE);
  ms -= res.minutes * Units.MINUTE;

  res.seconds = Math.floor(ms / Units.SECOND);
  ms -= res.seconds * Units.SECOND;

  return res;
}

export function formatElapsed(elapsed: ElapsedTime) {
  let out = '';
  if(elapsed.years > 0) out += `${elapsed.years} year${elapsed.years > 1 ? 's' : ''}, `;
  if(elapsed.days > 0) out += `${elapsed.days} day${elapsed.days > 1 ? 's' : ''}, `;
  if(elapsed.hours > 0) out += `${elapsed.hours} hour${elapsed.hours > 1 ? 's' : ''}, `
  if(elapsed.minutes > 0) out += `${elapsed.minutes} minute${elapsed.minutes > 1 ? 's' : ''}, `
  if(elapsed.seconds > 0) out += `${elapsed.seconds} second${elapsed.seconds > 1 ? 's' : ''}`

  out = out.trim();
  if(out.endsWith(',')) out = out.slice(0, out.length - 1);
  return out;
}

export function generateKVList(items: [string, string][]) {
  const maxKeyLength = Math.max(...items.map(i => i[0].length));
  const maxValueLength = Math.max(...items.map(i => Math.max(...i[1].split('\n').map(i => i.length))));

  return items.map(i => {
    if (i[0].length === 0) return '';
    const lineName = i[0].padStart(maxKeyLength);
    const lineValue = i[1]
      .split('\n')
      .map((content, index) => {
        const valuePadding = index > 0 ? maxValueLength + i[0].length + 2 : maxValueLength;
        return content.padStart(valuePadding);
      })
      .join('\n');


    return `${lineName}: ${lineValue}`;
  }).join('\n');
}

export function generateTable(data: TableData) {
  const divider = data.header.map(h => '-'.repeat(h.toString().length));

  const fd = [data.header, divider, ...data.rows];
  const longest: Array<number> = [];

  for (let i = 0; i < fd[0].length; ++i) {
    for (let j = 0; j < fd.length; ++j) {
      const thisCell = String(fd[j][i]);
      if (!longest[i] || thisCell.length > longest[i]) { longest[i] = thisCell.length; }
    }
  }

  const value = fd
    .map((x) => {
      return x
        .map((x, i) => {
          const padding = longest[i] + (data.offset || 2);
          return String(x).padEnd(padding, ' ');
        })
        .join('');
    })
    .join('\n');

  return value;
}

export function parseCodeblocks(input: string): string {
  return input.replace(CODEBLOCK_REGEX, '');
}

export function splitArray<T = any>(array: Array<T>, size: number): Array<Array<T>> {
  const out = [];
  for (let i = 0; i < array.length; i += size) {
    out.push(array.slice(i, i + size));
  }
  return out;
}

export function flat<T = any>(arr: Array<T>, sizePerElement: number): Array<Array<T>> {
  const res = [];
  for (let i = 0; i < arr.length; i += sizePerElement) res.push(arr.slice(i, sizePerElement + i));
  return res;
}

export function formatMessageLink(guildId: string, channelId: string, messageId: string) {
  return `https://discord.com/channels/${guildId}/${channelId}/${messageId}`
}