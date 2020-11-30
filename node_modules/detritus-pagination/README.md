# detritus-pagination
An easy-to-use paginator for the Discord API Wrapper [Detritus](https://npmjs.org/detritus-client).

## Using CommandClient
By default, `CommandClient#client` returns a ClusterClient, which is a class that manages `ShardClient`s. <br />
To make it work with `detritus-pagination`, either set `useClusterClient` to false in the CommandClient constructor, or use `PaginatorCluster` instead. <br />
It maintains a `WeakMap<ShardClient, Paginator>` internally.

## Examples
```js
// Imports
const { ShardClient } = require("detritus-client");
const { Paginator } = require("detritus-pagination");

// Detritus Client
const client = new ShardClient("token");

// Pagination Handler
const paginator = new Paginator(client, {
    // Maximum number of milliseconds for the bot to paginate
    // It is recommended not to set this too high
    // Defaults to 300000ms (5 minutes)
    maxTime: 300000,
    // Whether it should jump back to page 1 if the user tried to go past the last page
    // Defaults to false
    pageLoop: true,
    // Whether a page number should be shown in embed footers
    // If a string is passed as page, it will append the page number to the string
    pageNumber: true
});

// Reactions that will be passed in paginator.createReactionPaginator lateron
// This is optional and default emojis will be used if no `reactions` object is passed
const reactions = {
    previousPage: "⬅️",
    nextPage: "➡️"
};

const createEmbedMessage = (title, description) => ({
    embed: { title, description }
});

// Message event for commands 
client.on("messageCreate", async ctx => {
    const { message } = ctx;
    if (message.content === "!!test") {
        // Pages for this command
        const pages = [
            createEmbedMessage("Hello", ":)"),
            createEmbedMessage("Bye", ":(")
        ];

        // Create a ReactionPaginator
        const paging = await paginator.createReactionPaginator({
            // message is the message the user has sent
            message,
            // pages is an array of pages (will be passed as parameter in Message#edit)
            pages,
            // reactions is an object that includes `previousPage` and `nextPage` emojis (defined above)
            reactions
        });

        // You can also listen to events!
        // `next` event is fired when the user reacts with next page emoji
        // `previous` event is fired when the user reacts with previous page emoji
        paging.on("next", () => {
            message.reply("`next` event triggered");
        });

        paging.on("previous", () => {
            message.reply("`previous` event triggered");
        })

        paging.on("page", data => {
            message.reply(`skipped to page ${data.page}`);
        });
    }
});

// Run the client
(async () => {
    await client.run();
    console.log("Ready!");
})();
```
