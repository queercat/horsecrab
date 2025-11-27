# TODO
- [x] Idiomatic conditional rendering. 
- [x] JWT based authentication.
- [ ] RBAC for authorization.
- [ ] Disable / enable registration.
- [ ] Token only registration.
- [ ] Database backed CMS topic / sections.
- [ ] Admin panel.
- [ ] Draw a logo that isn't emoji slop.
- [ ] Use anyhow for errors.

# Ideas
- Expose the full source code on the site, provide utilities for generating a sha256 hash to know if the version is modified.
- - Make this very clear and in the front.

- What if each user could have their own post CSS?

--- 
# Data Modeling

## Roles
- Roles are collections of permisisons.
- Some default set of rules, (user, moderator, admin)

## Permission
- Something needed to perform an action or to see a resource.
- Immutable, can only be added to roles.

## Section
- A grouping of topics.
- Visual way of surfacing post.

## Topic
- A grouping of posts.

## Post
- In a topic.
- Can have a parent (e.g. the first post in a thread.)
- Can reply to another post.2
