# BookTool

## What is BookTool

**BookTool is a simple command-line tool for managing a personal library of
books, and a Rust library for others to build off to do the same.**

## Commands

BookTool provides the following commands:

- `list`

More complete documentation can be found by running `$ booktool [subcommand]
--help`.

## Goals and Ungoals

While I do hope this project will be helpful to people, it cannot be all things
to all people. To avoid feature creep, I have clearly defined some principles
here which will guide development.

### Ungoals

Things which will not be considered to be desirable for this project, and in
fact which are undesirable.

- **Being an app, service or website.** There are two reasons for this. One is
  that I saw an need and I plan to fill it: a simple tool to track books,
  including tags and lending. It's a command line tool becuase that's how
  I usually operate, given the choice. (Want to make a GUI or TUI? please feel
  free! I'd love to see it.) The second is that online is hard: I have almost
  no experience with online security and while this is a relatively low risk
  project, it's still a major undertaking I'm not ready for.

### Goals

Items which are desirable for this project.

- **Present a simple command-line interface.** The interface should be
  consistent, and use the principle of least surprise. We strive for
  discoverability and flexibility. Commands should be consistent. For example,
  when a book identifier is needed, any unambiguous string will do: a
  (case-insensitive) title, an ISBN, or a BookID, which is an integer assigned
  to each book in your database.
- **Provide a well-documented library.** Via the magic of
  `#![deny(missing_docs)]`, all public items will be documented. This
  documentation should be useful and clear.
- **Be written in idiomatic Rust.** Patterns, naming and other conventions of
  Rust projects should be respected.

## Tasks

Listed roughly in order of desired completion.

- [ ] Implement subcommands
  - [x] list
    - [x] show all books
    - [x] align vertically
  - [ ] add
    - [ ] add by ISBN using Google books
    - [ ] add by Library of Congress number?
  - [ ] add-parts
  - [ ] tag
  - [ ] history
  - [ ] lend
  - [ ] remove
- [ ] Documentation
  - [ ] add the `#![deny(missing_docs)]` directive
  - [ ] create useful documentation
  - [ ] including examples and tests
- [ ] publish to Crates.io
