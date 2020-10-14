# 11. gitignore design

Date: 2020-10-14

## Status

2020-10-14 proposed

## Context

We need ignore for git rules for our project, to make process fast.


As a user, they will had such as content:

```
├── .gitmodules
├── .idea
└── app
    ├── .git
    ├── .gitmodules
    ├── .travis.yml
    └── community
```

and also they may forgot build/target such as dir in project.

So we need to:

1. accept workspace Root `.Gitignore` file in project.
2. accept workspace Module `.Gitignore` file in project.
3. accept Different languages gitignore files
4. add some default editor's rules such as `.idea` and also start with `.*`

## Decision

add GitHub [gitignore](https://github.com/github/gitignore) project as gitignore file source.

## Consequences

Consequences here...
