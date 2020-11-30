module.exports = {
    Paginator: require("./structures/Paginator"),
    PaginatorCluster: require("./structures/PaginatorCluster"),
    get version() {
        return require("../package").version;
    }
};