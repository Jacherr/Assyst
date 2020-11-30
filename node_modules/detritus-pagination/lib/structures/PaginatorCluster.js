const { ClusterClient } = require("detritus-client");
const Paginator = require("./Paginator");
const assert = require("assert");

module.exports = class PaginatorCluster {
    constructor(clusterClient, data = {}) {
        assert.ok(
            clusterClient instanceof ClusterClient,
            "clusterClient must be an instance of ClusterClient"
        );

        const paginators = new WeakMap();

        for (const [, client] of clusterClient.shards) {
            paginators.set(client, new Paginator(client, data));
        }

        this.data = data;
        this.paginators = paginators;
    }

    findOrSetPaginator(client) {
        const cachedPaginator = this.paginators.get(client);
        if (cachedPaginator) return cachedPaginator;

        const paginator = new Paginator(client, this.data);
        this.paginators.set(client, paginator);

        return paginator;
    }

    createReactionPaginator(data) {
        const targetPaginator = this.findOrSetPaginator(data.message.client);

        return targetPaginator.createReactionPaginator(data);
    }
}