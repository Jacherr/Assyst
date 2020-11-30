"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.escape = exports.trueSlice = exports.url = exports.underline = exports.strike = exports.spoiler = exports.italics = exports.codestring = exports.codeblock = exports.bold = exports.Replacements = exports.Regexes = exports.Strings = void 0;
exports.Strings = Object.freeze({
    BOLD: '**',
    CODEBLOCK: '```',
    CODESTRING: '`',
    CODESTRING_DOUBLE: '``',
    ESCAPE: '\\',
    ITALICS: '_',
    SPOILER: '||',
    STRIKE: '~~',
    UNDERLINE: '__',
});
exports.Regexes = Object.freeze({
    [exports.Strings.BOLD]: /\*\*/g,
    [exports.Strings.CODEBLOCK]: new RegExp(exports.Strings.CODEBLOCK, 'g'),
    [exports.Strings.CODESTRING]: new RegExp(exports.Strings.CODESTRING, 'g'),
    [exports.Strings.ESCAPE]: /\\/g,
    [exports.Strings.ITALICS]: /(_|\*)/g,
    [exports.Strings.SPOILER]: /\|\|/g,
    [exports.Strings.STRIKE]: new RegExp(exports.Strings.STRIKE, 'g'),
    [exports.Strings.UNDERLINE]: new RegExp(exports.Strings.UNDERLINE, 'g'),
    EVERYONE: /@(everyone|here)/g,
    LINK: /\]\(/g,
    MENTION: /<@([!&]?[0-9]{16,21})>/g,
    MENTION_HARDCORE: /@/g,
    URL: /\)/g,
});
exports.Replacements = Object.freeze({
    [exports.Strings.BOLD]: '\\*\\*',
    [exports.Strings.CODEBLOCK]: '``\u200b`',
    [exports.Strings.CODESTRING]: '\\`',
    [exports.Strings.ESCAPE]: '\\\\',
    [exports.Strings.ITALICS]: '\\$1',
    [exports.Strings.SPOILER]: '\\|\\|',
    [exports.Strings.STRIKE]: '\\~\\~',
    [exports.Strings.UNDERLINE]: '\\_\\_',
    MENTION: '\u200b',
});
const defaultMarkupFilter = Object.freeze({
    limit: 2000,
    links: true,
    mentions: true,
    mentionEscapeCharacter: '\u200b',
    replacement: '',
});
const defaultBoldFilter = Object.freeze(Object.assign({}, defaultMarkupFilter, {
    limit: 1996,
    replacement: exports.Replacements[exports.Strings.BOLD],
}));
function bold(text, options = {}) {
    text = exports.escape.bold(text, options);
    return `**${text}**`;
}
exports.bold = bold;
const defaultCodeblockFilter = Object.freeze(Object.assign({}, defaultMarkupFilter, {
    language: '',
    limit: 1990,
    replacement: exports.Replacements[exports.Strings.CODEBLOCK],
}));
function codeblock(text, options = {}) {
    text = exports.escape.codeblock(text, options);
    return [
        exports.Strings.CODEBLOCK + (options.language || defaultCodeblockFilter.language),
        text,
        exports.Strings.CODEBLOCK,
    ].join('\n');
}
exports.codeblock = codeblock;
const defaultCodestringFilter = Object.freeze(Object.assign({}, defaultMarkupFilter, {
    limit: 1998,
    replacement: exports.Replacements[exports.Strings.CODESTRING],
}));
function codestring(text, options = {}) {
    let wrap = exports.Strings.CODESTRING;
    if (text.includes(exports.Strings.CODESTRING)) {
        options = Object.assign({
            limit: 1995,
            replacement: exports.Strings.CODESTRING + exports.Replacements.MENTION,
        }, options);
        text = exports.escape.codestring(text, options);
        wrap = exports.Strings.CODESTRING_DOUBLE;
        if (text.endsWith(exports.Strings.CODESTRING)) {
            text += exports.Replacements.MENTION;
        }
    }
    else {
        text = exports.escape.codestring(text, options);
    }
    return `${wrap}${text}${wrap}`;
}
exports.codestring = codestring;
const defaultItalicsFilter = Object.freeze(Object.assign({}, defaultMarkupFilter, {
    limit: 1998,
    replacement: exports.Replacements[exports.Strings.ITALICS],
}));
function italics(text, options = {}) {
    text = exports.escape.italics(text, options);
    return `_${text}_`;
}
exports.italics = italics;
const defaultSpoilerFilter = Object.freeze(Object.assign({}, defaultMarkupFilter, {
    limit: 1996,
    replacement: exports.Replacements[exports.Strings.SPOILER],
}));
function spoiler(text, options = {}) {
    text = exports.escape.spoiler(text, options);
    return `||${text}||`;
}
exports.spoiler = spoiler;
const defaultStrikeFilter = Object.freeze(Object.assign({}, defaultMarkupFilter, {
    limit: 1996,
    replacement: exports.Replacements[exports.Strings.STRIKE],
}));
function strike(text, options = {}) {
    text = exports.escape.strike(text, options);
    return `~~${text}~~`;
}
exports.strike = strike;
const defaultUnderlineFilter = Object.freeze(Object.assign({}, defaultMarkupFilter, {
    limit: 1996,
    replacement: exports.Replacements[exports.Strings.UNDERLINE],
}));
function underline(text, options = {}) {
    text = exports.escape.underline(text, options);
    return `__${text}__`;
}
exports.underline = underline;
function url(text, url) {
    url = exports.escape.url(url);
    return `[${text}](${url})`;
}
exports.url = url;
function trueSlice(text, limit) {
    if (limit) {
        return Buffer.from(text).slice(0, limit).toString();
    }
    return text;
}
exports.trueSlice = trueSlice;
exports.escape = Object.freeze({
    all: (text, options = {}) => {
        const filter = Object.assign({}, defaultMarkupFilter, options);
        text = text.replace(exports.Regexes[exports.Strings.ESCAPE], exports.Replacements[exports.Strings.ESCAPE]);
        text = text.replace(exports.Regexes[exports.Strings.ITALICS], exports.Replacements[exports.Strings.ITALICS]);
        text = text.replace(exports.Regexes[exports.Strings.BOLD], exports.Replacements[exports.Strings.BOLD]);
        text = text.replace(exports.Regexes[exports.Strings.CODESTRING], exports.Replacements[exports.Strings.CODESTRING]);
        text = text.replace(exports.Regexes[exports.Strings.SPOILER], exports.Replacements[exports.Strings.SPOILER]);
        text = text.replace(exports.Regexes[exports.Strings.STRIKE], exports.Replacements[exports.Strings.STRIKE]);
        text = text.replace(exports.Regexes[exports.Strings.UNDERLINE], exports.Replacements[exports.Strings.UNDERLINE]);
        if (filter.links) {
            text = exports.escape.links(text, filter.mentionEscapeCharacter);
        }
        if (filter.mentions) {
            text = exports.escape.mentions(text, filter.mentionEscapeCharacter);
        }
        return trueSlice(text, filter.limit);
    },
    bold: (text, options = {}) => {
        const filter = Object.assign({}, defaultBoldFilter, options);
        text = text.replace(exports.Regexes[exports.Strings.BOLD], filter.replacement);
        if (filter.mentions) {
            text = exports.escape.mentions(text, filter.mentionEscapeCharacter);
        }
        return trueSlice(text, filter.limit);
    },
    codeblock: (text, options = {}) => {
        const filter = Object.assign({}, defaultCodeblockFilter, options);
        while (text.includes(exports.Strings.CODEBLOCK)) {
            text = text.replace(exports.Regexes[exports.Strings.CODEBLOCK], filter.replacement);
        }
        if (options.limit === undefined) {
            filter.limit -= filter.language.length;
        }
        return trueSlice(text, filter.limit);
    },
    codestring: (text, options = {}) => {
        const filter = Object.assign({}, defaultCodestringFilter, options);
        text = text.replace(exports.Regexes[exports.Strings.CODESTRING], filter.replacement);
        if (filter.mentions) {
            text = exports.escape.mentions(text, filter.mentionEscapeCharacter);
        }
        return trueSlice(text, filter.limit);
    },
    italics: (text, options = {}) => {
        const filter = Object.assign({}, defaultItalicsFilter, options);
        text = text.replace(exports.Regexes[exports.Strings.ITALICS], filter.replacement);
        if (filter.mentions) {
            text = exports.escape.mentions(text, filter.mentionEscapeCharacter);
        }
        return trueSlice(text, filter.limit);
    },
    links: (text, replacement = exports.Replacements.MENTION) => {
        text = text.replace(exports.Regexes.LINK, `]${replacement}(`);
        return text;
    },
    mentions: (text, replacement = exports.Replacements.MENTION) => {
        text = text.replace(exports.Regexes.MENTION_HARDCORE, `@${replacement}`);
        return text;
    },
    spoiler: (text, options = {}) => {
        const filter = Object.assign({}, defaultSpoilerFilter, options);
        text = text.replace(exports.Regexes[exports.Strings.SPOILER], filter.replacement);
        if (filter.mentions) {
            text = exports.escape.mentions(text, filter.mentionEscapeCharacter);
        }
        return trueSlice(text, filter.limit);
    },
    strike: (text, options = {}) => {
        const filter = Object.assign({}, defaultStrikeFilter, options);
        text = text.replace(exports.Regexes[exports.Strings.STRIKE], filter.replacement);
        if (filter.mentions) {
            text = exports.escape.mentions(text, filter.mentionEscapeCharacter);
        }
        return trueSlice(text, filter.limit);
    },
    underline: (text, options = {}) => {
        const filter = Object.assign({}, defaultUnderlineFilter, options);
        text = text.replace(exports.Regexes[exports.Strings.UNDERLINE], filter.replacement);
        if (filter.mentions) {
            text = exports.escape.mentions(text, filter.mentionEscapeCharacter);
        }
        return trueSlice(text, filter.limit);
    },
    url: (text, options = {}) => {
        text = text.replace(exports.Regexes.URL, (match) => {
            return '%' + match.charCodeAt(0).toString(16);
        });
        return text;
    },
});
