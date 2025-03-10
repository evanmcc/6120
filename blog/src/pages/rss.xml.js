import rss from "@astrojs/rss";
import { getCollection, render } from "astro:content";
import { SITE_TITLE, SITE_DESCRIPTION } from "../consts";

export async function GET(context) {
    const posts = await getCollection("posts", ({ data }) => {
        return import.meta.env.PROD ? data.draft !== true : true;
    });

    const feed_items = posts.map((post) => {
      return {
          content: post.rendered.html,
          link: `/${post.slug}/`,
          title: post.data.title,
          pubDate: post.data.date,
          description: post.data.description,
      };
    });

    return rss({
        title: SITE_TITLE,
        description: SITE_DESCRIPTION,
        site: context.site,
        items: feed_items,
    });
}
