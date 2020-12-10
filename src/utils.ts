export type Serializable = string | number | boolean;

export const CODEBLOCK_REGEX = new RegExp('^\\s*```\\w*|```\\s*$', 'g');

export interface TableData {
  header: Array<Serializable>;
  rows: Array<Array<Serializable>>;
  offset?: number;
}

interface ElapsedTime {
  seconds: number,
  minutes: number,
  hours: number,
  days: number
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

export function elapsed(value: number): ElapsedTime {
  const date: Date = new Date(value);
  const elapsed = { days: date.getUTCDate() - 1, hours: date.getUTCHours(), minutes: date.getUTCMinutes(), seconds: date.getUTCSeconds() };
  return elapsed;
}

export function flat<T = any>(arr: Array<T>, sizePerElement: number): Array<Array<T>> {
  const res = [];
  for (let i = 0; i < arr.length; i += sizePerElement) res.push(arr.slice(i, sizePerElement + i));
  return res;
}