[![crates.io](https://img.shields.io/crates/v/renote.svg)](https://crates.io/crates/renote)

# What's Renote

**Renote** is a CLI to extend GitHub operation experience, which is a complementary tool to use with **gh** [GitHub’s official command line tool](https://github.com/cli/cli).

- Create a release note of issues from the latest release by [advanced search options](https://docs.github.com/en/github/searching-for-information-on-github/searching-issues-and-pull-requests)
- Add or remove labels of issues by advanced search options
- Add or remove issues to/from a milestone by advanced search options
- Search issues by advance search options

# Getting Started

## Prerequisites

The GitHub credential is necessary, please refer to [how to create a personal access token](https://github.com/settings/tokens).

## Installation

Thre are different ways to install renote.

- `huber install renote` ([huber](https://github.com/innobead/huber)) 
- `cargo install renote`
- Download the released binary directly

```console
❯ huber install renote
Updating managed repos
Updating unmanaged repos
Installing renote
Downloading package artifacts from github "https://github.com/ecatlabs/renote"
Setting renote (version: v0.1.0, source: github) as the current package
Installed executables:
 - /home/davidko/.huber/bin/renote
renote (version: v0.1.0, source: github) installed

❯ renote -V
renote v0.1.0 Commit: 261b21e-20210419081458
```

## Tutorials

### Create a release note config

Need to create a release note config, then update and save to a file which will be used for the note creation.

```console
❯ renote note config
---
owner: ""
repo: ""
token: ""
state: ""
note: ~
milestone: ~
show_contributor: false
extra_contributors: ~
exclude_issues: ~
sort: ~
labels: ~
any_labels: ~
exclude_labels: ~
highlight_labels:
  - label: ""
    title: ~
    description: ~
```

Example

```yaml
---
owner: longhorn
repo: longhorn
token: ""
state: closed        # open, closed, all
note: >-             # string or file path
  ## Release Note
  {content}

milestone: v1.1.1
show_contributor: true
extra_contributors:
  - innobead
exclude_issues:
  - 304
sort: asc            # asc, desc
labels: []
any_labels: []
exclude_labels:
  - wontfix
highlight_labels:
  - label: highlight
    title: Highlights
    description: ~

```

### Create a release note

Based on the note config file, you can search issues matching the release scope like milestone, issue filters, etc. Also, the note can be customized by the Markdown content or file where `{content}`
inside will be replaced by the generated note.

```console
❯ renote note create --config ./examples/note_config.yaml
...
```

The `note` configuration of the note config file to extend the generated note.

```
note: >- # or file path
  ## Release Note
  {content}
```

### Search issues

Search issues by the advanced query, and the output can be in different formats (console, JSON, YAML).

```console
❯ renote issue search -o longhorn -r longhorn -q "label:kind/upstream-issue"
 Url                                                         Title 
 https://api.github.com/repos/longhorn/longhorn/issues/2106  [BUG] Recurring backup job stuck on K8s 1.19.4 if volume is attached to the same node and powered do... 
 https://api.github.com/repos/longhorn/longhorn/issues/2062  [BUG] The feature Pod Deletion Policy When Node is Down doesn't work on Kubernetes >= v1.19.0 
```

### Add labels to issues

```console
❯ renote issue add-label -o longhorn -r longhorn -q "label:kind/upstream-issue" --labels "kind/extra-label"
...
```

### Remove labels from issues

```console
❯ renote issue remove-label -o longhorn -r longhorn -q "label:kind/upstream-issue" --labels "kind/extra-label"
...
```

### Move issues to a milestone

```console
❯ renote issue assign-milestone -o longhorn -r longhorn -q "label:kind/upstream-issue" next-milestone
...
```
