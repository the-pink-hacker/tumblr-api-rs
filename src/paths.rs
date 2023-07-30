macro_rules! paths {
    (
        path_prefix: $path_prefix: literal
        $($(#[$attr:meta])* $name: ident $path: literal $($argument: ident)*),*
        $(,)?
    ) => {
        $(
            $(#[$attr])*
            pub fn $name($($argument: impl core::fmt::Display),*) -> Result<reqwest::Url, oauth2::url::ParseError> {
                reqwest::Url::parse(&format!(concat!("https://api.tumblr.com/v2/", $path_prefix, $path), $($argument),*))
            }
        )*
    };
}

paths!(
    path_prefix: "blog/{}/"

    /// # Retrieve Blog Info
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#info---retrieve-blog-info
    ///
    /// Method: `GET`
    ///
    /// Auth: `API Key`
    blog_info "info" blog_id,

    /// # Retrieve a Blog Avatar
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#avatar--retrieve-a-blog-avatar
    ///
    /// Method: `GET`
    ///
    /// Auth: `None`
    blog_avatar "avatar" blog_id,

    /// # Retrieve a Blog Avatar
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#avatar--retrieve-a-blog-avatar
    ///
    /// Method: `GET`
    ///
    /// Auth: `None`
    blog_avatar_size "avatar/{}" blog_id size,

    /// # Retrieve Blog's Blocks
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#blocks--retrieve-blogs-blocks
    ///
    /// Method: `GET`
    ///
    /// Auth: `OAuth`
    blog_blocks_get "blocks" blog_id,

    /// # Block a Blog
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#blocks--block-a-blog
    ///
    /// Method: `POST`
    ///
    /// Auth: `OAuth`
    ///
    /// ---
    ///
    /// # Remove a Block
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#blocks--remove-a-block
    ///
    /// Method: `DELETE`
    ///
    /// Auth: `OAuth`
    blog_block "blocks" blog_id,

    /// # Block a list of Blogs
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#blocksbulk--block-a-list-of-blogs
    ///
    /// Method: `POST`
    ///
    /// Auth: `OAuth`
    blog_blocks_bulk "blocks/bulk" blog_id,

    /// # Retrieve Blog's Likes
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#likes--retrieve-blogs-likes
    ///
    /// Method: `GET`
    ///
    /// Auth: `API Key`
    blog_likes "likes" blog_id,

    /// # Retrieve Blog's following
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#following--retrieve-blogs-following
    ///
    /// Method: `GET`
    ///
    /// Auth: `OAuth`
    blog_following "following" blog_id,

    /// # Retrieve a Blog's Followers
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#followers--retrieve-a-blogs-followers
    ///
    /// Method: `GET`
    ///
    /// Auth: `OAuth`
    blog_followers "followers" blog_id,

    /// # Check If Followed By Blog
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#followed_by--check-if-followed-by-blog
    ///
    /// Method: `GET`
    ///
    /// Auth: `OAuth`
    blog_followed_by "followed_by" blog_id,

    /// # Retrieve Published Posts
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#posts--retrieve-published-posts
    ///
    /// Method: `GET`
    ///
    /// Auth: `API Key`
    blog_posts_search "posts" blog_id,

    /// # Retrieve Published Posts
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#posts--retrieve-published-posts
    ///
    /// Method: `GET`
    ///
    /// Auth: `API Key`
    blog_posts_get_type "posts/{}" blog_id post_type,

    /// # Retrieve Queued Posts
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#postsqueue--retrieve-queued-posts
    ///
    /// Method: `GET`
    ///
    /// Auth: `OAuth`
    blog_post_queue_get "posts/queue" blog_id,

    /// # Reorder Queued Posts
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#postsqueuereorder--reorder-queued-posts
    ///
    /// Method: `POST`
    ///
    /// Auth: `OAuth`
    blog_post_queue_reorder "posts/queue/reorder" blog_id,

    /// # Shuffle Queued Posts
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#postsqueueshuffle---shuffle-queued-posts
    ///
    /// Method: `POST`
    ///
    /// Auth: `OAuth`
    blog_post_queue_shuffle "posts/queue/shuffle" blog_id,

    /// # Retrieve Draft Posts
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#postsdraft--retrieve-draft-posts
    ///
    /// Method: `GET`
    ///
    /// Auth: `OAuth`
    blog_post_draft_get "posts/draft" blog_id,

    /// # Retrieve Submission Posts
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#postssubmission--retrieve-submission-posts
    ///
    /// Method: `GET`
    ///
    /// Auth: `OAuth`
    blog_post_submission_get "posts/submission" blog_id,

    /// # Retrieve Blog's Activity Feed
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#notifications--retrieve-blogs-activity-feed
    ///
    /// Method: `GET`
    ///
    /// Auth: `OAuth`
    blog_notifications "notification" blog_id,

    /// # Create/Reblog a Post (Neue Post Format)
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#post--create-a-new-blog-post-legacy
    ///
    /// Method: `POST`
    ///
    /// Auth: `OAuth`
    blog_post_create "posts" blog_id,

    /// # Fetching a Post (Neue Post Format)
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#postspost-id---fetching-a-post-neue-post-format
    ///
    /// Method: `GET`
    ///
    /// Auth: `OAuth`
    ///
    /// ---
    ///
    /// # Editing a Post (Neue Post Format)
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#postspost-id---editing-a-post-neue-post-format
    ///
    /// Method: `PUT`
    ///
    /// Auth: `OAuth`
    blog_post "posts/{}" blog_id post_id,

    /// # Delete a Post
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#postdelete--delete-a-post
    ///
    /// Method: `POST`
    ///
    /// Auth: `OAuth`
    blog_post_delete "post/delete" blog_id,

    /// # Muting a Post's Notifications
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#postspost-idmute---muting-a-posts-notifications
    ///
    /// Method: `POST` | `DELETE`
    ///
    /// Auth: `OAuth`
    blog_post_mute "posts/{}/mute" blog_id post_id,

    /// # Get notes for a specific Post
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#notes---get-notes-for-a-specific-post
    ///
    /// Method: `GET`
    ///
    /// Auth: `API Key`
    blog_notes "notes" blog_id,

    /// # Edit a Blog Post (Legacy)
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#postedit--edit-a-blog-post-legacy
    ///
    /// Method: `POST`
    ///
    /// Auth: `OAuth`
    #[deprecated = "Neue format is now preferred."]
    blog_legacy_post_edit "post/edit" blog_id,

    /// # Create a New Blog Post (Legacy)
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#post--create-a-new-blog-post-legacy
    ///
    /// Method: `POST`
    ///
    /// Auth: `OAuth`
    #[deprecated = "Neue format is now preferred."]
    blog_legacy_post "post" blog_id,

    /// # Reblog a Post (Legacy)
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#postreblog--reblog-a-post-legacy
    ///
    /// Method: `POST`
    ///
    /// Auth: `OAuth`
    #[deprecated = "Neue format is now preferred."]
    blog_legacy_post_reblog "post/reblog" blog_id,
);

paths!(
    path_prefix: "user/"

    /// # Retrieve a User's Dashboard
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#userlimits--get-a-users-limits
    ///
    /// Method: `GET`
    ///
    /// Auth: `OAuth`
    user_info "info",

    /// # Retrieve a User's Dashboard
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#userlimits--get-a-users-limits
    ///
    /// Method: `GET`
    ///
    /// Auth: `OAuth`
    user_limits "limits",

    /// # Retrieve a User's Dashboard
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#userdashboard--retrieve-a-users-dashboard
    ///
    /// Method: `GET`
    ///
    /// Auth: `OAuth`
    user_dashboard "dashboard",

    /// # Retrieve a User's Likes
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#userlikes--retrieve-a-users-likes
    ///
    /// Method: `GET`
    ///
    /// Auth: `OAuth`
    user_likes "likes",

    /// # Retrieve the Blogs a User Is Following
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#userfollowing--retrieve-the-blogs-a-user-is-following
    ///
    /// Method: `GET`
    ///
    /// Auth: `OAuth`
    user_following "following",

    /// # Follow a blog
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#userfollow--follow-a-blog
    ///
    /// Method: `POST`
    ///
    /// Auth: `OAuth`
    user_follow "follow",

    /// # Unfollow a blog
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#userunfollow--unfollow-a-blog
    ///
    /// Method: `POST`
    ///
    /// Auth: `OAuth`
    user_unfollow "unfollow",

    /// # Like a Post
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#userlike--like-a-post
    ///
    /// Method: `POST`
    ///
    /// Auth: `OAuth`
    user_like "like",

    /// # Unlike a Post
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#userunlike--unlike-a-post
    ///
    /// Method: `POST`
    ///
    /// Auth: `OAuth`
    user_unlike "unlike",

    /// # Tag Filtering
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#userfiltered_tags---tag-filtering
    ///
    /// Method: `GET` | `POST`
    ///
    /// Auth: `OAuth`
    user_filter_tags "filtered_tags",

    /// # Tag Filtering
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#userfiltered_tags---tag-filtering
    ///
    /// Method: `DELETE`
    ///
    /// Auth: `OAuth`
    user_filter_content_delete "filtered_tags/{}" tag,

    /// # Content Filtering
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#userfiltered_content---content-filtering
    ///
    /// Method: `GET` | `POST` | `DELETE`
    ///
    /// Auth: `OAuth`
    user_filter_content "filter_content",
);

paths!(
    path_prefix: ""

    /// # Get Posts with Tag
    ///
    /// Docs: https://www.tumblr.com/docs/en/api/v2#tagged--get-posts-with-tag
    ///
    /// Method: `GED`
    ///
    /// Auth: `API Key` | `OAuth`
    tagged_get "tagged",
);
