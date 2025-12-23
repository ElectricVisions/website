mmd header: {{../templates/header.html}}
mmd footer: {{../templates/footer.html}}
css: /css/main.css
tags: ai vibe coding notes


# Process over Magic: Beyond Vibe Coding - Book Notes

I bought this [book](https://pragprog.com/titles/ubaidev/process-over-magic-beyond-vibe-coding/) from Pragmatic Programmers.

It's a short book (100 or so pages) with video content as well. I actually
like short books. Less fluff, more stuff, as they say. Some of the tips are
pretty standard practices. Experienced developers start with a clean git, passing
test suite, and only commits working code. However, it's even more critical
these practices are followed when working with AI.

My school computer room had a poster up on the wall that read, "To err is human,
to really foul up requires a computer". This is even truer with AI. It allows you
amplify your progress, or your mistakes exponentially.

Whilst I was already familiar with some of these practices there was still
plenty to learn and some practices I already knew about but probably aren't
following properly (e.g. rolling back when things get out of hand). So it's good
to get these tips reaffirmed and in a single resource.

## Prompting Strategies

* Guide the AI, don't convince it
* Be precise, not vague (e.g., "Always run tests and verify they pass before
  committing code" instead of "be careful")
* Avoid role-based prompting (e.g., "You're an expert developer" tries to make
  the AI an actor which it's not)
* Start with what you want the result to look like (Markdown, JSON, roles)
* Use examples or exclusions to narrow focus
* Focused prompts: one goal (add feature, fix bug, OR refactor)
* Ask the AI:
  > If instructions are unclear, ask follow-up questions before continuing
  > If a request is impossible, explain why and suggest alternatives
* Iterate: plan, test, refine, compare, reuse prompts that work

## Git & Version Control

* Start with clean git and passing tests
* One prompt, one commit
* Only commit working code
* Always review code before committing
* Stuck? Roll back. Don't waste time fighting it. Try a different approach
* When you notice something else is broken: commit (if it's working
  & passing tests), then immediately fix and commit separately
* Use git worktrees to work on multiple approaches in parallel

## Planning & Documentation

* Large features: write detailed document (≤1000 words) as plan document
* Shape plan through iterations (can take several hours)
* Rewrite in compact format after shaping
* Walking Skeleton/MVP: start with simpler version, add to plan document,
  refactor to be more efficient
* Ask AI to update design docs with progress
* Write pseudocode directly into files, ask AI to implement
* Parallel designing: Ask AI to write/rewrite feature in different ways. Explore
  variations, different libraries. Review, tweak, rollback and try again. Think:
  AI guided spikes

## Code Quality & Review

* Don't trust confident responses - always verify and test
* AI doesn't know how to write code, it just knows patterns
* AI may write bad or poorly performing code - iterate on it
* Ask AI to review for refactoring opportunities after committing
* Ensure adequate test coverage

## Debugging & Problem-Solving

* Create step-by-step debugging plan (most likely → least likely causes)
* Large codebases: move in small steps; if instructions seem unclear, wrong, or
  unfamiliar, ask AI for more information before continuing
* Ask AI to explain how without writing code, then write it yourself (learn by
  doing, guided by AI)
* Ask AI to write a script for complex transformations (more efficient than
  having the AI do it directly)

## Workflow & Sustainability

* Take regular breaks for sustainable, comfortable pace
* Stop after 5 minutes if no good results (exception: initial scaffolding)

