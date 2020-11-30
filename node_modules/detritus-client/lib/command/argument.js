"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Argument = void 0;
const constants_1 = require("../constants");
const argumentparser_1 = require("./argumentparser");
const blankPrefixes = Object.freeze(['']);
/**
 * Command Argument
 * @category Command
 */
class Argument {
    constructor(options) {
        this._aliases = [];
        this._label = '';
        this._name = '';
        this._type = constants_1.CommandArgumentTypes.STRING;
        this.consume = false;
        this.default = undefined;
        this.help = '';
        this.prefixes = new Set(['-']);
        this.required = false;
        options = Object.assign({}, options);
        if (options.metadata !== undefined) {
            this.metadata = Object.assign({}, options.metadata);
        }
        if (options.prefix !== undefined) {
            if (!options.prefixes) {
                options.prefixes = [];
            }
            options.prefixes.push(options.prefix);
        }
        if (options.prefixes) {
            this.setPrefixes(options.prefixes, options.prefixSpace);
        }
        this.choices = options.choices;
        this.consume = !!options.consume;
        this.default = options.default;
        this.help = options.help || this.help;
        this.name = (options.name || this.name).toLowerCase();
        this.required = !!options.required;
        if (options.aliases) {
            this.aliases = options.aliases;
        }
        if (options.label) {
            this.label = options.label;
        }
        if (options.type) {
            this.type = options.type;
        }
    }
    get aliases() {
        return this._aliases;
    }
    set aliases(value) {
        this._aliases = (value || []).map((alias) => alias.toLowerCase());
        this._names = undefined;
    }
    get label() {
        return this._label || this.name;
    }
    set label(value) {
        this._label = value;
    }
    get name() {
        return this._name;
    }
    set name(value) {
        this._name = value;
        this._names = undefined;
    }
    get names() {
        if (this._names) {
            return this._names;
        }
        const names = [];
        const prefixes = (this.prefixes.size) ? this.prefixes : blankPrefixes;
        for (let prefix of prefixes) {
            names.push((prefix) ? prefix + this.name : this.name);
            for (let alias of this.aliases) {
                names.push((prefix) ? prefix + alias : alias);
            }
        }
        return this._names = names.sort((x, y) => y.length - x.length);
    }
    get type() {
        return this._type;
    }
    set type(value) {
        switch (value) {
            case Boolean:
                {
                    value = constants_1.CommandArgumentTypes.BOOL;
                }
                ;
                break;
            case Number:
                {
                    value = constants_1.CommandArgumentTypes.NUMBER;
                }
                ;
                break;
            case String:
                {
                    value = constants_1.CommandArgumentTypes.STRING;
                }
                ;
                break;
        }
        this._type = (value || this.type);
        if (typeof (this.default) !== 'function') {
            switch (this.type) {
                case constants_1.CommandArgumentTypes.BOOL:
                    {
                        this.default = !!this.default;
                    }
                    ;
                    break;
            }
        }
        if (Array.isArray(value)) {
            this.positionalArgs = new argumentparser_1.ArgumentParser(value, true);
        }
        else {
            this.positionalArgs = undefined;
        }
    }
    check(name) {
        return this.names.some((n) => n === name);
    }
    getInfo(content) {
        const info = { index: -1, name: '' };
        for (let name of this.names) {
            const index = content.indexOf(name);
            if (index !== -1) {
                info.index = index;
                info.name = name;
                break;
            }
        }
        return info;
    }
    getName(content) {
        for (let name of this.names) {
            if (name.includes(' ')) {
                const parts = name.split(' ');
                let matches = true;
                let copy = content;
                let store = '';
                for (let [key, part] of parts.entries()) {
                    if (copy.length === part.length) {
                        if (copy === part) {
                            store += copy;
                            copy = '';
                            continue;
                        }
                    }
                    else {
                        if (copy.startsWith(part + ' ')) {
                            store += part;
                            copy = copy.slice(part.length);
                            if (key !== (parts.length - 1)) {
                                while (copy.startsWith(' ')) {
                                    store += ' ';
                                    copy = copy.slice(1);
                                }
                            }
                            continue;
                        }
                    }
                    matches = false;
                    break;
                }
                if (matches) {
                    return store;
                }
            }
            else {
                if (content.length === name.length) {
                    if (content === name) {
                        return name;
                    }
                }
                else {
                    if (content.startsWith(name + ' ')) {
                        return name;
                    }
                }
            }
        }
        return null;
    }
    setPrefixes(prefixes, prefixSpace = false) {
        prefixes = prefixes.slice().sort((x, y) => y.length - x.length);
        if (prefixes.some((prefix) => prefix.endsWith(' '))) {
            prefixSpace = true;
        }
        this.prefixes.clear();
        for (let prefix of prefixes) {
            if (!prefix) {
                continue;
            }
            prefix = prefix.trim();
            if (prefixSpace) {
                prefix += ' ';
            }
            if (prefix) {
                this.prefixes.add(prefix);
            }
        }
        this._names = undefined;
    }
    async parse(value, context) {
        let parsedValue;
        if (typeof (this.type) === 'function') {
            parsedValue = await Promise.resolve(this.type(value, context));
        }
        else {
            try {
                switch (this.type) {
                    case constants_1.CommandArgumentTypes.BOOL:
                        {
                            parsedValue = !this.default;
                        }
                        ;
                        break;
                    case constants_1.CommandArgumentTypes.FLOAT:
                        {
                            parsedValue = parseFloat(value);
                        }
                        ;
                        break;
                    case constants_1.CommandArgumentTypes.NUMBER:
                        {
                            parsedValue = parseInt(value);
                        }
                        ;
                        break;
                    case constants_1.CommandArgumentTypes.STRING:
                        {
                            parsedValue = value || this.default || value;
                        }
                        ;
                        break;
                    default:
                        {
                            parsedValue = value || this.default;
                        }
                        ;
                        break;
                }
            }
            catch (error) {
                if (this.help) {
                    throw new Error(this.help.replace(/:error/g, error.message));
                }
                else {
                    throw error;
                }
            }
        }
        if (this.choices) {
            if (!this.choices.includes(parsedValue)) {
                throw new Error(this.help || `${parsedValue} is not a valid choice`);
            }
        }
        return parsedValue;
    }
}
exports.Argument = Argument;
