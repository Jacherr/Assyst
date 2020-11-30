"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ArgumentParser = void 0;
const utils_1 = require("../utils");
const argument_1 = require("./argument");
/**
 * Command Argument
 * @category Command
 */
class ArgumentParser {
    constructor(args = [], positional) {
        this.args = [];
        this.positional = false;
        this.positional = !!positional;
        this.initialize(args);
    }
    initialize(args = []) {
        this.args.length = 0;
        for (let arg of args) {
            this.args.push(new argument_1.Argument(arg));
        }
    }
    async parse(attributes, context) {
        const errors = {};
        const parsed = {};
        if (this.args.length) {
            if (this.positional) {
                for (const arg of this.args) {
                    try {
                        let value;
                        if (arg.consume) {
                            value = attributes.content;
                            attributes.content = '';
                        }
                        else {
                            if (attributes.content) {
                                // get first value from attributes.content;
                                let [x, content] = utils_1.getFirstArgument(attributes.content);
                                value = x;
                                attributes.content = content;
                            }
                            else {
                                continue;
                            }
                        }
                        parsed[arg.label] = await arg.parse(value.trim(), context);
                    }
                    catch (error) {
                        errors[arg.label] = error;
                    }
                }
            }
            else {
                const insensitive = attributes.content.toLowerCase();
                const args = this.args
                    .map((arg) => ({ arg, info: arg.getInfo(insensitive) }))
                    .filter((x) => x.info.index !== -1)
                    .sort((x, y) => y.info.index - x.info.index);
                for (const { arg, info } of args) {
                    const value = attributes.content.slice(info.index + info.name.length);
                    // incase something like `.command -argSOMEVALUE` happens, we the arg
                    if (value && !value.startsWith(' ')) {
                        continue;
                    }
                    attributes.content = attributes.content.slice(0, info.index).trim();
                    try {
                        if (arg.positionalArgs) {
                            const positional = await arg.positionalArgs.parse({ content: value, prefix: '' }, context);
                            Object.assign(parsed, positional.parsed);
                            Object.assign(errors, positional.errors);
                        }
                        else {
                            parsed[arg.label] = await arg.parse(value.trim(), context);
                        }
                    }
                    catch (error) {
                        errors[arg.label] = error;
                    }
                }
            }
            for (let arg of this.args) {
                if (!(arg.label in parsed) && !(arg.label in errors)) {
                    try {
                        if (arg.default !== undefined) {
                            let value;
                            if (typeof (arg.default) === 'function') {
                                value = await Promise.resolve(arg.default(context));
                            }
                            else {
                                value = arg.default;
                            }
                            if (typeof (value) === 'string') {
                                value = await arg.parse(value, context);
                            }
                            parsed[arg.label] = value;
                        }
                        else if (arg.required) {
                            throw new Error(arg.help || 'Missing required parameter');
                        }
                        // or else ignore the arg
                    }
                    catch (error) {
                        errors[arg.label] = error;
                    }
                }
            }
        }
        return { errors, parsed };
    }
}
exports.ArgumentParser = ArgumentParser;
