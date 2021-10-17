# Link to Notion

Quick add a link to a table within a [Notion](https://www.notion.so/) page.

# What is this about?

I use a particular page in [Notion](https://www.notion.so/) to save interesting links I find browsing. I don't really like to manually paste the link into the page, and the integrated extension in the browser for Notion lacks some feature (e.g., specifying tags).

This CLI utility simply helps a user to quick save a link in a Notion page.

## Requirements

- The utility is written in Rust, so you need [cargo](https://crates.io/) to compile it.
- A Notion API key and an integration with your Notion target page. Take a look at [this page](https://developers.notion.com/docs/getting-started) if you're unsure of what it means.

## Usage example

```bash
export NOTION_DATABASE_ID=...           # target page ID
export NOTION_KEY=...                   # your API key

l2n https://foo.bar -t tag1 tag2 tag3   # example 1
l2n https://bar.foo                     # example 2
```

Here, `example 1` adds the link together with three tags; `example 2` only adds the link without specifying any tag.

The following GIF shows a further example: