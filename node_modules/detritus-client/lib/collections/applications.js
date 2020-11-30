"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Applications = void 0;
const basecollection_1 = require("./basecollection");
const application_1 = require("../structures/application");
;
/**
 * Applications Collection
 * @category Collections
 */
class Applications extends basecollection_1.BaseClientCollection {
    constructor() {
        super(...arguments);
        this.lastRefresh = 0;
        this.refreshTime = 4 * (60 * 60) * 1000;
    }
    // 4 hours minimum in between application fetches
    get shouldRefresh() {
        return !this.length || this.refreshTime <= Date.now() - this.lastRefresh;
    }
    insert(application) {
        if (this.enabled) {
            this.set(application.id, application);
        }
    }
    async fill(applications) {
        if (this.enabled) {
            if (applications) {
                this.lastRefresh = Date.now();
            }
            else {
                if (!this.shouldRefresh) {
                    return;
                }
                applications = await this.client.rest.raw.fetchApplicationsDetectable();
                this.lastRefresh = Date.now();
            }
            this.clear();
            for (let raw of applications) {
                const application = new application_1.Application(this.client, raw);
                this.insert(application);
            }
        }
    }
    get [Symbol.toStringTag]() {
        return `Applications (${this.size.toLocaleString()} items)`;
    }
}
exports.Applications = Applications;
