"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.CODEBLOCK_REGEX = new RegExp('^\\s*```\\w*|```\\s*$', 'g');
function generateKVList(items) {
    const maxKeyLength = Math.max(...items.map(i => i[0].length));
    const maxValueLength = Math.max(...items.map(i => Math.max(...i[1].split('\n').map(i => i.length))));
    return items.map(i => {
        if (i[0].length === 0)
            return '';
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
exports.generateKVList = generateKVList;
function generateTable(data) {
    const divider = data.header.map(h => '-'.repeat(h.toString().length));
    const fd = [data.header, divider, ...data.rows];
    const longest = [];
    for (let i = 0; i < fd[0].length; ++i) {
        for (let j = 0; j < fd.length; ++j) {
            const thisCell = String(fd[j][i]);
            if (!longest[i] || thisCell.length > longest[i]) {
                longest[i] = thisCell.length;
            }
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
exports.generateTable = generateTable;
function parseCodeblocks(input) {
    return input.replace(exports.CODEBLOCK_REGEX, '');
}
exports.parseCodeblocks = parseCodeblocks;
function splitArray(array, size) {
    const out = [];
    for (let i = 0; i < array.length; i += size) {
        out.push(array.slice(i, i + size));
    }
    return out;
}
exports.splitArray = splitArray;
function elapsed(value) {
    const date = new Date(value);
    const elapsed = { days: date.getUTCDate() - 1, hours: date.getUTCHours(), minutes: date.getUTCMinutes(), seconds: date.getUTCSeconds() };
    return elapsed;
}
exports.elapsed = elapsed;
function flat(arr, sizePerElement) {
    const res = [];
    for (let i = 0; i < arr.length; i += sizePerElement)
        res.push(arr.slice(i, sizePerElement + i));
    return res;
}
exports.flat = flat;
